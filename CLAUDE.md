# CLAUDE.md — Instructions for Claude Agent

> This file provides project context for Claude Code / Zed Agent.
> Read it before every working session on the project.

## Agent Rules

- **No subagents without permission.** Never use the Agent tool (subagents) without
  explicitly asking the user first. Do the work directly instead.
- **English only for project files.** All documentation, comments, and commit messages
  must be in English, even if the user communicates in Russian.

## Project

**noted** — a Zed IDE extension that turns Zed into a knowledge-base editor for
Markdown. Supports wikilinks, callouts, tags, smart navigation, and live preview.
Compatible with Obsidian vaults. Includes a custom Tree-sitter grammar, an LSP
server written in Rust, and a companion theme.

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
│   ├── config.toml             # Language config (grammar disabled until tree-sitter generate)
│   ├── highlights.scm          # Syntax highlighting queries (inactive without grammar)
│   ├── injections.scm          # Code block language injection
│   ├── outline.scm             # Outline panel queries
│   └── folds.scm               # Code folding queries
├── grammars/tree-sitter-noted/
│   ├── grammar.js              # Tree-sitter grammar definition
│   ├── src/                    # Generated C parser — MISSING, run: tree-sitter generate
│   └── test/corpus/            # Tree-sitter test cases
└── crates/noted-lsp/
    ├── Cargo.toml              # LSP server crate
    └── src/
        ├── main.rs             # Entry point, server state, LSP handler dispatch
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

Currently implemented (Phase 2 partial):

**Token types:** `heading`, `markup`, `punctuation`

**Token modifiers:** `h1`, `h2`, `h3`, `bold`, `italic`

Full target set (Phase 2 complete) — see spec section B.1:

**Token types:** `heading`, `markup`, `string`, `comment`, `punctuation`

**Token modifiers:** `h1`–`h6`, `bold`, `italic`, `strikethrough`, `code`, `link`,
`wikilink`, `broken`, `tag`, `callout`, `checkbox`, `done`, `todo`, `math`,
`frontmatter`, `markup`

Note: semantic token visual styling is set at the **theme level** only
(in the companion theme extension). `semantic_token_rules` in `settings.json`
is not supported by Zed.

## LSP Capabilities Checklist

### Phase 1 (MVP) — COMPLETE ✓

- [x] `initialize` / `initialized`
- [x] `textDocument/didOpen` / `didChange` / `didClose` / `didSave`
- [x] `textDocument/completion` (trigger: `[`, wikilink targets from vault index)
- [x] `textDocument/definition` (jump to wikilink target file)
- [x] `textDocument/publishDiagnostics` (broken wikilinks → ERROR)
- [x] `textDocument/hover` (note title + first paragraph + tags + backlink count)
- [x] `textDocument/documentSymbol` (hierarchical heading tree)

### Phase 2 (Visual) — in progress

- [x] `textDocument/semanticTokens/full` (H1–H3, bold, italic — partial)
- [ ] `textDocument/semanticTokens/full/delta`
- [ ] `textDocument/codeAction`
- [x] `textDocument/inlayHint` (checkboxes ✓/○ — partial)
- [ ] `textDocument/rename` + `prepareRename`
- [ ] `workspace/symbol`

### Phase 3 (Preview)

- [ ] Custom notification: `mdlive/previewUrl` (send preview server URL)
- [ ] Custom notification: `mdlive/cursorPosition` (cursor sync)

## Testing

### Unit tests (`cargo test -p noted-lsp`) — 67 tests, all passing

- `vault/parser.rs`: heading/wikilink/tag/frontmatter extraction (8 tests)
- `vault/index.rs`: build_index, resolve_wikilink (7 tests)
- `completion.rs`: wikilink completion filtering and TextEdit ranges (6 tests)
- `definition.rs`: cursor-in-span detection, anchor/alias stripping (7 tests)
- `diagnostics.rs`: broken link detection, range computation (9 tests)
- `hover.rs`: snippet extraction, frontmatter skipping, hover output (14 tests)
- `symbols.rs`: heading tree building, nesting, range correctness (10 tests)
- `inlay_hints.rs`: checkbox hint positions (5 tests)
- `semantic_tokens.rs`: (no unit tests yet)

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

## Test Vault (fixtures — not yet created)

Planned location `tests/fixtures/vault/`:

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

2. **Tree-sitter grammar not generated:** `grammars/tree-sitter-noted/src/` must contain
   `parser.c` (generated by `tree-sitter generate`) for Zed to compile the grammar. Without it,
   the language fails to load. Currently the grammar is disabled in `config.toml` / `extension.toml`
   as a workaround until `tree-sitter-cli` is available.

3. **Tree-sitter conflict:** Zed already has a built-in `tree-sitter-markdown`.
   Our grammar registers as a separate language (`Noted Markdown`), not overwriting standard Markdown.

4. **Semantic tokens + theme:** Semantic tokens without `semantic_token_rules` in the companion
   theme have no visual effect. `semantic_token_rules` in `settings.json` is not supported by Zed.
   Visual styling requires the companion theme extension (Phase 2).

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
