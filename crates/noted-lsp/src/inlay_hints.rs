use tower_lsp::lsp_types::*;

/// Compute inlay hints for the given Markdown text.
///
/// Emits a virtual label after each checkbox marker:
/// - `- [x]` → ` ✓`
/// - `- [ ]` → ` ○`
pub fn compute_inlay_hints(text: &str) -> Vec<InlayHint> {
    let mut hints = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let ln = line_idx as u32;

        if let Some(col) = checkbox_col(line, true) {
            hints.push(hint(ln, col, " ✓"));
        } else if let Some(col) = checkbox_col(line, false) {
            hints.push(hint(ln, col, " ○"));
        }
    }

    hints
}

/// Returns the character column just after `]` in a checkbox marker,
/// or `None` if the pattern is not found.
///
/// `done = true`  → looks for `- [x]`
/// `done = false` → looks for `- [ ]`
fn checkbox_col(line: &str, done: bool) -> Option<u32> {
    let pattern = if done { "- [x]" } else { "- [ ]" };
    let byte_pos = line.find(pattern)?;
    Some((byte_pos + pattern.len()) as u32)
}

fn hint(line: u32, character: u32, label: &str) -> InlayHint {
    InlayHint {
        position: Position { line, character },
        label: InlayHintLabel::String(label.to_string()),
        kind: None,
        text_edits: None,
        tooltip: None,
        padding_left: None,
        padding_right: None,
        data: None,
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn label_str(hint: &InlayHint) -> &str {
        match &hint.label {
            InlayHintLabel::String(s) => s.as_str(),
            _ => panic!("expected String label"),
        }
    }

    #[test]
    fn test_done_checkbox() {
        let hints = compute_inlay_hints("- [x] Buy milk\n");
        assert_eq!(hints.len(), 1);
        assert_eq!(label_str(&hints[0]), " ✓");
        assert_eq!(
            hints[0].position,
            Position {
                line: 0,
                character: 5
            }
        );
    }

    #[test]
    fn test_open_checkbox() {
        let hints = compute_inlay_hints("- [ ] Write tests\n");
        assert_eq!(hints.len(), 1);
        assert_eq!(label_str(&hints[0]), " ○");
        assert_eq!(
            hints[0].position,
            Position {
                line: 0,
                character: 5
            }
        );
    }

    #[test]
    fn test_multiple_checkboxes() {
        let text = "- [x] Done\n- [ ] Todo\n- [x] Also done\n";
        let hints = compute_inlay_hints(text);
        assert_eq!(hints.len(), 3);
        assert_eq!(hints[0].position.line, 0);
        assert_eq!(hints[1].position.line, 1);
        assert_eq!(hints[2].position.line, 2);
    }

    #[test]
    fn test_no_hints_for_plain_text() {
        let hints = compute_inlay_hints("# Heading\n\nJust a paragraph.\n");
        assert!(hints.is_empty());
    }

    #[test]
    fn test_indented_checkbox() {
        let hints = compute_inlay_hints("  - [x] Indented\n");
        assert_eq!(hints.len(), 1);
        assert_eq!(hints[0].position.character, 7); // "  - [x]" = 7 chars
    }
}
