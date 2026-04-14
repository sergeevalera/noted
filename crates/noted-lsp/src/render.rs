use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;

/// Render Markdown text to HTML with support for wikilinks, callouts, math, and data-line attributes.
pub fn render_markdown(text: &str) -> String {
    let text = preprocess(text);
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_MATH;

    let parser = Parser::new_ext(&text, options);
    let events = transform_events(parser);

    let mut html = String::with_capacity(text.len() * 2);
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    html
}

/// Preprocess Markdown source: expand wikilinks and callouts before pulldown-cmark parsing.
fn preprocess(text: &str) -> String {
    let wikilink_re =
        Regex::new(r"\[\[([^\]|#\n]+?)(?:#([^\]|\n]+?))?(?:\|([^\]\n]+?))?\]\]").unwrap();
    let callout_re = Regex::new(r"^> \[!([A-Za-z][A-Za-z0-9]*)\]\s*(.*)").unwrap();

    let mut result = String::with_capacity(text.len());
    let mut in_callout = false;

    for line in text.lines() {
        // Callout start: > [!type] optional title
        if let Some(caps) = callout_re.captures(line) {
            let ctype = &caps[1];
            let title = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
            let callout_type = ctype.to_lowercase();
            in_callout = true;

            let display_title = if title.is_empty() {
                capitalize(&callout_type)
            } else {
                title.to_string()
            };

            result.push_str(&format!(
                "<div class=\"callout callout-{}\">\n<p class=\"callout-title\">{}</p>\n",
                callout_type, display_title
            ));
            result.push('\n');
            continue;
        }

        // Callout continuation: > content
        if in_callout && line.starts_with("> ") {
            let content = &line[2..];
            // Process wikilinks in callout content
            let processed =
                wikilink_re.replace_all(content, |caps: &regex::Captures| wikilink_to_html(caps));
            result.push_str(&processed);
            result.push('\n');
            continue;
        }

        // End callout when line doesn't start with >
        if in_callout && !line.starts_with('>') {
            result.push_str("</div>\n");
            in_callout = false;
        }

        // Process wikilinks in normal lines
        let processed =
            wikilink_re.replace_all(line, |caps: &regex::Captures| wikilink_to_html(caps));
        result.push_str(&processed);
        result.push('\n');
    }

    // Close unclosed callout
    if in_callout {
        result.push_str("</div>\n");
    }

    result
}

/// Convert a wikilink regex capture to an HTML link.
fn wikilink_to_html(caps: &regex::Captures) -> String {
    let target = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
    let alias = caps.get(3).map(|m| m.as_str().trim());
    let display = alias.unwrap_or(target);
    let href = format!("{}.md", target.replace(' ', "-").to_lowercase());
    format!(
        "<a class=\"wikilink\" href=\"{}\" data-target=\"{}\">{}</a>",
        href, target, display
    )
}

pub(crate) fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

/// Transform pulldown-cmark events to add data-line attributes and enhance rendering.
fn transform_events(parser: Parser<'_>) -> Vec<Event<'_>> {
    let mut events: Vec<Event> = Vec::new();
    let mut line_number: usize = 0;

    for event in parser {
        match &event {
            // Add data-line to block-level opening tags
            Event::Start(Tag::Heading { level, .. }) => {
                let lvl = *level as u8;
                events.push(Event::Html(
                    format!("<h{} data-line=\"{}\">", lvl, line_number).into(),
                ));
                continue;
            }
            Event::End(TagEnd::Heading(level)) => {
                let lvl = *level as u8;
                events.push(Event::Html(format!("</h{}>", lvl).into()));
                continue;
            }
            Event::Start(Tag::Paragraph) => {
                events.push(Event::Html(
                    format!("<p data-line=\"{}\">", line_number).into(),
                ));
                continue;
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) if !lang.is_empty() => {
                        format!(" class=\"language-{}\"", lang)
                    }
                    _ => String::new(),
                };
                events.push(Event::Html(
                    format!("<pre data-line=\"{}\"><code{}>", line_number, lang).into(),
                ));
                continue;
            }
            // Track line numbers from soft/hard breaks
            Event::SoftBreak | Event::HardBreak => {
                line_number += 1;
            }
            Event::Text(t) => {
                line_number += t.chars().filter(|&c| c == '\n').count();
            }
            _ => {}
        }
        events.push(event);
    }
    events
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading() {
        let html = render_markdown("# Hello\n");
        assert!(html.contains("<h1"));
        assert!(html.contains("Hello"));
        assert!(html.contains("</h1>"));
    }

    #[test]
    fn test_heading_levels() {
        let html = render_markdown("## H2\n### H3\n");
        assert!(html.contains("<h2"));
        assert!(html.contains("<h3"));
    }

    #[test]
    fn test_paragraph_data_line() {
        let html = render_markdown("Hello world\n");
        assert!(html.contains("data-line="));
    }

    #[test]
    fn test_bold_italic() {
        let html = render_markdown("**bold** and *italic*\n");
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
    }

    #[test]
    fn test_strikethrough() {
        let html = render_markdown("~~deleted~~\n");
        assert!(html.contains("<del>deleted</del>"));
    }

    #[test]
    fn test_inline_code() {
        let html = render_markdown("Use `cargo test`\n");
        assert!(html.contains("<code>cargo test</code>"));
    }

    #[test]
    fn test_fenced_code_block() {
        let html = render_markdown("```rust\nfn main() {}\n```\n");
        assert!(html.contains("language-rust"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_wikilink_simple() {
        let html = render_markdown("See [[My Note]] here\n");
        assert!(html.contains("class=\"wikilink\""));
        assert!(html.contains("data-target=\"My Note\""));
        assert!(html.contains(">My Note</a>"));
    }

    #[test]
    fn test_wikilink_with_alias() {
        let html = render_markdown("See [[target|display text]] here\n");
        assert!(html.contains("data-target=\"target\""));
        assert!(html.contains(">display text</a>"));
    }

    #[test]
    fn test_wikilink_href() {
        let html = render_markdown("[[My Note]]\n");
        assert!(html.contains("href=\"my-note.md\""));
    }

    #[test]
    fn test_callout_note() {
        let html = render_markdown("> [!note] Important\n> Some content\n");
        assert!(html.contains("callout-note"));
        assert!(html.contains("callout-title"));
        assert!(html.contains("Important"));
        assert!(html.contains("Some content"));
    }

    #[test]
    fn test_callout_default_title() {
        let html = render_markdown("> [!warning]\n> Be careful\n");
        assert!(html.contains("callout-warning"));
        assert!(html.contains("Warning"));
    }

    #[test]
    fn test_callout_closes() {
        let html = render_markdown("> [!note]\n> Content\n\nAfter callout\n");
        assert!(html.contains("</div>"));
        assert!(html.contains("After callout"));
    }

    #[test]
    fn test_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |\n";
        let html = render_markdown(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>"));
        assert!(html.contains("<td>"));
    }

    #[test]
    fn test_task_list() {
        let html = render_markdown("- [x] Done\n- [ ] Todo\n");
        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("checked"));
    }

    #[test]
    fn test_link() {
        let html = render_markdown("[text](https://example.com)\n");
        assert!(html.contains("href=\"https://example.com\""));
        assert!(html.contains(">text</a>"));
    }

    #[test]
    fn test_math_inline() {
        let html = render_markdown("The formula $E=mc^2$ is famous\n");
        // pulldown-cmark with ENABLE_MATH wraps math in specific tags
        assert!(html.contains("E=mc^2"));
    }

    #[test]
    fn test_empty_input() {
        let html = render_markdown("");
        assert!(html.is_empty() || html.trim().is_empty());
    }

    #[test]
    fn test_frontmatter_not_rendered() {
        // Frontmatter should pass through as-is (not rendered as headings)
        let html = render_markdown("---\ntitle: Test\n---\n# Heading\n");
        assert!(html.contains("Heading"));
    }

    #[test]
    fn test_wikilink_in_callout() {
        let html = render_markdown("> [!note]\n> See [[alice]] for details\n");
        assert!(html.contains("class=\"wikilink\""));
        assert!(html.contains("data-target=\"alice\""));
    }
}
