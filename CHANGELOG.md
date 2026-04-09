# Changelog

## 0.1.0 — Initial Release

### LSP Features

- **Wikilink completion** — type `[[` to autocomplete note names from vault index
- **Go-to-definition** — Cmd+click on wikilinks to jump to target file
- **Broken link diagnostics** — unresolved wikilinks flagged as errors
- **Hover preview** — note title, first paragraph, tags, and backlink count
- **Document symbols** — hierarchical heading tree (`Cmd+Shift+O`)
- **Workspace symbols** — search headings across all files (`Cmd+T`)
- **Rename** — F2 on wikilinks, updates all references across vault
- **Code actions** — toggle checkboxes, wrap in bold/italic/code/wikilink, heading level, insert callout/table, change callout type
- **Semantic tokens** — headings, bold, italic, strikethrough, code, wikilinks, tags, callouts, math, frontmatter (with delta encoding)
- **Inlay hints** — checkbox markers (`✓` / `○`)
- **Live preview** — browser preview with WebSocket live sync, callout styling, wikilink rendering

### Extension

- Zed extension (WASM) with auto-download of LSP binary from GitHub Releases
- Dev mode: set `NOTED_LSP_PATH` env var to use a local build
- Language registered as "Noted Markdown" for `.md` / `.markdown` files

### Grammar

- Tree-sitter grammar for wikilinks, embeds, tags, callouts, checkboxes, headings
- 24 corpus tests
- Highlight, fold, outline, and injection queries
