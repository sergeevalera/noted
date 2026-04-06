use tower_lsp::lsp_types::*;

/// Compute code actions available at the given selection/cursor position.
///
/// Supported actions:
/// - Toggle Checkbox: `- [ ]` ↔ `- [x]`
/// - Wrap selection in bold/italic/strikethrough/code/wikilink
/// - Increase/Decrease heading level
/// - Insert callout / table on empty lines
/// - Change callout type
pub fn compute_code_actions(
    uri: &Url,
    range: Range,
    text: &str,
) -> Vec<CodeActionOrCommand> {
    let mut actions = Vec::new();

    let start_line = range.start.line as usize;
    let line = match text.lines().nth(start_line) {
        Some(l) => l,
        None => return actions,
    };

    // ── Checkbox toggle ──
    if let Some(action) = checkbox_toggle(uri, start_line as u32, line) {
        actions.push(CodeActionOrCommand::CodeAction(action));
    }

    // ── Selection wrapping ──
    let has_selection = range.start != range.end;
    if has_selection {
        for (title, before, after) in [
            ("Wrap in Bold", "**", "**"),
            ("Wrap in Italic", "*", "*"),
            ("Wrap in Strikethrough", "~~", "~~"),
            ("Wrap in Code", "`", "`"),
            ("Wrap in Wikilink", "[[", "]]"),
        ] {
            actions.push(CodeActionOrCommand::CodeAction(wrap_action(
                uri, range, title, before, after,
            )));
        }
    }

    // ── Heading level ──
    if let Some(level) = heading_level(line) {
        if level < 6 {
            actions.push(CodeActionOrCommand::CodeAction(heading_change(
                uri,
                start_line as u32,
                line,
                level,
                level + 1,
                "Increase Heading Level",
            )));
        }
        if level > 1 {
            actions.push(CodeActionOrCommand::CodeAction(heading_change(
                uri,
                start_line as u32,
                line,
                level,
                level - 1,
                "Decrease Heading Level",
            )));
        }
    }

    // ── Insert on empty line ──
    if line.trim().is_empty() {
        actions.push(CodeActionOrCommand::CodeAction(insert_action(
            uri,
            start_line as u32,
            "Insert Callout",
            "> [!note]\n> ",
        )));
        actions.push(CodeActionOrCommand::CodeAction(insert_action(
            uri,
            start_line as u32,
            "Insert Table",
            "| Column 1 | Column 2 | Column 3 |\n| --- | --- | --- |\n|  |  |  |\n",
        )));
    }

    // ── Change callout type ──
    if let Some(callout_actions) = change_callout_type(uri, start_line as u32, line) {
        actions.extend(callout_actions);
    }

    actions
}

fn checkbox_toggle(uri: &Url, line: u32, line_text: &str) -> Option<CodeAction> {
    if let Some(pos) = line_text.find("- [x]") {
        let range = Range {
            start: Position { line, character: (pos + 2) as u32 },
            end: Position { line, character: (pos + 5) as u32 },
        };
        Some(make_edit_action(uri, "Toggle Checkbox", range, "[ ]"))
    } else if let Some(pos) = line_text.find("- [ ]") {
        let range = Range {
            start: Position { line, character: (pos + 2) as u32 },
            end: Position { line, character: (pos + 5) as u32 },
        };
        Some(make_edit_action(uri, "Toggle Checkbox", range, "[x]"))
    } else {
        None
    }
}

fn wrap_action(uri: &Url, range: Range, title: &str, before: &str, after: &str) -> CodeAction {
    let mut changes = std::collections::HashMap::new();
    changes.insert(
        uri.clone(),
        vec![
            // Insert `after` first (at end) so it doesn't shift the start position
            TextEdit {
                range: Range { start: range.end, end: range.end },
                new_text: after.to_string(),
            },
            TextEdit {
                range: Range { start: range.start, end: range.start },
                new_text: before.to_string(),
            },
        ],
    );
    CodeAction {
        title: title.to_string(),
        kind: Some(CodeActionKind::REFACTOR),
        edit: Some(WorkspaceEdit { changes: Some(changes), ..Default::default() }),
        ..Default::default()
    }
}

fn heading_level(line: &str) -> Option<u8> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('#') {
        return None;
    }
    let hashes = trimmed.bytes().take_while(|&b| b == b'#').count();
    if hashes > 6 || hashes == 0 {
        return None;
    }
    // Must be followed by a space or end-of-line
    let rest = &trimmed[hashes..];
    if rest.is_empty() || rest.starts_with(' ') {
        Some(hashes as u8)
    } else {
        None
    }
}

fn heading_change(
    uri: &Url,
    line: u32,
    line_text: &str,
    old_level: u8,
    new_level: u8,
    title: &str,
) -> CodeAction {
    let leading_spaces = line_text.len() - line_text.trim_start().len();
    let old_prefix_len = leading_spaces + old_level as usize;
    let new_hashes = "#".repeat(new_level as usize);
    let range = Range {
        start: Position { line, character: leading_spaces as u32 },
        end: Position { line, character: old_prefix_len as u32 },
    };
    make_edit_action(uri, title, range, &new_hashes)
}

fn insert_action(uri: &Url, line: u32, title: &str, text: &str) -> CodeAction {
    let range = Range {
        start: Position { line, character: 0 },
        end: Position { line, character: 0 },
    };
    make_edit_action(uri, title, range, text)
}

fn change_callout_type(
    uri: &Url,
    line: u32,
    line_text: &str,
) -> Option<Vec<CodeActionOrCommand>> {
    let trimmed = line_text.trim_start();
    if !trimmed.starts_with("> [!") {
        return None;
    }
    let after_marker = &trimmed[4..];
    let end = after_marker.find(']')?;
    let current_type = &after_marker[..end];

    let leading = line_text.len() - line_text.trim_start().len();
    let type_start = leading + 4; // "> [!" is 4 chars
    let type_end = type_start + end;

    let range = Range {
        start: Position { line, character: type_start as u32 },
        end: Position { line, character: type_end as u32 },
    };

    let types = ["note", "warning", "tip", "important", "caution", "info", "abstract", "todo", "success", "question", "failure", "danger", "bug", "example", "quote"];
    let actions: Vec<CodeActionOrCommand> = types
        .iter()
        .filter(|&&t| t != current_type)
        .map(|&t| {
            CodeActionOrCommand::CodeAction(make_edit_action(
                uri,
                &format!("Change to {}", capitalize(t)),
                range,
                t,
            ))
        })
        .collect();

    Some(actions)
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn make_edit_action(uri: &Url, title: &str, range: Range, new_text: &str) -> CodeAction {
    let mut changes = std::collections::HashMap::new();
    changes.insert(
        uri.clone(),
        vec![TextEdit { range, new_text: new_text.to_string() }],
    );
    CodeAction {
        title: title.to_string(),
        kind: Some(CodeActionKind::REFACTOR),
        edit: Some(WorkspaceEdit { changes: Some(changes), ..Default::default() }),
        ..Default::default()
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_uri() -> Url {
        Url::parse("file:///test.md").unwrap()
    }

    fn action_titles(actions: &[CodeActionOrCommand]) -> Vec<String> {
        actions
            .iter()
            .map(|a| match a {
                CodeActionOrCommand::CodeAction(ca) => ca.title.clone(),
                CodeActionOrCommand::Command(c) => c.title.clone(),
            })
            .collect()
    }

    fn first_edit_text(action: &CodeActionOrCommand) -> String {
        match action {
            CodeActionOrCommand::CodeAction(ca) => {
                let ws = ca.edit.as_ref().unwrap();
                let changes = ws.changes.as_ref().unwrap();
                let edits = changes.values().next().unwrap();
                edits.iter().map(|e| e.new_text.clone()).collect::<Vec<_>>().join("")
            }
            _ => panic!("expected CodeAction"),
        }
    }

    // ── Checkbox ──

    #[test]
    fn test_toggle_unchecked_to_checked() {
        let text = "- [ ] Buy milk\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(titles.contains(&"Toggle Checkbox".to_string()));
        let toggle = actions.iter().find(|a| match a {
            CodeActionOrCommand::CodeAction(ca) => ca.title == "Toggle Checkbox",
            _ => false,
        }).unwrap();
        assert_eq!(first_edit_text(toggle), "[x]");
    }

    #[test]
    fn test_toggle_checked_to_unchecked() {
        let text = "- [x] Done task\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let toggle = actions.iter().find(|a| match a {
            CodeActionOrCommand::CodeAction(ca) => ca.title == "Toggle Checkbox",
            _ => false,
        }).unwrap();
        assert_eq!(first_edit_text(toggle), "[ ]");
    }

    #[test]
    fn test_no_checkbox_on_plain_line() {
        let text = "Just some text\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(!titles.contains(&"Toggle Checkbox".to_string()));
    }

    // ── Selection wrapping ──

    #[test]
    fn test_wrap_actions_with_selection() {
        let text = "Some text here\n";
        let range = Range {
            start: Position { line: 0, character: 5 },
            end: Position { line: 0, character: 9 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(titles.contains(&"Wrap in Bold".to_string()));
        assert!(titles.contains(&"Wrap in Italic".to_string()));
        assert!(titles.contains(&"Wrap in Strikethrough".to_string()));
        assert!(titles.contains(&"Wrap in Code".to_string()));
        assert!(titles.contains(&"Wrap in Wikilink".to_string()));
    }

    #[test]
    fn test_no_wrap_without_selection() {
        let text = "Some text here\n";
        let range = Range {
            start: Position { line: 0, character: 5 },
            end: Position { line: 0, character: 5 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(!titles.contains(&"Wrap in Bold".to_string()));
    }

    // ── Headings ──

    #[test]
    fn test_heading_increase_decrease() {
        let text = "## Heading\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(titles.contains(&"Increase Heading Level".to_string()));
        assert!(titles.contains(&"Decrease Heading Level".to_string()));
    }

    #[test]
    fn test_h1_cannot_decrease() {
        let text = "# Heading\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(titles.contains(&"Increase Heading Level".to_string()));
        assert!(!titles.contains(&"Decrease Heading Level".to_string()));
    }

    #[test]
    fn test_h6_cannot_increase() {
        let text = "###### Heading\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(!titles.contains(&"Increase Heading Level".to_string()));
        assert!(titles.contains(&"Decrease Heading Level".to_string()));
    }

    #[test]
    fn test_heading_increase_edits() {
        let text = "## Section\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let increase = actions.iter().find(|a| match a {
            CodeActionOrCommand::CodeAction(ca) => ca.title == "Increase Heading Level",
            _ => false,
        }).unwrap();
        assert_eq!(first_edit_text(increase), "###");
    }

    // ── Empty line inserts ──

    #[test]
    fn test_insert_on_empty_line() {
        let text = "Some text\n\nMore text\n";
        let range = Range {
            start: Position { line: 1, character: 0 },
            end: Position { line: 1, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(titles.contains(&"Insert Callout".to_string()));
        assert!(titles.contains(&"Insert Table".to_string()));
    }

    #[test]
    fn test_no_insert_on_non_empty_line() {
        let text = "Some text\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(!titles.contains(&"Insert Callout".to_string()));
        assert!(!titles.contains(&"Insert Table".to_string()));
    }

    // ── Callout type change ──

    #[test]
    fn test_change_callout_type() {
        let text = "> [!note] My note\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        assert!(titles.contains(&"Change to Warning".to_string()));
        assert!(titles.contains(&"Change to Tip".to_string()));
        assert!(!titles.contains(&"Change to Note".to_string())); // current type excluded
    }

    #[test]
    fn test_no_callout_change_on_plain_blockquote() {
        let text = "> Just a quote\n";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        };
        let actions = compute_code_actions(&test_uri(), range, text);
        let titles = action_titles(&actions);
        // No "Change to" actions for plain blockquotes
        assert!(!titles.iter().any(|t| t.starts_with("Change to")));
    }

    // ── heading_level helper ──

    #[test]
    fn test_heading_level_helper() {
        assert_eq!(heading_level("# H1"), Some(1));
        assert_eq!(heading_level("## H2"), Some(2));
        assert_eq!(heading_level("### H3"), Some(3));
        assert_eq!(heading_level("###### H6"), Some(6));
        assert_eq!(heading_level("####### Too many"), None);
        assert_eq!(heading_level("#no space"), None);
        assert_eq!(heading_level("plain text"), None);
        assert_eq!(heading_level("  ## Indented"), Some(2));
    }
}
