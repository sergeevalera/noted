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
structure. Includes an LSP server written in Rust, a custom Tree-sitter grammar
(in a separate repo), and a companion theme.

## Repositories

- `noted/` — main extension (extension + LSP + language queries)
- `tree-sitter-noted/` — Tree-sitter grammar (separate repo, cloned by Zed during install)
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
│  │  - register  │    │  + Theme styles       │   │
│  │    language  │    └───────────┬───────────┘   │
│  │  - start LSP │                │ LSP Protocol  │
│  └──────┬───────┘                │ (stdin/stdout)│
│         │ spawn                  │               │
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
│  │  │ Rename   │ │ Doc Symbols│ │ Preview   │  │ │
│  │  │          │ │            │ │ Server    │  │ │
│  │  └──────────┘ └────────────┘ └───────────┘  │ │
│  └─────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘
```

## Tech Stack

- **Extension:** Rust → `wasm32-wasip2`, `zed_extension_api` crate
- **LSP:** Rust, `tower-lsp` + `tokio`, `pulldown-cmark`, `walkdir`, `regex`, `camino`
- **Grammar:** JavaScript (`grammar.js`), Tree-sitter CLI — lives in `tree-sitter-noted` repo
- **Theme:** JSON (Zed Theme Schema v0.2.0)

## Project Structure

```
noted/
├── extension.toml              # Zed extension manifest (references tree-sitter-noted repo)
├── Cargo.toml                  # Workspace root + WASM extension crate
├── src/lib.rs                  # Extension entry: locate LSP binary via NOTED_LSP_PATH
├── languages/noted/
│   ├── config.toml             # Language config (grammar = "noted")
│   ├── highlights.scm          # Syntax highlighting queries
│   ├── injections.scm          # Code block language injection (disabled — see below)
│   ├── outline.scm             # Outline panel queries
│   ├── folds.scm               # Code folding queries
│   └── semantic_token_rules.json # Maps custom LSP token types to theme syntax keys
├── grammars/noted/             # ⚠ Managed by Zed — cloned from tree-sitter-noted repo
│                               # NOT tracked in git (.gitignore)
└── crates/noted-lsp/
    ├── Cargo.toml              # LSP server crate
    └── src/
        ├── main.rs             # Entry point, server state, LSP handler dispatch
        ├── code_actions.rs     # textDocument/codeAction (checkbox, wrap, heading, callout)
        ├── completion.rs       # textDocument/completion (wikilinks, trigger: `[`)
        ├── definition.rs       # textDocument/definition (go-to wikilink target)
        ├── diagnostics.rs      # publishDiagnostics (broken wikilinks)
        ├── hover.rs            # textDocument/hover (note title + snippet + metadata)
        ├── inlay_hints.rs      # textDocument/inlayHint (checkbox ✓/○ — not shown in Zed)
        ├── preview.rs          # HTTP + WebSocket preview server (axum)
        ├── render.rs           # MD → HTML renderer (pulldown-cmark + wikilinks + callouts)
        ├── rename.rs           # prepareRename + rename wikilinks across vault
        ├── semantic_tokens.rs  # textDocument/semanticTokens (headings, bold, italic, nested)
        ├── symbols.rs          # textDocument/documentSymbol (heading tree)
        ├── workspace_symbols.rs # workspace/symbol (not exposed in Zed UI)
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
- Linting: `cargo clippy -p noted-lsp -- -D warnings`
- Tests: `cargo test` must pass before every commit
- Error handling: use `anyhow` for LSP, `thiserror` for library errors
- Async: `tokio` runtime, `tower-lsp` async handlers
- Logging: `tracing` crate (`info!`, `warn!`, `error!`, `debug!`)
- Do not use `unwrap()` in production code — only in tests
- Prefer `&str` over `String` in function arguments
- Use `camino::Utf8PathBuf` instead of `std::path::PathBuf` for paths

### Tree-sitter

- Grammar source lives in the **`tree-sitter-noted`** repo (not in this repo)
- Every new grammar rule must be accompanied by a test in `test/corpus/`
- After changing `grammar.js` → run `tree-sitter generate` + `tree-sitter test`
  in the `tree-sitter-noted` repo
- Push changes to `tree-sitter-noted`, then update `extension.toml` with the new
  commit hash in the `rev` field
- The `grammars/noted/` directory in this repo is managed by Zed (git-ignored)

### Commits

- Format: `type(scope): description`
- Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`
- Examples: `feat(lsp): add wikilink completion`, `fix(grammar): handle empty callouts`

## Semantic Token Types (for LSP)

The LSP uses **custom token type names** mapped to theme syntax keys via
`languages/noted/semantic_token_rules.json`. This file tells Zed how to resolve
custom types to theme styles, with fallback chains for compatibility with non-Noted themes.

**LSP legend (custom token types):**

| LSP type (legend) | Used for | Rules file maps to (primary) |
|---|---|---|
| `heading` | headings (H1–H6) | `heading.h1`–`heading.h6` |
| `markup` | inline formatting (bold, italic, strikethrough, code, wikilink, tag, callout, checkbox, math) | `markup.bold`, `markup.italic`, etc. |
| `string` | links, link text | `string.link` |
| `comment` | frontmatter | `comment.frontmatter` |
| `punctuation` | MD syntax (`#`, `**`, `[[`, `]]`, etc.) | `punctuation.markup_punctuation` |

**Token modifiers** (used for both rules matching and delta caching):
`h1`–`h6`, `bold`, `italic`, `strikethrough`, `code`, `link`, `wikilink`, `broken`,
`tag`, `callout`, `checkbox_done`, `checkbox_todo`, `math`, `frontmatter`,
`markup_punctuation`

**How styling works:**
1. LSP emits tokens with custom type + modifier bitmask
2. `semantic_token_rules.json` matches type + modifiers → theme `syntax` key fallback chain
3. Theme's `syntax` section provides actual color/weight/style
4. Fallback keys ensure basic styling even with non-Noted themes

**Nested markup:** Formatting spans (bold, italic, strikethrough) recurse to handle
nesting (e.g., code inside bold, italic inside heading). Formatting modifiers inherit
from parent formatting context. Atomic elements (code, wikilinks, tags) use their own
styling without inheriting parent modifiers. Heading gaps use `TYPE_HEADING`.

**Position encoding:** All internal processing uses byte offsets (natural for Rust regex).
Before emitting tokens, byte positions are converted to UTF-16 column offsets using
`encode_utf16().count()` — required because LSP defaults to UTF-16 encoding.

Delta caching: per-document flat u32 token cache; unchanged files return empty delta.
Broken wikilinks flagged with `broken` modifier only after vault index is populated.

**User setup:** Add `"semantic_tokens": "combined"` to `settings.json` under
`languages > Noted Markdown` (default is `"off"`, which disables semantic tokens).
No `semantic_token_rules` in settings.json needed — the extension ships its own rules file.

## LSP Capabilities Checklist

### Working in Zed

- [x] `initialize` / `initialized`
- [x] `textDocument/didOpen` / `didChange` / `didClose` / `didSave`
- [x] `textDocument/completion` (trigger: `[`, wikilink targets from vault index)
- [x] `textDocument/definition` (jump to wikilink target file)
- [x] `textDocument/publishDiagnostics` (broken wikilinks → ERROR)
- [x] `textDocument/hover` (note title + first paragraph + tags + backlink count)
- [x] `textDocument/documentSymbol` (hierarchical heading tree)
- [x] `textDocument/semanticTokens/full` (with nested markup + UTF-16 encoding)
- [x] `textDocument/semanticTokens/full/delta` (prefix/suffix diff; empty on unchanged)
- [x] `textDocument/codeAction` (toggle checkbox, wrap, heading level, callout, table)
- [x] `textDocument/rename` + `prepareRename`
- [x] `workspace/executeCommand` (`noted.openPreview`, `noted.showLinks`, `noted.showTag`)
- [x] Preview HTTP server (axum, `127.0.0.1`, random port)
- [x] WebSocket live sync (`didChange` → render → broadcast)

### Implemented but not exposed in Zed

- [x] `textDocument/inlayHint` (checkboxes ✓/○ — Zed doesn't render for extensions)
- [x] `workspace/symbol` (heading search — Zed has no UI for this)

## Testing

### Unit tests (`cargo test -p noted-lsp`) — 150 tests, all passing

- `vault/parser.rs`: heading/wikilink/tag/frontmatter extraction (8 tests)
- `vault/index.rs`: build_index, resolve_wikilink (7 tests)
- `code_actions.rs`: checkbox toggle, wrap, heading level, callout change (14 tests)
- `completion.rs`: wikilink completion filtering and TextEdit ranges (6 tests)
- `definition.rs`: cursor-in-span detection, anchor/alias stripping (7 tests)
- `diagnostics.rs`: broken link detection, range computation (9 tests)
- `hover.rs`: snippet extraction, frontmatter skipping, hover output (14 tests)
- `symbols.rs`: heading tree building, nesting, range correctness (10 tests)
- `inlay_hints.rs`: checkbox hint positions (5 tests)
- `semantic_tokens.rs`: token positions, delta encoding, nested markup, UTF-16 (30 tests)
- `workspace_symbols.rs`: query filtering, symbol kinds (4 tests)
- `render.rs`: (included in hover tests)

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

2. **Tree-sitter grammar lives in `tree-sitter-noted` repo.** The `grammars/noted/`
   directory in this repo is managed by Zed (cloned from `tree-sitter-noted` during
   extension install). It is git-ignored. To change the grammar, edit `tree-sitter-noted`,
   push, and update `extension.toml` with the new commit hash.

3. **Tree-sitter query compatibility (0.26+):** Do NOT use `field: _` (wildcard) on
   fields that point to anonymous regex nodes (like `field("marker", /regex/)`). This
   causes "Impossible pattern" errors. Either query named child nodes directly or change
   the grammar to use named nodes for those fields.

4. **Tree-sitter conflict:** Zed already has a built-in `tree-sitter-markdown`.
   Our grammar registers as a separate language (`Noted Markdown`), not overwriting standard Markdown.

5. **Semantic tokens + Zed:** Custom LSP token type names require a
   `semantic_token_rules.json` file in the language directory to map them to theme
   `syntax` keys. Without this file, custom names are silently ignored. The extension
   ships `languages/noted/semantic_token_rules.json` with fallback chains for
   compatibility. User must set `"semantic_tokens": "combined"` in `settings.json`
   under `languages > Noted Markdown` (default is `"off"`). No user-level
   `semantic_token_rules` needed.

6. **UTF-16 position encoding:** LSP defaults to UTF-16 column offsets. All token
   positions and ranges must be converted from byte offsets using `encode_utf16().count()`.
   Byte offsets cause misaligned highlighting on lines with multi-byte characters
   (emoji, em dash, checkmarks, etc.).

7. **LSP binary distribution:** Zed extensions cannot include binaries.
   Dev mode: set `NOTED_LSP_PATH` env var. Phase 4: auto-download from GitHub Releases.

8. **wasm32-wasip2 limitations:** `std::env::var` does not work in the WASM sandbox.
   Use `worktree.shell_env()` to read environment variables (including `NOTED_LSP_PATH`).

9. **VaultIndex on startup:** The index is built asynchronously after `initialize`.
   Features that depend on it (completion, hover, definition, diagnostics) return
   empty/None if called before indexing completes. Diagnostics are republished after
   indexing finishes.

10. **Zed limitations for extensions:** Inlay hints and workspace symbols are implemented
    in the LSP but Zed does not expose them for extension languages. Keep the code
    (may work in future Zed versions) but do not advertise in README.

## References

- [Project Specification](./zed-md-wysiwyg-spec.md) (full version)
- [Zed Extension API docs](https://zed.dev/docs/extensions)
- [Zed Extension API crate](https://crates.io/crates/zed_extension_api)
- [tower-lsp docs](https://docs.rs/tower-lsp)
- [pulldown-cmark docs](https://docs.rs/pulldown-cmark)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- [Tree-sitter docs](https://tree-sitter.github.io/tree-sitter/)
- [Obsidian Flavored Markdown](https://help.obsidian.md/Editing+and+formatting/Obsidian+Flavored+Markdown)
