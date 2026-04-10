use tower_lsp::lsp_types::*;

use crate::vault::{resolve_wikilink, VaultIndex};

/// Build hover content for the wikilink under the cursor, if any.
///
/// Shows: note title, first paragraph of body, tags, backlink count.
/// Returns `None` if the cursor is not inside a `[[...]]` or the target is unresolved.
pub fn compute_hover(line_text: &str, character: u32, index: &VaultIndex) -> Option<Hover> {
    let (target, _span) = find_wikilink_at(line_text, character)?;
    let path = resolve_wikilink(index, &target)?;
    let note = index.notes.get(&path)?;

    let mut parts: Vec<String> = vec![format!("## {}", note.title)];

    // Body snippet — read from disk; silently omitted if unavailable
    if let Ok(content) = std::fs::read_to_string(path.as_std_path()) {
        let snippet = body_snippet(&content);
        if !snippet.is_empty() {
            parts.push(snippet);
        }
    }

    // Metadata footer
    let mut footer: Vec<String> = Vec::new();
    if !note.tags.is_empty() {
        let tags = note
            .tags
            .iter()
            .map(|t| format!("#{}", t.name))
            .collect::<Vec<_>>()
            .join(" ");
        footer.push(format!("**Tags:** {}", tags));
    }
    let backlink_count = index.backlinks.get(&path).map(|v| v.len()).unwrap_or(0);
    if backlink_count > 0 {
        footer.push(format!("**Backlinks:** {}", backlink_count));
    }
    if !footer.is_empty() {
        parts.push(footer.join("  \n"));
    }

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: parts.join("\n\n"),
        }),
        range: None,
    })
}

/// Find the wikilink target at `character` in `line_text`.
/// Returns `(target_string, byte_span)` or `None` if not in a wikilink.
fn find_wikilink_at(line_text: &str, character: u32) -> Option<(String, std::ops::Range<usize>)> {
    let cursor = (character as usize).min(line_text.len());
    let mut search_from = 0;

    while let Some(rel_open) = line_text[search_from..].find("[[") {
        let open_pos = search_from + rel_open;
        let after_open = &line_text[open_pos..];
        let Some(rel_close) = after_open.find("]]") else {
            break;
        };
        let close_pos = open_pos + rel_close;

        if cursor >= open_pos && cursor <= close_pos + 1 {
            let inner = &line_text[open_pos + 2..close_pos];
            let target = inner.split(['#', '|']).next()?.trim().to_string();
            if target.is_empty() {
                return None;
            }
            return Some((target, open_pos..close_pos + 2));
        }

        search_from = close_pos + 2;
    }

    None
}

/// Extract the first meaningful paragraph from note content.
/// Skips YAML frontmatter, leading blank lines, and heading lines.
/// Stops at the first blank line after text begins, or after 300 chars.
fn body_snippet(content: &str) -> String {
    let body = skip_frontmatter(content);
    let mut lines: Vec<&str> = Vec::new();
    let mut char_count: usize = 0;

    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() {
            if !lines.is_empty() {
                break; // End of first paragraph
            }
            continue; // Skip leading blank lines
        }
        if t.starts_with('#') {
            if !lines.is_empty() {
                break; // Heading after paragraph ends the snippet
            }
            continue; // Skip headings before the paragraph
        }
        lines.push(t);
        char_count += t.len();
        if char_count >= 300 {
            break;
        }
    }

    lines.join("\n")
}

fn skip_frontmatter(content: &str) -> &str {
    if !content.starts_with("---") {
        return content;
    }
    let rest = &content[3..];
    // end_pos is the position of the '\n' that precedes the closing '---'
    let end_pos = match rest.find("\n---") {
        Some(p) => p,
        None => return content,
    };
    // Skip past '\n---' (4 chars) to reach whatever follows the closing marker
    let after_marker = &rest[end_pos + 4..];
    // Skip the rest of the closing '---' line (e.g. trailing spaces) and its newline
    match after_marker.find('\n') {
        Some(nl) => &after_marker[nl + 1..],
        None => "", // closing '---' was the last line
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{build_index, parse_note};
    use camino::Utf8PathBuf;

    fn make_index(notes: &[(&str, &str)]) -> VaultIndex {
        let entries = notes
            .iter()
            .map(|(path, content)| parse_note(&Utf8PathBuf::from(path), content))
            .collect();
        build_index(entries)
    }

    // ── find_wikilink_at ──

    #[test]
    fn test_find_wikilink_inside() {
        let r = find_wikilink_at("See [[alice]] here", 8);
        assert_eq!(r.map(|(t, _)| t).as_deref(), Some("alice"));
    }

    #[test]
    fn test_find_wikilink_cursor_on_open_bracket() {
        let r = find_wikilink_at("[[alice]]", 0);
        assert_eq!(r.map(|(t, _)| t).as_deref(), Some("alice"));
    }

    #[test]
    fn test_find_wikilink_cursor_on_close_bracket() {
        let r = find_wikilink_at("[[alice]]", 8); // second `]`
        assert_eq!(r.map(|(t, _)| t).as_deref(), Some("alice"));
    }

    #[test]
    fn test_find_wikilink_outside() {
        assert!(find_wikilink_at("[[alice]] text", 12).is_none());
        assert!(find_wikilink_at("text [[alice]]", 2).is_none());
    }

    #[test]
    fn test_find_wikilink_strips_anchor() {
        let r = find_wikilink_at("[[alice#section]]", 4);
        assert_eq!(r.map(|(t, _)| t).as_deref(), Some("alice"));
    }

    #[test]
    fn test_find_wikilink_strips_alias() {
        let r = find_wikilink_at("[[alice|Alice W.]]", 4);
        assert_eq!(r.map(|(t, _)| t).as_deref(), Some("alice"));
    }

    // ── body_snippet ──

    #[test]
    fn test_snippet_simple_paragraph() {
        let content = "# Heading\n\nFirst paragraph text.\nContinued here.\n\nSecond paragraph.\n";
        assert_eq!(
            body_snippet(content),
            "First paragraph text.\nContinued here."
        );
    }

    #[test]
    fn test_snippet_skips_frontmatter() {
        let content = "---\ntitle: Test\n---\n\n# Heading\n\nBody text here.\n";
        assert_eq!(body_snippet(content), "Body text here.");
    }

    #[test]
    fn test_snippet_skips_leading_heading() {
        let content = "# Title\n\nParagraph.\n";
        assert_eq!(body_snippet(content), "Paragraph.");
    }

    #[test]
    fn test_snippet_empty_note() {
        assert_eq!(body_snippet("# Just a heading\n"), "");
        assert_eq!(body_snippet("---\ntitle: T\n---\n"), "");
    }

    // ── compute_hover ──

    #[test]
    fn test_hover_resolves_title() {
        // File doesn't exist on disk, so no snippet — but title comes from the index
        let index = make_index(&[("/vault/alice.md", "# Alice\n\nHello world.\n")]);
        let hover = compute_hover("See [[alice]] here", 8, &index);
        assert!(hover.is_some());
        let HoverContents::Markup(mc) = hover.unwrap().contents else {
            panic!()
        };
        assert!(mc.value.contains("## Alice"));
    }

    #[test]
    fn test_hover_broken_link_returns_none() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        assert!(compute_hover("See [[missing]] here", 8, &index).is_none());
    }

    #[test]
    fn test_hover_outside_wikilink_returns_none() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        assert!(compute_hover("plain text", 3, &index).is_none());
    }

    #[test]
    fn test_hover_shows_tags() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n\nTagged with #person.\n")]);
        let hover = compute_hover("[[alice]]", 3, &index).unwrap();
        let HoverContents::Markup(mc) = hover.contents else {
            panic!()
        };
        assert!(mc.value.contains("#person"));
    }
}
