use std::collections::HashMap;

use tower_lsp::lsp_types::*;

use crate::vault::VaultIndex;

/// Check if the cursor is on a renameable wikilink target.
/// Returns the range of the target text inside `[[...]]` (excluding brackets, anchor, alias).
pub fn prepare_rename(line_text: &str, line: u32, character: u32) -> Option<PrepareRenameResponse> {
    let (target, start_col, end_col) = find_wikilink_target_at(line_text, character)?;
    if target.is_empty() {
        return None;
    }
    Some(PrepareRenameResponse::RangeWithPlaceholder {
        range: Range {
            start: Position { line, character: start_col },
            end: Position { line, character: end_col },
        },
        placeholder: target.to_string(),
    })
}

/// Compute a workspace edit that renames a wikilink target across all vault files.
///
/// When the cursor is on `[[old-name]]` and the user renames to `new-name`:
/// - Every `[[old-name]]`, `[[old-name#anchor]]`, `[[old-name|alias]]` across the vault
///   gets its target portion replaced with `new-name`.
pub fn compute_rename(
    line_text: &str,
    character: u32,
    new_name: &str,
    index: &VaultIndex,
) -> Option<WorkspaceEdit> {
    let (target, _, _) = find_wikilink_target_at(line_text, character)?;
    if target.is_empty() {
        return None;
    }

    let target_lower = target.to_lowercase();
    let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();

    // Scan every note in the vault for wikilinks matching this target.
    // LinkReference stores the col of `[[`, so the target text starts at col+2.
    for entry in index.notes.values() {
        for link in &entry.links {
            if link.target.to_lowercase() != target_lower {
                continue;
            }
            let uri = match Url::from_file_path(entry.path.as_std_path()) {
                Ok(u) => u,
                Err(_) => continue,
            };
            // Target starts right after `[[`
            let start_col = link.col + 2;
            let end_col = start_col + link.target.len() as u32;
            changes.entry(uri).or_default().push(TextEdit {
                range: Range {
                    start: Position { line: link.line, character: start_col },
                    end: Position { line: link.line, character: end_col },
                },
                new_text: new_name.to_string(),
            });
        }
    }

    if changes.is_empty() {
        return None;
    }

    Some(WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    })
}

/// Find the wikilink target text at the given cursor position.
/// Returns (target_str, start_col, end_col) for the target portion only.
fn find_wikilink_target_at(line_text: &str, character: u32) -> Option<(&str, u32, u32)> {
    let cursor = (character as usize).min(line_text.len());
    let mut search_from = 0;

    while let Some(rel_open) = line_text[search_from..].find("[[") {
        let open_pos = search_from + rel_open;
        let after_open = &line_text[open_pos..];
        let Some(rel_close) = after_open.find("]]") else { break };
        let close_pos = open_pos + rel_close;

        if cursor >= open_pos && cursor <= close_pos + 1 {
            let inner = &line_text[open_pos + 2..close_pos];
            // Target is the part before # or |
            let target_end = inner.find(['#', '|']).unwrap_or(inner.len());
            let target = inner[..target_end].trim();
            if target.is_empty() {
                return None;
            }
            // Find the actual start/end columns of the target within the line
            let target_start_in_inner = inner.find(target)?;
            let abs_start = (open_pos + 2 + target_start_in_inner) as u32;
            let abs_end = abs_start + target.len() as u32;
            return Some((target, abs_start, abs_end));
        }

        search_from = close_pos + 2;
    }

    None
}


// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;
    use crate::vault::{build_index, parse_note};

    fn make_index(notes: &[(&str, &str)]) -> VaultIndex {
        let entries: Vec<_> = notes
            .iter()
            .map(|(path, content)| parse_note(&Utf8PathBuf::from(*path), content))
            .collect();
        build_index(entries)
    }

    // ── prepare_rename tests ───────────────────────────────────────────────

    #[test]
    fn test_prepare_rename_on_wikilink() {
        let resp = prepare_rename("See [[alice]] here", 0, 8);
        assert!(resp.is_some());
        match resp.unwrap() {
            PrepareRenameResponse::RangeWithPlaceholder { range, placeholder } => {
                assert_eq!(placeholder, "alice");
                assert_eq!(range.start, Position { line: 0, character: 6 });
                assert_eq!(range.end, Position { line: 0, character: 11 });
            }
            _ => panic!("expected RangeWithPlaceholder"),
        }
    }

    #[test]
    fn test_prepare_rename_outside_wikilink() {
        assert!(prepare_rename("See [[alice]] here", 0, 0).is_none());
        assert!(prepare_rename("See [[alice]] here", 0, 16).is_none());
    }

    #[test]
    fn test_prepare_rename_with_anchor() {
        let resp = prepare_rename("[[note#section]]", 0, 4);
        assert!(resp.is_some());
        match resp.unwrap() {
            PrepareRenameResponse::RangeWithPlaceholder { placeholder, .. } => {
                assert_eq!(placeholder, "note");
            }
            _ => panic!("expected RangeWithPlaceholder"),
        }
    }

    #[test]
    fn test_prepare_rename_with_alias() {
        let resp = prepare_rename("[[note|display text]]", 0, 4);
        assert!(resp.is_some());
        match resp.unwrap() {
            PrepareRenameResponse::RangeWithPlaceholder { placeholder, .. } => {
                assert_eq!(placeholder, "note");
            }
            _ => panic!("expected RangeWithPlaceholder"),
        }
    }

    // ── compute_rename tests ───────────────────────────────────────────────

    #[test]
    fn test_rename_updates_all_references() {
        let index = make_index(&[
            ("/vault/alice.md", "# Alice\n\nSee [[bob]].\n"),
            ("/vault/bob.md", "# Bob\n"),
            ("/vault/carol.md", "# Carol\n\nAlso [[bob]] here.\n"),
        ]);
        let edit = compute_rename("See [[bob]].", 6, "robert", &index);
        assert!(edit.is_some());
        let changes = edit.unwrap().changes.unwrap();
        // Both alice.md and carol.md reference [[bob]]
        assert_eq!(changes.len(), 2);
        for edits in changes.values() {
            for e in edits {
                assert_eq!(e.new_text, "robert");
            }
        }
    }

    #[test]
    fn test_rename_no_references() {
        let index = make_index(&[
            ("/vault/alice.md", "# Alice\n"),
            ("/vault/bob.md", "# Bob\n"),
        ]);
        // "alice" has no incoming wikilinks
        let edit = compute_rename("[[alice]]", 4, "alicia", &index);
        // alice.md doesn't link to itself, so no changes found
        assert!(edit.is_none());
    }

    #[test]
    fn test_rename_preserves_anchor() {
        let index = make_index(&[
            ("/vault/doc.md", "# Doc\n\n[[target#section]]\n"),
            ("/vault/target.md", "# Target\n"),
        ]);
        let edit = compute_rename("[[target#section]]", 4, "new-target", &index);
        assert!(edit.is_some());
        let changes = edit.unwrap().changes.unwrap();
        // Should only replace "target", not "target#section"
        for edits in changes.values() {
            for e in edits {
                assert_eq!(e.new_text, "new-target");
            }
        }
    }

    #[test]
    fn test_rename_preserves_alias() {
        let index = make_index(&[
            ("/vault/doc.md", "# Doc\n\n[[target|click here]]\n"),
            ("/vault/target.md", "# Target\n"),
        ]);
        let edit = compute_rename("[[target|click here]]", 4, "new-target", &index);
        assert!(edit.is_some());
        let changes = edit.unwrap().changes.unwrap();
        for edits in changes.values() {
            for e in edits {
                assert_eq!(e.new_text, "new-target");
            }
        }
    }

}
