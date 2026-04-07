---
title: Zed Plugin
status: active
tags: [project, rust, zed]
---

# Zed Plugin

A Zed IDE extension for enhanced Markdown editing.

## Architecture

> [!note] Design Decision
> We use a separate LSP binary instead of embedding logic in the WASM extension.

The extension consists of:
- **WASM extension** (`lib.rs`) — registers language, launches LSP
- **LSP server** — handles all intelligence features
- **Tree-sitter grammar** — syntax parsing

## Features

See [[rust-learning]] for Rust patterns used here.

```rust
fn main() {
    println!("Hello from noted-lsp!");
}
```

> [!warning] Important
> Always run `cargo test` before committing.

## Team

- [[alice]] — contributor

#project #rust #zed
