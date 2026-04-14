use regex::Regex;
use tower_lsp::lsp_types::*;

use crate::vault::{resolve_wikilink, VaultIndex};

// Token type indices — must match the legend order declared in `legend()`
pub const TYPE_HEADING: u32 = 0;
pub const TYPE_MARKUP: u32 = 1;
pub const TYPE_STRING: u32 = 2;
pub const TYPE_COMMENT: u32 = 3;
pub const TYPE_PUNCTUATION: u32 = 4;

// Token modifier bits (1 << legend index)
pub const MOD_H1: u32 = 1 << 0;
pub const MOD_H2: u32 = 1 << 1;
pub const MOD_H3: u32 = 1 << 2;
pub const MOD_H4: u32 = 1 << 3;
pub const MOD_H5: u32 = 1 << 4;
pub const MOD_H6: u32 = 1 << 5;
pub const MOD_BOLD: u32 = 1 << 6;
pub const MOD_ITALIC: u32 = 1 << 7;
pub const MOD_STRIKETHROUGH: u32 = 1 << 8;
pub const MOD_CODE: u32 = 1 << 9;
pub const MOD_LINK: u32 = 1 << 10;
pub const MOD_WIKILINK: u32 = 1 << 11;
pub const MOD_BROKEN: u32 = 1 << 12;
pub const MOD_TAG: u32 = 1 << 13;
pub const MOD_CALLOUT: u32 = 1 << 14;
pub const MOD_CHECKBOX_DONE: u32 = 1 << 15;
pub const MOD_CHECKBOX_TODO: u32 = 1 << 16;
pub const MOD_MATH: u32 = 1 << 17;
pub const MOD_FRONTMATTER: u32 = 1 << 18;
pub const MOD_MARKUP_PUNCTUATION: u32 = 1 << 19;

/// Compiled regex patterns for inline Markdown scanning.
struct InlineScanner {
    bold_italic_re: Regex,
    bold_re: Regex,
    strike_re: Regex,
    italic_re: Regex,
    code_re: Regex,
    wikilink_re: Regex,
    link_re: Regex,
    math_re: Regex,
    tag_re: Regex,
}

impl InlineScanner {
    fn new() -> Self {
        Self {
            bold_italic_re: Regex::new(r"\*\*\*(.+?)\*\*\*").unwrap(),
            bold_re: Regex::new(r"\*\*(.+?)\*\*").unwrap(),
            strike_re: Regex::new(r"~~(.+?)~~").unwrap(),
            italic_re: Regex::new(r"\*([^*\n]+?)\*").unwrap(),
            code_re: Regex::new(r"`([^`\n]+)`").unwrap(),
            wikilink_re: Regex::new(r"\[\[([^\]\n|#]+?)(?:[#|][^\]\n]*)?\]\]").unwrap(),
            link_re: Regex::new(r"\[([^\]\n]+)\]\(([^)\n]+)\)").unwrap(),
            math_re: Regex::new(r"\$([^$\n]+)\$").unwrap(),
            tag_re: Regex::new(r"#[A-Za-z][\w\-/]*").unwrap(),
        }
    }
}

/// The `SemanticTokensLegend` advertised in `initialize`.
/// Must stay in sync with the `TYPE_*` / `MOD_*` constants above.
pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            SemanticTokenType::new("heading"),
            SemanticTokenType::new("markup"),
            SemanticTokenType::new("string"),
            SemanticTokenType::new("comment"),
            SemanticTokenType::new("punctuation"),
        ],
        token_modifiers: vec![
            SemanticTokenModifier::new("h1"),
            SemanticTokenModifier::new("h2"),
            SemanticTokenModifier::new("h3"),
            SemanticTokenModifier::new("h4"),
            SemanticTokenModifier::new("h5"),
            SemanticTokenModifier::new("h6"),
            SemanticTokenModifier::new("bold"),
            SemanticTokenModifier::new("italic"),
            SemanticTokenModifier::new("strikethrough"),
            SemanticTokenModifier::new("code"),
            SemanticTokenModifier::new("link"),
            SemanticTokenModifier::new("wikilink"),
            SemanticTokenModifier::new("broken"),
            SemanticTokenModifier::new("tag"),
            SemanticTokenModifier::new("callout"),
            SemanticTokenModifier::new("checkbox_done"),
            SemanticTokenModifier::new("checkbox_todo"),
            SemanticTokenModifier::new("math"),
            SemanticTokenModifier::new("frontmatter"),
            SemanticTokenModifier::new("markup_punctuation"),
        ],
    }
}

/// Compute semantic tokens for the given Markdown text.
/// `index` is used to distinguish resolved vs broken wikilinks.
/// Returns tokens sorted by position and LSP delta-encoded.
pub fn compute_semantic_tokens(text: &str, index: &VaultIndex) -> Vec<SemanticToken> {
    let scanner = InlineScanner::new();
    let callout_re = Regex::new(r"^> \[!([A-Za-z][A-Za-z0-9]*)\]").unwrap();
    let checkbox_done_re = Regex::new(r"^[ \t]*(?:[-*+]|\d+[.)]) \[(?:x|X)\] ").unwrap();
    let checkbox_todo_re = Regex::new(r"^[ \t]*(?:[-*+]|\d+[.)]) \[ \] ").unwrap();

    let lines: Vec<&str> = text.lines().collect();
    let mut raw: Vec<(u32, u32, u32, u32, u32)> = Vec::new();

    // ── Frontmatter ──────────────────────────────────────────────────────────
    let frontmatter_end = detect_frontmatter(&lines);

    for (i, line) in lines[..frontmatter_end].iter().enumerate() {
        let ln = i as u32;
        let len = line.len() as u32;
        if len == 0 {
            continue;
        }
        if i == 0 || i + 1 == frontmatter_end {
            raw.push((ln, 0, len, TYPE_PUNCTUATION, MOD_MARKUP_PUNCTUATION));
        } else {
            raw.push((ln, 0, len, TYPE_COMMENT, MOD_FRONTMATTER));
        }
    }

    // ── Body ─────────────────────────────────────────────────────────────────
    let mut in_code_block = false;

    for (i, line) in lines[frontmatter_end..].iter().enumerate() {
        let ln = (frontmatter_end + i) as u32;
        let len = line.len();

        // Fenced code block fence — toggle state; skip tokenising fence lines
        if line.starts_with("```") || line.starts_with("~~~") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            continue;
        }

        // ── Headings ─────────────────────────────────────────────────────────
        if let Some((hash_len, text_start)) = parse_heading(line) {
            raw.push((
                ln,
                0,
                hash_len as u32,
                TYPE_PUNCTUATION,
                MOD_MARKUP_PUNCTUATION,
            ));
            let content_len = len.saturating_sub(text_start);
            if content_len > 0 {
                let mut heading_covered = vec![false; content_len];
                tokenize_inline_range(
                    &scanner,
                    ln,
                    line,
                    text_start,
                    len,
                    Some((TYPE_HEADING, heading_mod(hash_len))),
                    &mut heading_covered,
                    &mut raw,
                    index,
                    0,
                );
            }
            continue;
        }

        // Byte-indexed coverage map — prevents lower-priority patterns from
        // re-tokenising ranges already claimed by higher-priority ones.
        let mut covered = vec![false; len];

        // ── Callout: > [!type] ────────────────────────────────────────────────
        if let Some(m) = callout_re.find(line) {
            covered[m.start()..m.end()].fill(true);
            raw.push((
                ln,
                m.start() as u32,
                m.len() as u32,
                TYPE_MARKUP,
                MOD_CALLOUT,
            ));
        }

        // ── Checkbox ─────────────────────────────────────────────────────────
        if let Some(m) = checkbox_done_re.find(line) {
            if let Some(bracket_pos) = line[m.start()..m.end()].find('[') {
                let abs = m.start() + bracket_pos;
                covered[abs..abs + 3].fill(true);
                raw.push((ln, abs as u32, 3, TYPE_MARKUP, MOD_CHECKBOX_DONE));
            }
        } else if let Some(m) = checkbox_todo_re.find(line) {
            if let Some(bracket_pos) = line[m.start()..m.end()].find('[') {
                let abs = m.start() + bracket_pos;
                covered[abs..abs + 3].fill(true);
                raw.push((ln, abs as u32, 3, TYPE_MARKUP, MOD_CHECKBOX_TODO));
            }
        }

        // ── Inline content (with nesting support) ────────────────────────────
        tokenize_inline_range(
            &scanner,
            ln,
            line,
            0,
            len,
            None,
            &mut covered,
            &mut raw,
            index,
            0,
        );
    }

    // Convert byte offsets to UTF-16 column offsets (LSP default encoding)
    let mut raw: Vec<_> = raw
        .into_iter()
        .map(|(ln, byte_col, byte_len, ty, mods)| {
            let line_str = lines[ln as usize];
            let col = line_str[..byte_col as usize].encode_utf16().count() as u32;
            let len = line_str[byte_col as usize..(byte_col + byte_len) as usize]
                .encode_utf16()
                .count() as u32;
            (ln, col, len, ty, mods)
        })
        .collect();

    // Sort by (line, start_col) then delta-encode
    raw.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    encode_delta(raw)
}

/// Tokenize inline Markdown content in `line[start..end]`, with nesting support.
///
/// Formatting spans (bold, italic, strikethrough) recurse to handle nested elements.
/// Atomic elements (code, wikilinks, links, math, tags) are leaf-level.
/// Uncovered gaps receive `gap` tokens (e.g., heading text or bold text).
///
/// `covered` has length `end - start` and tracks which bytes are already claimed.
/// `depth` prevents runaway recursion (max 3 levels).
#[allow(clippy::too_many_arguments)]
fn tokenize_inline_range(
    scanner: &InlineScanner,
    ln: u32,
    line: &str,
    start: usize,
    end: usize,
    gap: Option<(u32, u32)>,
    covered: &mut [bool],
    raw: &mut Vec<(u32, u32, u32, u32, u32)>,
    index: &VaultIndex,
    depth: u32,
) {
    if start >= end {
        return;
    }
    if depth > 3 {
        if let Some((gt, gm)) = gap {
            raw.push((ln, start as u32, (end - start) as u32, gt, gm));
        }
        return;
    }

    let content = &line[start..end];

    // Inherit formatting modifiers from parent formatting context only
    // (not from heading context — heading gaps use TYPE_HEADING).
    let fmt_mods = match gap {
        Some((TYPE_MARKUP, mods)) => mods,
        _ => 0,
    };

    // ── Formatting spans (recurse into content) ──────────────────────────────

    // Bold+italic ***...***
    for cap in scanner.bold_italic_re.captures_iter(content) {
        let full = cap.get(0).unwrap();
        if covered[full.start()] {
            continue;
        }
        let inner = cap.get(1).unwrap();
        covered[full.start()..full.end()].fill(true);
        raw.push((
            ln,
            (start + full.start()) as u32,
            3,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        raw.push((
            ln,
            (start + inner.end()) as u32,
            3,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        let inner_mods = fmt_mods | MOD_BOLD | MOD_ITALIC;
        let mut inner_covered = vec![false; inner.end() - inner.start()];
        tokenize_inline_range(
            scanner,
            ln,
            line,
            start + inner.start(),
            start + inner.end(),
            Some((TYPE_MARKUP, inner_mods)),
            &mut inner_covered,
            raw,
            index,
            depth + 1,
        );
    }

    // Bold **...**
    for cap in scanner.bold_re.captures_iter(content) {
        let full = cap.get(0).unwrap();
        if covered[full.start()] {
            continue;
        }
        let inner = cap.get(1).unwrap();
        covered[full.start()..full.end()].fill(true);
        raw.push((
            ln,
            (start + full.start()) as u32,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        raw.push((
            ln,
            (start + inner.end()) as u32,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        let inner_mods = fmt_mods | MOD_BOLD;
        let mut inner_covered = vec![false; inner.end() - inner.start()];
        tokenize_inline_range(
            scanner,
            ln,
            line,
            start + inner.start(),
            start + inner.end(),
            Some((TYPE_MARKUP, inner_mods)),
            &mut inner_covered,
            raw,
            index,
            depth + 1,
        );
    }

    // Strikethrough ~~...~~
    for cap in scanner.strike_re.captures_iter(content) {
        let full = cap.get(0).unwrap();
        if covered[full.start()] {
            continue;
        }
        let inner = cap.get(1).unwrap();
        covered[full.start()..full.end()].fill(true);
        raw.push((
            ln,
            (start + full.start()) as u32,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        raw.push((
            ln,
            (start + inner.end()) as u32,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        let inner_mods = fmt_mods | MOD_STRIKETHROUGH;
        let mut inner_covered = vec![false; inner.end() - inner.start()];
        tokenize_inline_range(
            scanner,
            ln,
            line,
            start + inner.start(),
            start + inner.end(),
            Some((TYPE_MARKUP, inner_mods)),
            &mut inner_covered,
            raw,
            index,
            depth + 1,
        );
    }

    // Italic *...*
    for cap in scanner.italic_re.captures_iter(content) {
        let full = cap.get(0).unwrap();
        if covered[full.start()] {
            continue;
        }
        let inner = cap.get(1).unwrap();
        covered[full.start()..full.end()].fill(true);
        raw.push((
            ln,
            (start + full.start()) as u32,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        raw.push((
            ln,
            (start + inner.end()) as u32,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        let inner_mods = fmt_mods | MOD_ITALIC;
        let mut inner_covered = vec![false; inner.end() - inner.start()];
        tokenize_inline_range(
            scanner,
            ln,
            line,
            start + inner.start(),
            start + inner.end(),
            Some((TYPE_MARKUP, inner_mods)),
            &mut inner_covered,
            raw,
            index,
            depth + 1,
        );
    }

    // ── Atomic elements (no recursion) ───────────────────────────────────────

    // Inline code `...`
    for cap in scanner.code_re.captures_iter(content) {
        let full = cap.get(0).unwrap();
        if covered[full.start()] {
            continue;
        }
        let inner = cap.get(1).unwrap();
        covered[full.start()..full.end()].fill(true);
        raw.push((
            ln,
            (start + full.start()) as u32,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        raw.push((
            ln,
            (start + inner.start()) as u32,
            inner.len() as u32,
            TYPE_MARKUP,
            MOD_CODE,
        ));
        raw.push((
            ln,
            (start + inner.end()) as u32,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
    }

    // Wikilinks [[target]]
    for cap in scanner.wikilink_re.captures_iter(content) {
        let full = cap.get(0).unwrap();
        if covered[full.start()] {
            continue;
        }
        let target = cap.get(1).unwrap().as_str().trim();
        covered[full.start()..full.end()].fill(true);
        let wikilink_mod = if !index.notes.is_empty() && resolve_wikilink(index, target).is_none() {
            MOD_WIKILINK | MOD_BROKEN
        } else {
            MOD_WIKILINK
        };
        // [[ opening
        raw.push((
            ln,
            (start + full.start()) as u32,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        // inner content
        let inner_start = full.start() + 2;
        let inner_end = full.end() - 2;
        if inner_end > inner_start {
            raw.push((
                ln,
                (start + inner_start) as u32,
                (inner_end - inner_start) as u32,
                TYPE_MARKUP,
                wikilink_mod,
            ));
        }
        // ]] closing
        raw.push((
            ln,
            (start + full.end() - 2) as u32,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
    }

    // Regular links [text](url)
    for cap in scanner.link_re.captures_iter(content) {
        let full = cap.get(0).unwrap();
        if covered[full.start()] {
            continue;
        }
        let text_m = cap.get(1).unwrap();
        let url_m = cap.get(2).unwrap();
        covered[full.start()..full.end()].fill(true);
        raw.push((
            ln,
            (start + full.start()) as u32,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        )); // [
        raw.push((
            ln,
            (start + text_m.start()) as u32,
            text_m.len() as u32,
            TYPE_STRING,
            MOD_LINK,
        ));
        raw.push((
            ln,
            (start + text_m.end()) as u32,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        )); // ](
        raw.push((
            ln,
            (start + url_m.start()) as u32,
            url_m.len() as u32,
            TYPE_STRING,
            MOD_LINK,
        ));
        raw.push((
            ln,
            (start + url_m.end()) as u32,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        )); // )
    }

    // Math $...$
    for cap in scanner.math_re.captures_iter(content) {
        let full = cap.get(0).unwrap();
        if covered[full.start()] {
            continue;
        }
        let inner = cap.get(1).unwrap();
        covered[full.start()..full.end()].fill(true);
        raw.push((
            ln,
            (start + full.start()) as u32,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
        raw.push((
            ln,
            (start + inner.start()) as u32,
            inner.len() as u32,
            TYPE_MARKUP,
            MOD_MATH,
        ));
        raw.push((
            ln,
            (start + inner.end()) as u32,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION,
        ));
    }

    // Tags #word
    for m in scanner.tag_re.find_iter(content) {
        if covered[m.start()] {
            continue;
        }
        covered[m.start()..m.end()].fill(true);
        raw.push((
            ln,
            (start + m.start()) as u32,
            m.len() as u32,
            TYPE_MARKUP,
            MOD_TAG,
        ));
    }

    // ── Gap tokens for uncovered ranges ──────────────────────────────────────
    if let Some((gt, gm)) = gap {
        emit_gaps(ln, start, covered, gt, gm, raw);
    }
}

/// Emit tokens for uncovered byte ranges within a content span.
fn emit_gaps(
    ln: u32,
    abs_start: usize,
    covered: &[bool],
    gap_type: u32,
    gap_mods: u32,
    raw: &mut Vec<(u32, u32, u32, u32, u32)>,
) {
    let mut i = 0;
    while i < covered.len() {
        if !covered[i] {
            let gap_start = i;
            while i < covered.len() && !covered[i] {
                i += 1;
            }
            raw.push((
                ln,
                (abs_start + gap_start) as u32,
                (i - gap_start) as u32,
                gap_type,
                gap_mods,
            ));
        } else {
            i += 1;
        }
    }
}

// ── Delta helpers ─────────────────────────────────────────────────────────────

/// Flatten a token list into the raw u32 array used by the LSP wire format (5 u32 per token).
pub fn tokens_to_flat(tokens: &[SemanticToken]) -> Vec<u32> {
    let mut flat = Vec::with_capacity(tokens.len() * 5);
    for t in tokens {
        flat.extend_from_slice(&[
            t.delta_line,
            t.delta_start,
            t.length,
            t.token_type,
            t.token_modifiers_bitset,
        ]);
    }
    flat
}

/// Reconstruct `SemanticToken` objects from a flat u32 array (5 u32 per token).
fn flat_to_tokens(flat: &[u32]) -> Vec<SemanticToken> {
    flat.chunks_exact(5)
        .map(|c| SemanticToken {
            delta_line: c[0],
            delta_start: c[1],
            length: c[2],
            token_type: c[3],
            token_modifiers_bitset: c[4],
        })
        .collect()
}

/// Compute the minimal edit set to transform `old` into `new` (both flat u32 arrays).
///
/// `start` / `delete_count` are in raw u32 element units; `data` contains the replacement tokens.
/// Returns an empty vec when the arrays are identical (unchanged file → empty delta).
pub fn compute_token_delta(old: &[u32], new: &[u32]) -> Vec<SemanticTokensEdit> {
    if old == new {
        return vec![];
    }

    // Find the common prefix (must align to a 5-element token boundary).
    let raw_prefix = old
        .iter()
        .zip(new.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let prefix = (raw_prefix / 5) * 5;

    // Find the common suffix in the remaining slices.
    // Suffix is safe to reuse only when the encoded u32 values match exactly, which implies
    // the delta encoding is consistent with the shared preceding context.
    let old_rest = &old[prefix..];
    let new_rest = &new[prefix..];
    let raw_suffix = old_rest
        .iter()
        .rev()
        .zip(new_rest.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let suffix = (raw_suffix / 5) * 5;

    let delete_count = (old_rest.len() - suffix) as u32;
    let insert = &new_rest[..new_rest.len() - suffix];

    vec![SemanticTokensEdit {
        start: prefix as u32,
        delete_count,
        data: if insert.is_empty() {
            None
        } else {
            Some(flat_to_tokens(insert))
        },
    }]
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Returns the exclusive end-line index of the frontmatter block (0 = no frontmatter).
fn detect_frontmatter(lines: &[&str]) -> usize {
    if lines.is_empty() || lines[0].trim() != "---" {
        return 0;
    }
    for (i, line) in lines[1..].iter().enumerate() {
        let t = line.trim();
        if t == "---" || t == "..." {
            return i + 2; // i is 0-based in lines[1..], so actual index is i+1; exclusive end = i+2
        }
    }
    0 // No closing marker — treat as no frontmatter
}

/// Returns `Some((hash_count, text_start))` if `line` is an ATX heading (`# …` through `###### …`).
fn parse_heading(line: &str) -> Option<(usize, usize)> {
    let hash_count = line.bytes().take_while(|&b| b == b'#').count();
    if hash_count == 0 || hash_count > 6 {
        return None;
    }
    if !line[hash_count..].starts_with(' ') {
        return None;
    }
    Some((hash_count, hash_count + 1))
}

fn heading_mod(level: usize) -> u32 {
    match level {
        1 => MOD_H1,
        2 => MOD_H2,
        3 => MOD_H3,
        4 => MOD_H4,
        5 => MOD_H5,
        _ => MOD_H6,
    }
}

/// Convert absolute `(line, col, len, type, mods)` tuples to LSP delta-encoded tokens.
fn encode_delta(raw: Vec<(u32, u32, u32, u32, u32)>) -> Vec<SemanticToken> {
    let mut tokens = Vec::with_capacity(raw.len());
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;
    for (line, start, length, token_type, modifiers) in raw {
        let delta_line = line - prev_line;
        let delta_start = if delta_line == 0 {
            start - prev_start
        } else {
            start
        };
        tokens.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset: modifiers,
        });
        prev_line = line;
        prev_start = start;
    }
    tokens
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{build_index, parse_note};
    use camino::Utf8PathBuf;

    /// Decode delta-encoded tokens back to absolute `(line, col, len, type, mods)` tuples.
    fn decode(tokens: &[SemanticToken]) -> Vec<(u32, u32, u32, u32, u32)> {
        let mut result = Vec::new();
        let mut line = 0u32;
        let mut col = 0u32;
        for t in tokens {
            line += t.delta_line;
            col = if t.delta_line == 0 {
                col + t.delta_start
            } else {
                t.delta_start
            };
            result.push((line, col, t.length, t.token_type, t.token_modifiers_bitset));
        }
        result
    }

    fn empty_index() -> VaultIndex {
        VaultIndex::default()
    }

    fn index_with_note(path: &str, content: &str) -> VaultIndex {
        build_index(vec![parse_note(&Utf8PathBuf::from(path), content)])
    }

    fn has(
        tokens: &[(u32, u32, u32, u32, u32)],
        line: u32,
        col: u32,
        len: u32,
        ty: u32,
        mods: u32,
    ) -> bool {
        tokens.iter().any(|t| *t == (line, col, len, ty, mods))
    }

    #[test]
    fn test_h1_heading() {
        let idx = empty_index();
        let tokens = decode(&compute_semantic_tokens("# Title\n", &idx));
        // `#` → punctuation + markup_punctuation
        assert!(has(
            &tokens,
            0,
            0,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // `Title` (5 chars) → heading + h1
        assert!(has(&tokens, 0, 2, 5, TYPE_HEADING, MOD_H1));
    }

    #[test]
    fn test_h3_heading() {
        let idx = empty_index();
        let tokens = decode(&compute_semantic_tokens("### Sub\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            3,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 4, 3, TYPE_HEADING, MOD_H3));
    }

    #[test]
    fn test_h6_heading() {
        let idx = empty_index();
        let tokens = decode(&compute_semantic_tokens("###### Deep\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            6,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 7, 4, TYPE_HEADING, MOD_H6));
    }

    #[test]
    fn test_bold() {
        let idx = empty_index();
        let tokens = decode(&compute_semantic_tokens("**hello**\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 2, 5, TYPE_MARKUP, MOD_BOLD));
        assert!(has(
            &tokens,
            0,
            7,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_italic() {
        let idx = empty_index();
        let tokens = decode(&compute_semantic_tokens("*hello*\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 1, 5, TYPE_MARKUP, MOD_ITALIC));
        assert!(has(
            &tokens,
            0,
            6,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_strikethrough() {
        let idx = empty_index();
        let tokens = decode(&compute_semantic_tokens("~~gone~~\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 2, 4, TYPE_MARKUP, MOD_STRIKETHROUGH));
        assert!(has(
            &tokens,
            0,
            6,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_inline_code() {
        let idx = empty_index();
        let tokens = decode(&compute_semantic_tokens("`hello`\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 1, 5, TYPE_MARKUP, MOD_CODE));
        assert!(has(
            &tokens,
            0,
            6,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_wikilink_resolved() {
        let idx = index_with_note("/vault/target.md", "# Target\n");
        let tokens = decode(&compute_semantic_tokens("[[target]]\n", &idx));
        // [[ at 0, `target` (6 chars) at 2, ]] at 8
        assert!(has(
            &tokens,
            0,
            0,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 2, 6, TYPE_MARKUP, MOD_WIKILINK));
        assert!(has(
            &tokens,
            0,
            8,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // Must NOT be broken
        assert!(!tokens.iter().any(|t| t.4 & MOD_BROKEN != 0));
    }

    #[test]
    fn test_wikilink_broken() {
        let idx = index_with_note("/vault/other.md", "# Other\n");
        let tokens = decode(&compute_semantic_tokens("[[missing]]\n", &idx));
        assert!(has(
            &tokens,
            0,
            2,
            7,
            TYPE_MARKUP,
            MOD_WIKILINK | MOD_BROKEN
        ));
    }

    #[test]
    fn test_wikilink_empty_index_not_broken() {
        // Before vault indexing completes, wikilinks should not be flagged broken
        let idx = empty_index();
        let tokens = decode(&compute_semantic_tokens("[[anything]]\n", &idx));
        assert!(!tokens.iter().any(|t| t.4 & MOD_BROKEN != 0));
    }

    #[test]
    fn test_regular_link() {
        let idx = empty_index();
        // [text](url) — 12 chars: [ at 0, text 1-4, ]( at 5-6, url 7-9, ) at 10
        let tokens = decode(&compute_semantic_tokens("[text](url)\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        )); // [
        assert!(has(&tokens, 0, 1, 4, TYPE_STRING, MOD_LINK)); // text
        assert!(has(
            &tokens,
            0,
            5,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        )); // ](
        assert!(has(&tokens, 0, 7, 3, TYPE_STRING, MOD_LINK)); // url
        assert!(has(
            &tokens,
            0,
            10,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        )); // )
    }

    #[test]
    fn test_tag() {
        let idx = empty_index();
        // "Hello #world" — #world starts at col 6, len 6
        let tokens = decode(&compute_semantic_tokens("Hello #world\n", &idx));
        assert!(has(&tokens, 0, 6, 6, TYPE_MARKUP, MOD_TAG));
    }

    #[test]
    fn test_math() {
        let idx = empty_index();
        // "$x+1$" — $ at 0, x+1 at 1 (len 3), $ at 4
        let tokens = decode(&compute_semantic_tokens("$x+1$\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 1, 3, TYPE_MARKUP, MOD_MATH));
        assert!(has(
            &tokens,
            0,
            4,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_frontmatter() {
        let idx = empty_index();
        let text = "---\ntitle: Foo\n---\n";
        let tokens = decode(&compute_semantic_tokens(text, &idx));
        // Line 0: `---` (3 chars) → punctuation + markup_punctuation
        assert!(has(
            &tokens,
            0,
            0,
            3,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // Line 1: `title: Foo` (10 chars) → comment + frontmatter
        assert!(has(&tokens, 1, 0, 10, TYPE_COMMENT, MOD_FRONTMATTER));
        // Line 2: `---` → punctuation + markup_punctuation
        assert!(has(
            &tokens,
            2,
            0,
            3,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_callout() {
        let idx = empty_index();
        // "> [!NOTE]" — 9 chars → markup + callout
        let tokens = decode(&compute_semantic_tokens("> [!NOTE]\n", &idx));
        assert!(has(&tokens, 0, 0, 9, TYPE_MARKUP, MOD_CALLOUT));
    }

    #[test]
    fn test_checkbox_done() {
        let idx = empty_index();
        // "- [x] item" — [x] at col 2, len 3
        let tokens = decode(&compute_semantic_tokens("- [x] item\n", &idx));
        assert!(has(&tokens, 0, 2, 3, TYPE_MARKUP, MOD_CHECKBOX_DONE));
    }

    #[test]
    fn test_checkbox_todo() {
        let idx = empty_index();
        // "- [ ] item" — [ ] at col 2, len 3
        let tokens = decode(&compute_semantic_tokens("- [ ] item\n", &idx));
        assert!(has(&tokens, 0, 2, 3, TYPE_MARKUP, MOD_CHECKBOX_TODO));
    }

    // ── Delta tests ──────────────────────────────────────────────────────────

    #[test]
    fn test_delta_unchanged_returns_empty_edits() {
        let idx = empty_index();
        let tokens = compute_semantic_tokens("# Hello\n", &idx);
        let flat = tokens_to_flat(&tokens);
        let edits = compute_token_delta(&flat, &flat);
        assert!(edits.is_empty(), "unchanged content must produce no edits");
    }

    #[test]
    fn test_delta_changed_returns_edits() {
        let idx = empty_index();
        let old_flat = tokens_to_flat(&compute_semantic_tokens("# Hello\n", &idx));
        let new_flat = tokens_to_flat(&compute_semantic_tokens("## World\n", &idx));
        let edits = compute_token_delta(&old_flat, &new_flat);
        assert!(!edits.is_empty(), "changed content must produce edits");
    }

    #[test]
    fn test_delta_applying_edit_produces_new_flat() {
        let idx = empty_index();
        let old_flat = tokens_to_flat(&compute_semantic_tokens("plain text\n", &idx));
        let new_flat = tokens_to_flat(&compute_semantic_tokens("# Heading\n", &idx));
        let edits = compute_token_delta(&old_flat, &new_flat);

        // Simulate the client applying the edit to the flat u32 array
        let mut result = old_flat.clone();
        for edit in &edits {
            let start = edit.start as usize;
            let end = start + edit.delete_count as usize;
            let data_flat = edit
                .data
                .as_deref()
                .map(|tokens| tokens_to_flat(tokens))
                .unwrap_or_default();
            result.splice(start..end, data_flat);
        }
        assert_eq!(
            result, new_flat,
            "applying edits must produce the new flat array"
        );
    }

    #[test]
    fn test_tokens_to_flat_roundtrip() {
        let idx = empty_index();
        let tokens = compute_semantic_tokens("**bold** and *italic*\n", &idx);
        let flat = tokens_to_flat(&tokens);
        assert_eq!(flat.len(), tokens.len() * 5);
        // Verify each token's 5 values are in order
        for (i, t) in tokens.iter().enumerate() {
            let base = i * 5;
            assert_eq!(flat[base], t.delta_line);
            assert_eq!(flat[base + 1], t.delta_start);
            assert_eq!(flat[base + 2], t.length);
            assert_eq!(flat[base + 3], t.token_type);
            assert_eq!(flat[base + 4], t.token_modifiers_bitset);
        }
    }

    #[test]
    fn test_no_tokens_inside_fenced_code_block() {
        let idx = empty_index();
        let text = "```\n**bold**\n```\n";
        let tokens = decode(&compute_semantic_tokens(text, &idx));
        // No bold tokens should appear inside the code block
        assert!(!tokens.iter().any(|t| t.4 & MOD_BOLD != 0));
    }

    #[test]
    fn test_bold_not_retokenised_as_italic() {
        let idx = empty_index();
        // The inner `*text*` of `**text**` must not produce an italic token
        let tokens = decode(&compute_semantic_tokens("**text**\n", &idx));
        assert!(!tokens.iter().any(|t| t.4 & MOD_ITALIC != 0));
        assert!(tokens.iter().any(|t| t.4 & MOD_BOLD != 0));
    }

    // ── Nesting tests ────────────────────────────────────────────────────────

    #[test]
    fn test_heading_with_bold() {
        let idx = empty_index();
        // "## Heading **bold** text"
        //  01234567890123456789012345
        let tokens = decode(&compute_semantic_tokens("## Heading **bold** text\n", &idx));
        // ## at 0
        assert!(has(
            &tokens,
            0,
            0,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "Heading " at 3 (8 chars) → heading+h2 (gap)
        assert!(has(&tokens, 0, 3, 8, TYPE_HEADING, MOD_H2));
        // ** at 11
        assert!(has(
            &tokens,
            0,
            11,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "bold" at 13 (4 chars) → markup+bold
        assert!(has(&tokens, 0, 13, 4, TYPE_MARKUP, MOD_BOLD));
        // ** at 17
        assert!(has(
            &tokens,
            0,
            17,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // " text" at 19 (5 chars) → heading+h2 (gap)
        assert!(has(&tokens, 0, 19, 5, TYPE_HEADING, MOD_H2));
    }

    #[test]
    fn test_heading_with_inline_code() {
        let idx = empty_index();
        // "# Title `code` end"
        //  0123456789012345678
        let tokens = decode(&compute_semantic_tokens("# Title `code` end\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "Title " at 2 (6 chars) → heading+h1
        assert!(has(&tokens, 0, 2, 6, TYPE_HEADING, MOD_H1));
        // ` at 8
        assert!(has(
            &tokens,
            0,
            8,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "code" at 9 (4 chars) → markup+code
        assert!(has(&tokens, 0, 9, 4, TYPE_MARKUP, MOD_CODE));
        // ` at 13
        assert!(has(
            &tokens,
            0,
            13,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // " end" at 14 (4 chars) → heading+h1
        assert!(has(&tokens, 0, 14, 4, TYPE_HEADING, MOD_H1));
    }

    #[test]
    fn test_heading_with_wikilink() {
        let idx = index_with_note("/vault/setup.md", "# Setup\n");
        // "## See [[setup]]"
        //  01234567890123456
        let tokens = decode(&compute_semantic_tokens("## See [[setup]]\n", &idx));
        assert!(has(
            &tokens,
            0,
            0,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "See " at 3 (4 chars) → heading+h2
        assert!(has(&tokens, 0, 3, 4, TYPE_HEADING, MOD_H2));
        // [[ at 7
        assert!(has(
            &tokens,
            0,
            7,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "setup" at 9 (5 chars) → markup+wikilink
        assert!(has(&tokens, 0, 9, 5, TYPE_MARKUP, MOD_WIKILINK));
        // ]] at 14
        assert!(has(
            &tokens,
            0,
            14,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_bold_with_code_inside() {
        let idx = empty_index();
        // "**bold `code` more**"
        //  01234567890123456789
        let tokens = decode(&compute_semantic_tokens("**bold `code` more**\n", &idx));
        // ** at 0
        assert!(has(
            &tokens,
            0,
            0,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "bold " at 2 (5 chars) → markup+bold (gap)
        assert!(has(&tokens, 0, 2, 5, TYPE_MARKUP, MOD_BOLD));
        // ` at 7
        assert!(has(
            &tokens,
            0,
            7,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "code" at 8 (4 chars) → markup+code
        assert!(has(&tokens, 0, 8, 4, TYPE_MARKUP, MOD_CODE));
        // ` at 12
        assert!(has(
            &tokens,
            0,
            12,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // " more" at 13 (5 chars) → markup+bold (gap)
        assert!(has(&tokens, 0, 13, 5, TYPE_MARKUP, MOD_BOLD));
        // ** at 18
        assert!(has(
            &tokens,
            0,
            18,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_code_with_multibyte_char() {
        let idx = empty_index();
        // "shows `✓` end" — ✓ is 3 bytes in UTF-8, 1 code unit in UTF-16
        // UTF-16 positions: ` at 6, ✓ at 7 (len 1), ` at 8
        let tokens = decode(&compute_semantic_tokens("shows `✓` end\n", &idx));
        assert!(has(
            &tokens,
            0,
            6,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 7, 1, TYPE_MARKUP, MOD_CODE)); // len 1, not 3
        assert!(has(
            &tokens,
            0,
            8,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_bold_after_multibyte_char() {
        let idx = empty_index();
        // "— **bold** end" — em dash is 3 bytes UTF-8, 1 UTF-16
        // UTF-16: — at 0, space at 1, ** at 2, bold at 4, ** at 8
        let tokens = decode(&compute_semantic_tokens("— **bold** end\n", &idx));
        assert!(has(
            &tokens,
            0,
            2,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        assert!(has(&tokens, 0, 4, 4, TYPE_MARKUP, MOD_BOLD));
        assert!(has(
            &tokens,
            0,
            8,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }

    #[test]
    fn test_italic_inside_bold() {
        let idx = empty_index();
        // "**bold *italic* text**"
        //  0123456789012345678901
        let tokens = decode(&compute_semantic_tokens("**bold *italic* text**\n", &idx));
        // ** at 0
        assert!(has(
            &tokens,
            0,
            0,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "bold " at 2 (5 chars) → markup+bold (gap)
        assert!(has(&tokens, 0, 2, 5, TYPE_MARKUP, MOD_BOLD));
        // * at 7
        assert!(has(
            &tokens,
            0,
            7,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // "italic" at 8 (6 chars) → markup+bold+italic (inherited)
        assert!(has(&tokens, 0, 8, 6, TYPE_MARKUP, MOD_BOLD | MOD_ITALIC));
        // * at 14
        assert!(has(
            &tokens,
            0,
            14,
            1,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
        // " text" at 15 (5 chars) → markup+bold (gap)
        assert!(has(&tokens, 0, 15, 5, TYPE_MARKUP, MOD_BOLD));
        // ** at 20
        assert!(has(
            &tokens,
            0,
            20,
            2,
            TYPE_PUNCTUATION,
            MOD_MARKUP_PUNCTUATION
        ));
    }
}
