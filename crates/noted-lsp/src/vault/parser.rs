use camino::Utf8PathBuf;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag as MdTag, TagEnd};
use regex::Regex;

use super::index::{Frontmatter, Heading, LinkReference, NoteEntry, Tag};

/// Parse a Markdown file into a `NoteEntry`.
pub fn parse_note(path: &Utf8PathBuf, content: &str) -> NoteEntry {
    let (frontmatter, body_offset) = extract_frontmatter(content);
    let body = &content[body_offset..];
    let body_line_offset = count_newlines(&content[..body_offset]);

    let headings = extract_headings(body, body_line_offset);
    let links = extract_wikilinks(content);
    let tags = extract_tags(content);

    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| {
            headings
                .iter()
                .find(|h| h.level == 1)
                .map(|h| h.text.clone())
        })
        .unwrap_or_else(|| path.file_stem().unwrap_or(path.as_str()).to_string());

    NoteEntry {
        path: path.clone(),
        title,
        headings,
        links,
        tags,
        frontmatter,
    }
}

// ── Frontmatter ────────────────────────────────────────────────────────────

/// Returns parsed frontmatter and the byte offset where the body begins.
fn extract_frontmatter(content: &str) -> (Option<Frontmatter>, usize) {
    if !content.starts_with("---") {
        return (None, 0);
    }
    // Find the closing ---
    let rest = &content[3..];
    let end = rest.find("\n---").or_else(|| rest.find("\r\n---"));
    let Some(end_pos) = end else {
        return (None, 0);
    };

    let raw = rest[..end_pos].trim().to_string();
    // Body starts after the closing --- line
    let closing_start = 3 + end_pos;
    let closing_line_end = content[closing_start..]
        .find('\n')
        .map(|i| closing_start + i + 1)
        .unwrap_or(content.len());

    let fm = parse_frontmatter_fields(&raw);
    (Some(fm), closing_line_end)
}

fn parse_frontmatter_fields(raw: &str) -> Frontmatter {
    let mut title = None;
    let mut tags = Vec::new();

    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("title:") {
            title = Some(rest.trim().trim_matches('"').to_string());
        } else if let Some(rest) = line.strip_prefix("tags:") {
            // Inline list: tags: [foo, bar]  or  tags: foo
            let cleaned = rest.trim().trim_matches(['[', ']'].as_ref());
            for tag in cleaned.split(',') {
                let t = tag.trim().trim_matches('"');
                if !t.is_empty() {
                    tags.push(t.to_string());
                }
            }
        } else if let Some(rest) = line
            .strip_prefix("  - ")
            .or_else(|| line.strip_prefix("- "))
        {
            // YAML list item under tags:
            let t = rest.trim().trim_matches('"');
            if !t.is_empty() {
                tags.push(t.to_string());
            }
        }
    }

    Frontmatter {
        title,
        tags,
        raw: raw.to_string(),
    }
}

// ── Headings ───────────────────────────────────────────────────────────────

fn extract_headings(body: &str, line_offset: u32) -> Vec<Heading> {
    let line_starts = build_line_starts(body);
    let parser = Parser::new_ext(body, Options::empty()).into_offset_iter();

    let mut headings = Vec::new();
    let mut current: Option<(u8, u32, String)> = None;

    for (event, range) in parser {
        match event {
            Event::Start(MdTag::Heading { level, .. }) => {
                let line = offset_to_line(&line_starts, range.start) + line_offset;
                let lvl = heading_level_to_u8(level);
                current = Some((lvl, line, String::new()));
            }
            Event::Text(text) => {
                if let Some((_, _, ref mut buf)) = current {
                    buf.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, line, text)) = current.take() {
                    headings.push(Heading { level, text, line });
                }
            }
            _ => {}
        }
    }

    headings
}

fn heading_level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

// ── Wikilinks ──────────────────────────────────────────────────────────────

fn extract_wikilinks(content: &str) -> Vec<LinkReference> {
    // [[target]]  [[target#anchor]]  [[target|alias]]  [[target#anchor|alias]]
    let re = Regex::new(r"\[\[([^\]|#\n]+?)(?:#([^\]|\n]+?))?(?:\|([^\]\n]+?))?\]\]").unwrap();
    let line_starts = build_line_starts(content);

    re.captures_iter(content)
        .map(|cap| {
            let full = cap.get(0).unwrap();
            let line = offset_to_line(&line_starts, full.start());
            let col = full.start() - line_starts[line as usize];
            LinkReference {
                target: cap[1].trim().to_string(),
                anchor: cap.get(2).map(|m| m.as_str().trim().to_string()),
                alias: cap.get(3).map(|m| m.as_str().trim().to_string()),
                line,
                col: col as u32,
            }
        })
        .collect()
}

// ── Tags ───────────────────────────────────────────────────────────────────

fn extract_tags(content: &str) -> Vec<Tag> {
    // Match #tag not at the start of a line (which would be a heading marker)
    let re = Regex::new(r"(?m)(?:^|\s)#([A-Za-z][\w\-/]*)").unwrap();
    let line_starts = build_line_starts(content);

    re.captures_iter(content)
        .map(|cap| {
            let m = cap.get(1).unwrap();
            let line = offset_to_line(&line_starts, m.start());
            Tag {
                name: cap[1].to_string(),
                line,
            }
        })
        .collect()
}

// ── Utilities ──────────────────────────────────────────────────────────────

/// Returns a vec where `line_starts[i]` is the byte offset of the start of line `i`.
fn build_line_starts(content: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (i, b) in content.bytes().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

fn offset_to_line(line_starts: &[usize], offset: usize) -> u32 {
    line_starts
        .partition_point(|&start| start <= offset)
        .saturating_sub(1) as u32
}

fn count_newlines(s: &str) -> u32 {
    s.bytes().filter(|&b| b == b'\n').count() as u32
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn path(s: &str) -> Utf8PathBuf {
        Utf8PathBuf::from(s)
    }

    #[test]
    fn test_parse_headings() {
        let content = "# Title\n\nSome text.\n\n## Section\n\nMore text.\n";
        let entry = parse_note(&path("note.md"), content);
        assert_eq!(entry.headings.len(), 2);
        assert_eq!(entry.headings[0].level, 1);
        assert_eq!(entry.headings[0].text, "Title");
        assert_eq!(entry.headings[0].line, 0);
        assert_eq!(entry.headings[1].level, 2);
        assert_eq!(entry.headings[1].text, "Section");
    }

    #[test]
    fn test_title_from_h1() {
        let content = "# My Note\n\nContent.\n";
        let entry = parse_note(&path("my-note.md"), content);
        assert_eq!(entry.title, "My Note");
    }

    #[test]
    fn test_title_fallback_to_filename() {
        let content = "Just some content without headings.\n";
        let entry = parse_note(&path("my-note.md"), content);
        assert_eq!(entry.title, "my-note");
    }

    #[test]
    fn test_parse_wikilinks() {
        let content = "See [[Alice]] and [[Bob|Robert]] and [[Notes/Project#intro]].\n";
        let entry = parse_note(&path("note.md"), content);
        assert_eq!(entry.links.len(), 3);
        assert_eq!(entry.links[0].target, "Alice");
        assert_eq!(entry.links[0].alias, None);
        assert_eq!(entry.links[1].target, "Bob");
        assert_eq!(entry.links[1].alias.as_deref(), Some("Robert"));
        assert_eq!(entry.links[2].target, "Notes/Project");
        assert_eq!(entry.links[2].anchor.as_deref(), Some("intro"));
    }

    #[test]
    fn test_parse_tags() {
        let content = "Tagged with #rust and #async-programming.\n";
        let entry = parse_note(&path("note.md"), content);
        assert_eq!(entry.tags.len(), 2);
        assert_eq!(entry.tags[0].name, "rust");
        assert_eq!(entry.tags[1].name, "async-programming");
    }

    #[test]
    fn test_parse_frontmatter() {
        let content = "---\ntitle: My Note\ntags: [rust, async]\n---\n\n# Body\n";
        let entry = parse_note(&path("note.md"), content);
        let fm = entry.frontmatter.unwrap();
        assert_eq!(fm.title.as_deref(), Some("My Note"));
        assert_eq!(fm.tags, vec!["rust", "async"]);
        // Title comes from frontmatter
        assert_eq!(entry.title, "My Note");
    }

    #[test]
    fn test_frontmatter_title_takes_priority_over_h1() {
        let content = "---\ntitle: Frontmatter Title\n---\n\n# H1 Title\n";
        let entry = parse_note(&path("note.md"), content);
        assert_eq!(entry.title, "Frontmatter Title");
    }

    #[test]
    fn test_headings_after_frontmatter_have_correct_lines() {
        let content = "---\ntitle: Test\n---\n\n# Heading One\n\n## Heading Two\n";
        let entry = parse_note(&path("note.md"), content);
        assert_eq!(entry.headings.len(), 2);
        // Frontmatter is 3 lines + blank line = line 4 (0-indexed)
        assert_eq!(entry.headings[0].line, 4);
        assert_eq!(entry.headings[1].line, 6);
    }
}
