# CLAUDE.md — Instructions for Claude Agent

> This file provides project context for Claude Code / Zed Agent.
> Read it before every working session on the project.

## Agent Rules

- **No subagents without permission.** Never use the Agent tool (subagents) without
  explicitly asking the user first. Do the work directly instead.
- **English only for project files.** All documentation, comments, and commit messages
  must be in English, even if the user communicates in Russian.

## Project

**noted** — a Zed IDE extension that turns the editor into an Obsidian-like
environment for Markdown. Includes a custom Tree-sitter grammar, an LSP server
written in Rust, and a companion theme.

## Repositories

- `noted/` — main extension (extension + LSP + grammar)
- `noted-theme/` — companion theme (separate extension, JSON-only)

## Architecture

```
┌─────────────────────────────────────────────────┐
│                    Zed IDE                       │
│                                                  │
│  ┌──────────────┐    ┌───────────────────────┐  │
│  │  Extension    │    │  Editor               │  │
│  │  (WASM)      │    │  + Tree-sitter grammar │  │
│  │  lib.rs      │    │  + Semantic tokens     │  │
│  │  - register  │    │  + Inlay hints         │  │
│  │    language   │    │  + Theme styles        │  │
│  │  - start LSP │    └───────────┬────────────┘  │
│  └──────┬───────┘                │ LSP Protocol  │
│         │ spawn                  │ (stdin/stdout) │
│         ▼                        ▼                │
│  ┌─────────────────────────────────────────────┐ │
│  │           noted-lsp (Rust binary)          │ │
│  │                                              │ │
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
│  │                                              │ │
│  │  ┌──────────────────────────────────────┐    │ │
│  │  │ Preview Server (Phase 3)             │    │ │
│  │  │ axum HTTP + tokio-tungstenite WS     │    │ │
│  │  └──────────────────────────────────────┘    │ │
│  └──────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘
```

## Tech Stack

- **Extension:** Rust → `wasm32-wasip2`, `zed_extension_api` crate
- **LSP:** Rust, `tower-lsp` + `tokio`, `pulldown-cmark`, `walkdir`, `notify`, `serde`
- **Grammar:** JavaScript (`grammar.js`), Tree-sitter CLI
- **Theme:** JSON (Zed Theme Schema v0.2.0)

## Project Structure

```
noted/
├── extension.toml              # Zed extension manifest
├── Cargo.toml                  # WASM extension crate
├── src/lib.rs                  # Extension entry: register language, start LSP
├── languages/obsidian-md/
│   ├── config.toml             # Language config (file types, etc.)
│   ├── highlights.scm          # Syntax highlighting queries
│   ├── injections.scm          # Code block language injection
│   ├── outline.scm             # Outline panel queries
│   └── folds.scm               # Code folding queries
├── grammars/tree-sitter-obsidian-md/
│   ├── grammar.js              # Tree-sitter grammar definition
│   ├── src/                    # Generated C parser (tree-sitter generate)
│   └── test/corpus/            # Tree-sitter test cases
└── lsp/
    ├── Cargo.toml              # LSP server crate
    └── src/
        ├── main.rs             # Entry point, tower-lsp setup
        ├── state.rs            # Server state, vault index
        ├── indexer.rs          # Vault scanning, file parsing
        ├── completion.rs       # textDocument/completion (wikilinks)
        ├── definition.rs       # textDocument/definition (go-to-def)
        ├── diagnostics.rs      # publishDiagnostics (broken links)
        ├── hover.rs            # textDocument/hover (note preview)
        ├── semantic_tokens.rs  # textDocument/semanticTokens
        ├── code_actions.rs     # textDocument/codeAction
        ├── inlay_hints.rs      # textDocument/inlayHint
        ├── rename.rs           # textDocument/rename
        ├── symbols.rs          # textDocument/documentSymbol + workspace/symbol
        └── utils.rs            # Shared utilities
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

### Commits

- Format: `type(scope): description`
- Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`
- Examples: `feat(lsp): add wikilink completion`, `fix(grammar): handle empty callouts`

## Semantic Token Types (for LSP)

The LSP must register the following custom token types and modifiers:

**Token types:** `heading`, `markup`, `string`, `comment`, `punctuation`

**Token modifiers:** `h1`, `h2`, `h3`, `h4`, `h5`, `h6`, `bold`, `italic`,
`strikethrough`, `code`, `link`, `wikilink`, `broken`, `tag`, `callout`,
`checkbox`, `done`, `todo`, `math`, `frontmatter`, `markup`

Full table — see spec, section B.1.

## LSP Capabilities Checklist

Implement in this order (by phase):

### Phase 1 (MVP)
- [x] `initialize` / `initialized`
- [ ] `textDocument/didOpen` / `didChange` / `didClose`
- [ ] `textDocument/completion` (trigger: `[`)
- [ ] `textDocument/definition`
- [ ] `textDocument/publishDiagnostics`
- [ ] `textDocument/hover`
- [ ] `textDocument/documentSymbol`

### Phase 2 (Visual)
- [ ] `textDocument/semanticTokens/full`
- [ ] `textDocument/semanticTokens/full/delta`
- [ ] `textDocument/codeAction`
- [ ] `textDocument/inlayHint`
- [ ] `textDocument/rename` + `prepareRename`
- [ ] `workspace/symbol`

### Phase 3 (Preview)
- [ ] Custom notification: `mdlive/previewUrl` (send preview server URL)
- [ ] Custom notification: `mdlive/cursorPosition` (cursor sync)

## Testing

### Unit tests (cargo test)
- `indexer.rs`: MD file parsing, heading/link/tag extraction
- `completion.rs`: filtering and ranking completion items
- `diagnostics.rs`: broken link detection
- `semantic_tokens.rs`: correct token type markup

### Integration tests
- Test vault in `tests/fixtures/vault/` (20+ .md files)
- Verify: vault index builds correctly
- Verify: go-to-definition finds target files
- Verify: rename updates all references

### Manual testing
- Install as dev extension in Zed
- Open test vault
- Verify each LSP feature manually

## Test Vault (fixtures)

Create in `tests/fixtures/vault/`:

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

2. **Tree-sitter conflict:** Zed already has a built-in `tree-sitter-markdown`.
   Our grammar must register as a separate language (e.g. `obsidian-md`),
   not overwrite the standard Markdown. File association: `.md` files.

3. **Semantic tokens + theme:** Semantic tokens without proper `semantic_token_rules`
   in settings will have no visual effect. The extension documentation must include
   recommended rules. The companion theme is a separate extension.

4. **LSP binary distribution:** Zed extensions cannot include binaries.
   The LSP needs to be downloaded on install (via `download_file` API) or checked
   for in the system. In dev mode — specify an absolute path to the compiled binary.

5. **wasm32-wasip2 limitations:** The WASM extension cannot make network requests,
   file operations (except through Zed API), or launch processes without capability.
   All of that is handled by the LSP (a regular native binary, no restrictions).

## References

- [Project Specification](./zed-md-wysiwyg-spec.md) (full version)
- [Zed Extension API docs](https://zed.dev/docs/extensions)
- [Zed Extension API crate](https://crates.io/crates/zed_extension_api)
- [tower-lsp docs](https://docs.rs/tower-lsp)
- [pulldown-cmark docs](https://docs.rs/pulldown-cmark)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- [Tree-sitter docs](https://tree-sitter.github.io/tree-sitter/)
- [Obsidian Flavored Markdown](https://help.obsidian.md/Editing+and+formatting/Obsidian+Flavored+Markdown)
