# CLAUDE.md — Instructions for Claude Agent

> This file provides project context for Claude Code / Zed Agent.
> Read it before every working session on the project.

## Agent Rules

- **No subagents without permission.** Never use the Agent tool (subagents) without
  explicitly asking the user first. Do the work directly instead.
- **English only for project files.** All documentation, comments, and commit messages
  must be in English, even if the user communicates in Russian.

## Project

**noted** — a Zed IDE extension for Markdown knowledge bases. Supports wikilinks,
callouts, tags, smart navigation, and live preview. Works with Obsidian vault file
structure. Includes a custom Tree-sitter grammar, an LSP server written in Rust,
and a companion theme.

## Repositories

- `noted/` — main extension (extension + LSP + grammar)
- `noted-theme/` — companion theme (separate extension, JSON-only)

## Architecture

```
┌──────────────────────────────────────────────────┐
│                    Zed IDE                       │
│                                                  │
│  ┌──────────────┐    ┌───────────────────────┐   │
│  │  Extension   │    │  Editor               │   │
│  │  (WASM)      │    │  + Tree-sitter grammar│   │
│  │  lib.rs      │    │  + Semantic tokens    │   │
│  │  - register  │    │  + Inlay hints        │   │
│  │    language  │    │  + Theme styles       │   │
│  │  - start LSP │    └───────────┬───────────┘   │
│  └──────┬───────┘                │ LSP Protocol  │
│         │ spawn                  │ (stdin/stdout)│
│         ▼                        ▼               │
│  ┌─────────────────────────────────────────────┐ │
│  │           noted-lsp (Rust binary)           │ │
│  │                                             │ │
│  │  ┌──────────┐ ┌────────────┐ ┌───────────┐  │ │
│  │  │ Indexer  │ │ Completion │ │ Semantic  │  │ │
│  │  │ (vault   │ │ (wikilinks)│ │ Tokens    │  │ │
│  │  │  scan)   │ │            │ │           │  │ │
│  │  ├──────────┤ ├────────────┤ ├───────────┤  │ │
│  │  │ Hover    │ │ Diagnostics│ │ Code      │  │ │
│  │  │ (preview)│ │ (broken    │ │ Actions   │  │ │
│  │  │          │ │  links)    │ │           │  │ │
│  │  ├──────────┤ ├────────────┤ ├───────────┤  │ │
│  │  │ Rename   │ │ Doc Symbols│ │ Inlay     │  │ │
│  │  │          │ │            │ │ Hints     │  │ │
│  │  └──────────┘ └────────────┘ └───────────┘  │ │
│  │                                             │ │
│  │  ┌──────────────────────────────────────┐   │ │
│  │  │ Preview Server (Phase 3)             │   │ │
│  │  │ axum HTTP + tokio-tungstenite WS     │   │ │
│  │  └──────────────────────────────────────┘   │ │
│  └─────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘
```

## Tech Stack

- **Extension:** Rust → `wasm32-wasip2`, `zed_extension_api` crate
- **LSP:** Rust, `tower-lsp` + `tokio`, `pulldown-cmark`, `walkdir`, `regex`, `camino`
- **Grammar:** JavaScript (`grammar.js`), Tree-sitter CLI
- **Theme:** JSON (Zed Theme Schema v0.2.0)

## Project Structure

```
noted/
├── extension.toml              # Zed extension manifest
├── Cargo.toml                  # Workspace root + WASM extension crate
├── src/lib.rs                  # Extension entry: locate LSP binary via NOTED_LSP_PATH
├── languages/noted/
│   ├── config.toml             # Language config (grammar = "noted")
│   ├── highlights.scm          # Syntax highlighting queries (inactive without grammar)
│   ├── injections.scm          # Code block language injection
│   ├── outline.scm             # Outline panel queries
│   └── folds.scm               # Code folding queries
├── grammars/noted/
│   ├── grammar.js              # Tree-sitter grammar definition
│   ├── src/                    # Generated C parser (parser.c, tree_sitter/parser.h)
│   └── test/corpus/            # Tree-sitter test cases
└── crates/noted-lsp/
    ├── Cargo.toml              # LSP server crate
    └── src/
        ├── main.rs             # Entry point, server state, LSP handler dispatch
        ├── code_actions.rs     # textDocument/codeAction (checkbox, wrap, heading, callout)
        ├── completion.rs       # textDocument/completion (wikilinks, trigger: `[`)
        ├── definition.rs       # textDocument/definition (go-to wikilink target)
        ├── diagnostics.rs      # publishDiagnostics (broken wikilinks)
        ├── hover.rs            # textDocument/hover (note title + snippet + metadata)
        ├── inlay_hints.rs      # textDocument/inlayHint (checkbox ✓/○ markers)
        ├── semantic_tokens.rs  # textDocument/semanticTokens (headings, bold, italic)
        ├── symbols.rs          # textDocument/documentSymbol (heading tree)
        └── vault/
            ├── mod.rs          # Re-exports
            ├── index.rs        # VaultIndex, build_index, resolve_wikilink
            ├── parser.rs       # parse_note: headings, wikilinks, tags, frontmatter
            └── scanner.rs      # scan_vault: walkdir, skips hidden dirs
```

## Coding Rules

### Rust

- Edition 2021, stable toolchain
- Formatting: `cargo fmt` (rustfmt defaults)
- Linting: `cargo clippy -- -D warnings`
- Tests: `cargo test` must pass before every commit
- Error handling: use `anyhow` for LSP, `thiserror` for library errors
- Async: `tokio` runtime, `tower-lsp` async handlers
- Logging: `tracing` crate (`info!`, `warn!`, `error!`, `debug!`)
- Do not use `unwrap()` in production code — only in tests
- Prefer `&str` over `String` in function arguments
- Use `camino::Utf8PathBuf` instead of `std::path::PathBuf` for paths

### Tree-sitter

- Every new grammar rule must be accompanied by a test in `test/corpus/`
- Test format: standard tree-sitter test format
- After changing `grammar.js` → run `tree-sitter generate` + `tree-sitter test`
- Commit the generated `src/parser.c` and `src/tree_sitter/parser.h`
- Re-enable grammar in `languages/noted/config.toml` and `extension.toml`
  once the C files are committed

### Commits

- Format: `type(scope): description`
- Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`
- Examples: `feat(lsp): add wikilink completion`, `fix(grammar): handle empty callouts`

## Semantic Token Types (for LSP)

Full implementation complete (Phase 2 step 2.1.1 + 2.1.2):

**Zed limitation:** Zed only recognizes **standard LSP semantic token types**. Custom
type names are silently ignored. The LSP therefore maps tokens to standard types:

| LSP type (legend) | Used for | Theme `syntax` key |
|---|---|---|
| `keyword` | headings (H1–H6) | `keyword` |
| `variable` | markup (bold, italic, strikethrough, code, wikilink, tag, callout, checkbox, math) | `variable` |
| `string` | links, link text | `string` |
| `comment` | frontmatter | `comment` |
| `operator` | punctuation markers (`#`, `**`, `[[`, `]]`, etc.) | `operator` |

**Token modifiers** (internal, used for computation logic and delta caching):
`h1`–`h6`, `bold`, `italic`, `strikethrough`, `code`, `link`, `wikilink`, `broken`,
`tag`, `callout`, `checkbox_done`, `checkbox_todo`, `math`, `frontmatter`,
`markup_punctuation`

Delta caching: per-document flat u32 token cache; unchanged files return empty delta.
Broken wikilinks flagged with `broken` modifier only after vault index is populated.

**User setup:** Add `"semantic_tokens": "combined"` to `settings.json` under
`languages > Noted Markdown` (default is `"off"`, which disables semantic tokens).
Styling comes from the theme's `syntax` section — no `semantic_token_rules` needed.

### Future: Custom Token Types (when Zed adds support)

When Zed supports custom semantic token types and modifiers in `semantic_token_rules`,
revert the legend to use descriptive names and add per-element styling:

**Target legend (restore in `semantic_tokens.rs`):**

| Token type | Token modifier | Element |
|---|---|---|
| `heading` | `h1`–`h6` | Heading levels with distinct colors/weights |
| `markup` | `bold` | Bold text (`**...**`) — font_weight: bold |
| `markup` | `italic` | Italic text (`*...*`) — font_style: italic |
| `markup` | `strikethrough` | Strikethrough (`~~...~~`) — dimmed color |
| `markup` | `code` | Inline code (`` `...` ``) — monospace, gold/amber |
| `markup` | `wikilink` | Wikilink target (`[[...]]`) — cyan/teal, underline |
| `markup` | `wikilink`, `broken` | Broken wikilink — red, underline |
| `markup` | `tag` | Tag (`#tag`) — brown/gold, subtle background |
| `markup` | `callout` | Callout header (`> [!type]`) — pink, italic |
| `markup` | `checkbox_done` | Checked `[x]` — dimmed |
| `markup` | `checkbox_todo` | Unchecked `[ ]` — normal |
| `markup` | `math` | Math (`$...$`) — green, italic |
| `comment` | `frontmatter` | YAML frontmatter — dimmed, italic |
| `punctuation` | `markup_punctuation` | Markdown delimiters — very dimmed |
| `string` | `link` | Standard links `[text](url)` |

**How to migrate:**

1. In `semantic_tokens.rs`, change `legend()` token types back from standard names
   (`keyword`, `variable`, `operator`) to custom names (`heading`, `markup`, `punctuation`)
2. Add `semantic_token_rules` to the companion theme or recommend in `settings.json`:
   ```json
   "semantic_token_rules": [
     { "token_type": "heading", "token_modifiers": ["h1"], "foreground_color": "#8FBF6A", "font_weight": "bold" },
     { "token_type": "heading", "token_modifiers": ["h2"], "foreground_color": "#7AAD58", "font_weight": "bold" },
     { "token_type": "heading", "token_modifiers": ["h3"], "foreground_color": "#669B48", "font_weight": "bold" },
     { "token_type": "heading", "token_modifiers": ["h4"], "foreground_color": "#548938" },
     { "token_type": "heading", "token_modifiers": ["h5"], "foreground_color": "#447830" },
     { "token_type": "heading", "token_modifiers": ["h6"], "foreground_color": "#3A6828" },
     { "token_type": "markup", "token_modifiers": ["bold"], "font_weight": "bold" },
     { "token_type": "markup", "token_modifiers": ["italic"], "font_style": "italic" },
     { "token_type": "markup", "token_modifiers": ["strikethrough"], "foreground_color": "#7A7C72" },
     { "token_type": "markup", "token_modifiers": ["code"], "foreground_color": "#E0B460" },
     { "token_type": "markup", "token_modifiers": ["wikilink"], "foreground_color": "#7CB5C4" },
     { "token_type": "markup", "token_modifiers": ["broken"], "foreground_color": "#CC4444" },
     { "token_type": "markup", "token_modifiers": ["tag"], "foreground_color": "#D4A56A" },
     { "token_type": "markup", "token_modifiers": ["callout"], "foreground_color": "#C47D8A", "font_style": "italic" },
     { "token_type": "markup", "token_modifiers": ["math"], "foreground_color": "#B8DC94", "font_style": "italic" },
     { "token_type": "comment", "token_modifiers": ["frontmatter"], "foreground_color": "#545648", "font_style": "italic" },
     { "token_type": "punctuation", "token_modifiers": ["markup_punctuation"], "foreground_color": "#4A4A40" }
   ]
   ```
3. Test: verify each element gets its distinct color
4. Track Zed issue: watch for custom semantic token support in Zed changelogs

## Execution Plan Progress

### Phase 0 (Scaffolding & Research) — COMPLETE ✓

- [x] 0.1.1 `[HUMAN]` Install Rust, wasm target, tree-sitter-cli
- [x] 0.1.2 `[AGENT]` Cargo workspace structure (extension + LSP crates)
- [x] 0.1.3 `[AGENT]` Minimal `src/lib.rs` for Zed extension
- [x] 0.1.4 `[HUMAN]` Install as dev extension in Zed
- [x] 0.2.1 `[AGENT]` Hello world LSP (tower-lsp, hover)
- [x] 0.2.2 `[AGENT]` Connect extension to LSP binary
- [x] 0.2.3 `[AGENT]` Add diagnostic to LSP
- [x] 0.3.1 `[AGENT]` Semantic tokens provider (prototype)
- [x] 0.3.2 `[HUMAN]` Verify semantic token styling in Zed
- [x] 0.3.3 `[AGENT]` Inlay hints provider (checkboxes)
- [x] 0.3.4 `[HUMAN]` Checkpoint: Strategy A+B confirmed

### Phase 1 (MVP) — COMPLETE ✓

- [x] 1.1.1 `[AGENT]` Tree-sitter grammar (`grammar.js` + generated `src/parser.c`, 24/24 corpus tests passing)
- [x] 1.1.2 `[AGENT]` Tree-sitter query files (highlights.scm, folds.scm, outline.scm, injections.scm)
- [x] 1.1.3 `[AGENT]` Integrate grammar into extension (grammar enabled in config.toml + extension.toml, generated files in src/)
- [x] 1.2.1 `[AGENT]` Vault indexer data structures + scanner + parser
- [x] 1.2.2 `[AGENT]` build_index + resolve_wikilink
- [x] 1.2.3 `[AGENT]` Integrate vault indexer into LSP
- [x] 1.3.1 `[AGENT]` Completion (wikilinks, trigger: `[`)
- [x] 1.4.1 `[AGENT]` Go-to-definition (wikilink target + anchor)
- [x] 1.5.1 `[AGENT]` Diagnostics (broken wikilinks)
- [x] 1.6.1 `[AGENT]` Hover (note title + snippet + tags + backlinks)
- [x] 1.7.1 `[AGENT]` Document symbols (hierarchical heading tree)
- [x] 1.8.1 `[AGENT]` Remove hello world code
- [x] 1.9.1 `[AGENT]` Create test vault in `tests/fixtures/vault/` (10 notes, covers wikilinks, tags, callouts, math, tables, broken links, frontmatter)
- [x] 1.9.2 `[HUMAN]` MVP integration test

### Phase 2 (Visual) — COMPLETE ✓

- [x] 2.1.1 `[AGENT]` Semantic tokens full (H1–H6, bold, italic, strikethrough, code, wikilink, tag, callout, checkbox, math, frontmatter)
- [x] 2.1.2 `[AGENT]` Semantic tokens delta (prefix/suffix diff; empty delta on unchanged)
- [x] 2.2.1 `[AGENT]` Companion theme (noted-theme/ — Verdant Garden dark + light, `syntax` rules for standard token types)
- [x] 2.2.2 `[HUMAN]` Visual verification of theme
- [x] 2.3.1 `[AGENT]` Code actions (toggle checkbox, wrap selection, heading level, insert callout/table, change callout type)
- [x] 2.4.1 `[AGENT]` Rename (prepareRename + rename wikilinks across files)
- [x] 2.5.1 `[AGENT]` Workspace symbols (search headings across all files)
- [x] 2.6.1 `[HUMAN]` Phase 2 integration test

### Phase 3 (Preview) — COMPLETE ✓

- [x] 3.1.1 `[AGENT]` HTTP server (axum, embedded in LSP)
- [x] 3.1.2 `[AGENT]` Preview HTML + WebSocket client
- [x] 3.2.1 `[AGENT]` MD → HTML renderer (pulldown-cmark + wikilinks + callouts + math)
- [x] 3.2.2 `[AGENT]` Preview CSS styles
- [x] 3.3.1 `[AGENT]` Live sync (didChange → WebSocket → browser)
- [x] 3.4.1 `[AGENT]` Preview command (code action + executeCommand)
- [x] 3.4.2 `[HUMAN]` Phase 3 testing

### Phase 4 (Publishing) — IN PROGRESS

- [x] 4.1.1 `[AGENT]` CI workflow (.github/workflows/ci.yml)
- [x] 4.1.2 `[AGENT]` Release workflow (.github/workflows/release.yml)
- [x] 4.2.1 `[AGENT]` Auto-download LSP binary in extension
- [x] 4.3.1 `[AGENT]` README.md (final)
- [x] 4.3.2 `[AGENT]` CHANGELOG.md
- [x] 4.3.3 `[AGENT]` Theme README.md
- [ ] 4.4.1 `[HUMAN]` Publish to Zed extension registry

## LSP Capabilities Checklist

### Phase 1 (MVP) — COMPLETE ✓

- [x] `initialize` / `initialized`
- [x] `textDocument/didOpen` / `didChange` / `didClose` / `didSave`
- [x] `textDocument/completion` (trigger: `[`, wikilink targets from vault index)
- [x] `textDocument/definition` (jump to wikilink target file)
- [x] `textDocument/publishDiagnostics` (broken wikilinks → ERROR)
- [x] `textDocument/hover` (note title + first paragraph + tags + backlink count)
- [x] `textDocument/documentSymbol` (hierarchical heading tree)

### Phase 2 (Visual) — COMPLETE ✓

- [x] `textDocument/semanticTokens/full` (H1–H6, bold, italic, strikethrough, code, wikilink, tag, callout, checkbox, math, frontmatter — mapped to standard LSP types: keyword, variable, string, comment, operator)
- [x] `textDocument/semanticTokens/full/delta` (prefix/suffix diff; empty delta on unchanged file)
- [x] `textDocument/codeAction` (toggle checkbox, wrap selection, heading level, insert callout/table, change callout type)
- [x] `textDocument/inlayHint` (checkboxes ✓/○)
- [x] `textDocument/rename` + `prepareRename`
- [x] `workspace/symbol`

### Phase 3 (Preview) — COMPLETE ✓

- [x] `workspace/executeCommand` (`noted.openPreview` — start preview server, return URL)
- [x] Preview HTTP server (axum, `127.0.0.1`, random port)
- [x] WebSocket live sync (`didChange` → render → broadcast)
- [x] MD → HTML renderer (pulldown-cmark + wikilinks + callouts + math + tables)

## Testing

### Unit tests (`cargo test -p noted-lsp`) — 142 tests, all passing

- `vault/parser.rs`: heading/wikilink/tag/frontmatter extraction (8 tests)
- `vault/index.rs`: build_index, resolve_wikilink (7 tests)
- `code_actions.rs`: checkbox toggle, wrap, heading level, callout change (14 tests)
- `completion.rs`: wikilink completion filtering and TextEdit ranges (6 tests)
- `definition.rs`: cursor-in-span detection, anchor/alias stripping (7 tests)
- `diagnostics.rs`: broken link detection, range computation (9 tests)
- `hover.rs`: snippet extraction, frontmatter skipping, hover output (14 tests)
- `symbols.rs`: heading tree building, nesting, range correctness (10 tests)
- `inlay_hints.rs`: checkbox hint positions (5 tests)
- `semantic_tokens.rs`: token positions, delta encoding, broken wikilinks (23 tests)

### Integration tests (planned)

- Test vault in `tests/fixtures/vault/` (not yet created)
- Verify: vault index builds correctly
- Verify: go-to-definition finds target files
- Verify: rename updates all references

### Manual testing

- Build: `cargo build --release -p noted-lsp`
- Set `NOTED_LSP_PATH` in shell profile
- Install as dev extension in Zed (`zed: install dev extension`)
- Open a directory with `.md` files
- Verify each LSP feature

## Test Vault (fixtures)

Location: `tests/fixtures/vault/`:

```
vault/
├── index.md              # Links to all other files
├── daily/
│   ├── 2026-04-04.md     # Daily note with tags and checkboxes
│   └── 2026-04-05.md
├── projects/
│   ├── zed-plugin.md     # Wikilinks, callouts, code blocks
│   └── rust-learning.md  # Headings H1-H6, math
├── people/
│   └── alice.md          # Frontmatter, backlinks
├── templates/
│   └── daily.md          # Template file
├── broken-links.md       # File with intentionally broken links
├── callouts-test.md      # All callout types
├── math-test.md          # Math blocks ($...$, $$...$$)
└── table-test.md         # Markdown tables
```

## Common Mistakes (avoid)

1. **Extension vs LSP confusion:** The extension (WASM) only registers the language and starts the LSP.
   All logic lives in the LSP binary. Do not put logic in `lib.rs`.

2. **Tree-sitter grammar regeneration:** After changing `grammar.js`, run
   `tree-sitter generate` + `tree-sitter test` in `grammars/noted/`.
   Commit updated `src/parser.c` and `src/tree_sitter/parser.h`.

3. **Tree-sitter conflict:** Zed already has a built-in `tree-sitter-markdown`.
   Our grammar registers as a separate language (`Noted Markdown`), not overwriting standard Markdown.

4. **Semantic tokens + Zed:** Zed only recognizes standard LSP semantic token types
   (keyword, variable, string, comment, operator, etc.). Custom type names are silently
   ignored — never use custom names in the legend. The LSP maps heading→keyword,
   markup→variable, punctuation→operator. Styling comes from the theme's `syntax`
   section. User must set `"semantic_tokens": "combined"` in `settings.json` under
   `languages > Noted Markdown` (default is `"off"`). `semantic_token_rules` in
   settings.json does NOT work for custom types. Theme JSON does NOT support
   `semantic_token_rules` at all.

5. **LSP binary distribution:** Zed extensions cannot include binaries.
   Dev mode: set `NOTED_LSP_PATH` env var. Phase 4: auto-download from GitHub Releases.

6. **wasm32-wasip2 limitations:** `std::env::var` does not work in the WASM sandbox.
   Use `worktree.shell_env()` to read environment variables (including `NOTED_LSP_PATH`).

7. **VaultIndex on startup:** The index is built asynchronously after `initialize`.
   Features that depend on it (completion, hover, definition, diagnostics) return
   empty/None if called before indexing completes. Diagnostics are republished after
   indexing finishes.

## References

- [Project Specification](./zed-md-wysiwyg-spec.md) (full version)
- [Zed Extension API docs](https://zed.dev/docs/extensions)
- [Zed Extension API crate](https://crates.io/crates/zed_extension_api)
- [tower-lsp docs](https://docs.rs/tower-lsp)
- [pulldown-cmark docs](https://docs.rs/pulldown-cmark)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- [Tree-sitter docs](https://tree-sitter.github.io/tree-sitter/)
- [Obsidian Flavored Markdown](https://help.obsidian.md/Editing+and+formatting/Obsidian+Flavored+Markdown)
