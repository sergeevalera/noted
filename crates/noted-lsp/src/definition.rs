use tower_lsp::lsp_types::*;

use crate::vault::{resolve_wikilink, VaultIndex};

/// Returns a `Location` for the wikilink target under the cursor, if any.
///
/// Finds `[[...]]` around `character`, extracts the target (before `#` or `|`),
/// resolves it against the vault index, and returns a Location pointing to
/// the start of the target file.
pub fn find_definition(line_text: &str, character: u32, index: &VaultIndex) -> Option<Location> {
    let cursor = (character as usize).min(line_text.len());

    // Walk through all [[...]] spans in the line; return the first one that
    // contains the cursor position.
    let mut search_from = 0;
    while let Some(rel_open) = line_text[search_from..].find("[[") {
        let open_pos = search_from + rel_open;
        let after_open = &line_text[open_pos..];
        let Some(rel_close) = after_open.find("]]") else { break };
        let close_pos = open_pos + rel_close;

        // Span covers open_pos..=close_pos+1 (both `]]` chars)
        if cursor >= open_pos && cursor <= close_pos + 1 {
            let inner = &line_text[open_pos + 2..close_pos];
            let target = inner.split(['#', '|']).next()?.trim();
            if target.is_empty() {
                return None;
            }
            let path = resolve_wikilink(index, target)?;
            let uri = Url::from_file_path(path.as_std_path()).ok()?;
            return Some(Location {
                uri,
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 0, character: 0 },
                },
            });
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
        notes
            .iter()
            .map(|(path, content)| parse_note(&Utf8PathBuf::from(path), content))
            .collect::<Vec<_>>()
            .pipe(build_index)
    }

    // Helper to call the pipe pattern without a trait
    trait Pipe: Sized {
        fn pipe<F, R>(self, f: F) -> R where F: FnOnce(Self) -> R { f(self) }
    }
    impl<T> Pipe for T {}

    #[test]
    fn test_definition_simple() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let loc = find_definition("See [[alice]] here", 8, &index);
        assert!(loc.is_some());
        let loc = loc.unwrap();
        assert!(loc.uri.path().ends_with("alice.md"));
        assert_eq!(loc.range.start, Position { line: 0, character: 0 });
    }

    #[test]
    fn test_definition_cursor_on_brackets() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        // Cursor on the first `[`
        assert!(find_definition("[[alice]]", 0, &index).is_some());
        // Cursor on the last `]`
        assert!(find_definition("[[alice]]", 8, &index).is_some());
    }

    #[test]
    fn test_definition_outside_wikilink() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        // Cursor after the closing `]]`
        assert!(find_definition("[[alice]] text", 12, &index).is_none());
        // Cursor before the opening `[[`
        assert!(find_definition("See [[alice]]", 0, &index).is_none());
    }

    #[test]
    fn test_definition_with_anchor() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        // `[[alice#section]]` — target is still "alice"
        let loc = find_definition("[[alice#section]]", 4, &index);
        assert!(loc.is_some());
        assert!(loc.unwrap().uri.path().ends_with("alice.md"));
    }

    #[test]
    fn test_definition_with_alias() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let loc = find_definition("[[alice|Alice In Wonderland]]", 4, &index);
        assert!(loc.is_some());
    }

    #[test]
    fn test_definition_broken_link() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        assert!(find_definition("[[nonexistent]]", 4, &index).is_none());
    }

    #[test]
    fn test_definition_multiple_links_on_line() {
        let index = make_index(&[
            ("/vault/alice.md", "# Alice\n"),
            ("/vault/bob.md", "# Bob\n"),
        ]);
        let line = "See [[alice]] and [[bob]].";
        // Cursor inside first link
        let loc = find_definition(line, 8, &index).unwrap();
        assert!(loc.uri.path().ends_with("alice.md"));
        // Cursor inside second link
        let loc = find_definition(line, 21, &index).unwrap();
        assert!(loc.uri.path().ends_with("bob.md"));
    }
}
