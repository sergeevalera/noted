use tower_lsp::lsp_types::*;

use crate::vault::VaultIndex;

/// Returns wikilink completion items if the cursor is inside `[[...`.
///
/// `line`      — 0-based line number (used to build the TextEdit range)
/// `line_text` — full text of the current line
/// `character` — cursor column (byte offset; assumes ASCII-safe content for MVP)
pub fn compute_completions(
    line: u32,
    line_text: &str,
    character: u32,
    index: &VaultIndex,
) -> Vec<CompletionItem> {
    let cursor = (character as usize).min(line_text.len());
    let prefix = &line_text[..cursor];

    // Find the last `[[` before the cursor
    let Some(bracket_pos) = prefix.rfind("[[") else {
        return vec![];
    };
    let after_brackets = &prefix[bracket_pos + 2..];

    // If the wikilink starting at bracket_pos is already closed (]] anywhere after [[),
    // the cursor is inside a completed link — don't offer completions
    if line_text[bracket_pos..].contains("]]") {
        return vec![];
    }

    let partial = after_brackets.to_lowercase();
    // Character column right after the `[[` (byte == char for ASCII content)
    let start_char = (bracket_pos + 2) as u32;

    index
        .notes
        .values()
        .filter(|note| {
            partial.is_empty()
                || note.title.to_lowercase().starts_with(&partial)
                || note
                    .path
                    .file_stem()
                    .map(|s| s.to_lowercase().starts_with(&partial))
                    .unwrap_or(false)
        })
        .map(|note| {
            // Use the file stem as insert text (matches how wikilinks are written)
            let stem = note
                .path
                .file_stem()
                .unwrap_or(note.title.as_str())
                .to_string();
            CompletionItem {
                label: note.title.clone(),
                kind: Some(CompletionItemKind::REFERENCE),
                detail: Some(note.path.to_string()),
                // Replace the partial text after `[[` with the note stem
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: Range {
                        start: Position {
                            line,
                            character: start_char,
                        },
                        end: Position { line, character },
                    },
                    new_text: stem,
                })),
                filter_text: Some(note.title.clone()),
                ..Default::default()
            }
        })
        .collect()
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

    #[test]
    fn test_no_completion_outside_wikilink() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let items = compute_completions(0, "just some text", 14, &index);
        assert!(items.is_empty());
    }

    #[test]
    fn test_completion_after_double_bracket() {
        let index = make_index(&[
            ("/vault/alice.md", "# Alice\n"),
            ("/vault/bob.md", "# Bob\n"),
        ]);
        // Cursor right after `[[`
        let items = compute_completions(0, "See [[", 6, &index);
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_completion_filters_by_partial() {
        let index = make_index(&[
            ("/vault/alice.md", "# Alice\n"),
            ("/vault/albert.md", "# Albert\n"),
            ("/vault/bob.md", "# Bob\n"),
        ]);
        let items = compute_completions(0, "See [[al", 8, &index);
        assert_eq!(items.len(), 2);
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"Alice"));
        assert!(labels.contains(&"Albert"));
    }

    #[test]
    fn test_completion_text_edit_range() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let items = compute_completions(3, "See [[ali", 9, &index);
        assert_eq!(items.len(), 1);
        if let Some(CompletionTextEdit::Edit(edit)) = &items[0].text_edit {
            assert_eq!(
                edit.range.start,
                Position {
                    line: 3,
                    character: 6
                }
            ); // after `[[`
            assert_eq!(
                edit.range.end,
                Position {
                    line: 3,
                    character: 9
                }
            ); // cursor
            assert_eq!(edit.new_text, "alice");
        } else {
            panic!("expected TextEdit");
        }
    }

    #[test]
    fn test_no_completion_after_closed_wikilink() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        // Cursor is after a completed wikilink
        let items = compute_completions(0, "[[alice]] and [[", 16, &index);
        // Should trigger for the second `[[`
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_no_completion_inside_closed_wikilink() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        // `[[alice]]` — cursor between the brackets but `]]` is present after `[[`
        let items = compute_completions(0, "[[alice]]", 5, &index);
        assert!(items.is_empty());
    }
}
