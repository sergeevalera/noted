use tower_lsp::lsp_types::*;

use crate::vault::VaultIndex;

/// Search headings across all vault files, filtered by query.
/// Returns flat `SymbolInformation` entries (workspace/symbol doesn't support hierarchy).
#[allow(deprecated)] // SymbolInformation::deprecated is deprecated but required by the type
pub fn compute_workspace_symbols(query: &str, index: &VaultIndex) -> Vec<SymbolInformation> {
    let query_lower = query.to_lowercase();
    let mut symbols = Vec::new();

    for entry in index.notes.values() {
        let uri = match Url::from_file_path(entry.path.as_std_path()) {
            Ok(u) => u,
            Err(_) => continue,
        };

        for heading in &entry.headings {
            // Filter: empty query returns all, otherwise substring match
            if !query_lower.is_empty() && !heading.text.to_lowercase().contains(&query_lower) {
                continue;
            }

            let kind = match heading.level {
                1 => SymbolKind::FILE,
                2 => SymbolKind::MODULE,
                3 => SymbolKind::NAMESPACE,
                _ => SymbolKind::STRING,
            };

            symbols.push(SymbolInformation {
                name: heading.text.clone(),
                kind,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position { line: heading.line, character: 0 },
                        end: Position { line: heading.line, character: 0 },
                    },
                },
                container_name: Some(entry.title.clone()),
            });
        }
    }

    // Sort by file path then line number for stable ordering
    symbols.sort_by(|a, b| {
        a.location.uri.as_str().cmp(b.location.uri.as_str())
            .then(a.location.range.start.line.cmp(&b.location.range.start.line))
    });

    symbols
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

    #[test]
    fn test_empty_query_returns_all_headings() {
        let index = make_index(&[
            ("/vault/a.md", "# Alpha\n## Beta\n"),
            ("/vault/b.md", "# Gamma\n"),
        ]);
        let symbols = compute_workspace_symbols("", &index);
        assert_eq!(symbols.len(), 3);
    }

    #[test]
    fn test_query_filters_by_substring() {
        let index = make_index(&[
            ("/vault/a.md", "# Alpha\n## Beta\n"),
            ("/vault/b.md", "# Gamma\n"),
        ]);
        let symbols = compute_workspace_symbols("bet", &index);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "Beta");
    }

    #[test]
    fn test_query_is_case_insensitive() {
        let index = make_index(&[
            ("/vault/a.md", "# Hello World\n"),
        ]);
        let symbols = compute_workspace_symbols("HELLO", &index);
        assert_eq!(symbols.len(), 1);
    }

    #[test]
    fn test_heading_levels_map_to_symbol_kinds() {
        let index = make_index(&[
            ("/vault/a.md", "# H1\n## H2\n### H3\n#### H4\n"),
        ]);
        let symbols = compute_workspace_symbols("", &index);
        assert_eq!(symbols[0].kind, SymbolKind::FILE);
        assert_eq!(symbols[1].kind, SymbolKind::MODULE);
        assert_eq!(symbols[2].kind, SymbolKind::NAMESPACE);
        assert_eq!(symbols[3].kind, SymbolKind::STRING);
    }

    #[test]
    fn test_container_name_is_note_title() {
        let index = make_index(&[
            ("/vault/a.md", "# My Note\n## Section\n"),
        ]);
        let symbols = compute_workspace_symbols("Section", &index);
        assert_eq!(symbols[0].container_name.as_deref(), Some("My Note"));
    }

    #[test]
    fn test_no_matches_returns_empty() {
        let index = make_index(&[
            ("/vault/a.md", "# Alpha\n"),
        ]);
        let symbols = compute_workspace_symbols("zzz", &index);
        assert!(symbols.is_empty());
    }
}
