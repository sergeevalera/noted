use std::collections::HashMap;
use std::sync::Arc;

use regex::Regex;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

// Token type indices (must match the legend order)
const TYPE_HEADING: u32 = 0;
const TYPE_MARKUP: u32 = 1;
const TYPE_PUNCTUATION: u32 = 2;

// Token modifier bits (1 << index in legend)
const MOD_H1: u32 = 1 << 0;
const MOD_H2: u32 = 1 << 1;
const MOD_H3: u32 = 1 << 2;
const MOD_BOLD: u32 = 1 << 3;
const MOD_ITALIC: u32 = 1 << 4;

struct NotedLsp {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, String>>>,
}

impl NotedLsp {
    async fn publish_connected_diagnostic(&self, uri: Url) {
        let diagnostic = Diagnostic {
            range: Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 0, character: 0 },
            },
            severity: Some(DiagnosticSeverity::INFORMATION),
            message: "noted-lsp is connected and working".to_string(),
            ..Default::default()
        };
        self.client
            .publish_diagnostics(uri, vec![diagnostic], None)
            .await;
    }
}

/// Compute semantic tokens for the given text.
/// Returns tokens sorted and encoded in the LSP delta format.
fn compute_semantic_tokens(text: &str) -> Vec<SemanticToken> {
    let bold_re = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    let italic_re = Regex::new(r"\*([^*\n]+?)\*").unwrap();

    // Collect raw tokens as (line, start_char, length, type, modifiers)
    let mut raw: Vec<(u32, u32, u32, u32, u32)> = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let ln = line_idx as u32;

        // Headings — handle before inline markup
        if line.starts_with("### ") {
            raw.push((ln, 0, 3, TYPE_PUNCTUATION, 0));
            let len = line.len().saturating_sub(4) as u32;
            if len > 0 {
                raw.push((ln, 4, len, TYPE_HEADING, MOD_H3));
            }
            continue;
        } else if line.starts_with("## ") {
            raw.push((ln, 0, 2, TYPE_PUNCTUATION, 0));
            let len = line.len().saturating_sub(3) as u32;
            if len > 0 {
                raw.push((ln, 3, len, TYPE_HEADING, MOD_H2));
            }
            continue;
        } else if line.starts_with("# ") {
            raw.push((ln, 0, 1, TYPE_PUNCTUATION, 0));
            let len = line.len().saturating_sub(2) as u32;
            if len > 0 {
                raw.push((ln, 2, len, TYPE_HEADING, MOD_H1));
            }
            continue;
        }

        // Track covered byte ranges to avoid italic matching inside bold
        let mut covered = vec![false; line.len()];

        // Bold: **content**
        for cap in bold_re.captures_iter(line) {
            let full = cap.get(0).unwrap();
            let content = cap.get(1).unwrap();
            for i in full.start()..full.end() {
                covered[i] = true;
            }
            raw.push((ln, full.start() as u32, 2, TYPE_PUNCTUATION, 0));
            raw.push((ln, content.start() as u32, content.len() as u32, TYPE_MARKUP, MOD_BOLD));
            raw.push((ln, content.end() as u32, 2, TYPE_PUNCTUATION, 0));
        }

        // Italic: *content* (skip positions already covered by bold)
        for cap in italic_re.captures_iter(line) {
            let full = cap.get(0).unwrap();
            if covered[full.start()] {
                continue;
            }
            let content = cap.get(1).unwrap();
            raw.push((ln, full.start() as u32, 1, TYPE_PUNCTUATION, 0));
            raw.push((ln, content.start() as u32, content.len() as u32, TYPE_MARKUP, MOD_ITALIC));
            raw.push((ln, content.end() as u32, 1, TYPE_PUNCTUATION, 0));
        }
    }

    // Sort by line then by start position
    raw.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    // Encode as LSP delta format
    let mut tokens = Vec::with_capacity(raw.len());
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;

    for (line, start, length, token_type, modifiers) in raw {
        let delta_line = line - prev_line;
        let delta_start = if delta_line == 0 { start - prev_start } else { start };
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

/// Returns the character position after `]` if the line contains a checkbox.
/// `done = true`  → looks for `- [x]`
/// `done = false` → looks for `- [ ]`
fn checkbox_hint_col(line: &str, done: bool) -> Option<u32> {
    let pattern = if done { "- [x]" } else { "- [ ]" };
    let byte_pos = line.find(pattern)?;
    // position after the closing `]`
    let char_pos = (byte_pos + pattern.len()) as u32;
    Some(char_pos)
}

#[tower_lsp::async_trait]
impl LanguageServer for NotedLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: vec![
                                    SemanticTokenType::new("heading"),
                                    SemanticTokenType::new("markup"),
                                    SemanticTokenType::new("punctuation"),
                                ],
                                token_modifiers: vec![
                                    SemanticTokenModifier::new("h1"),
                                    SemanticTokenModifier::new("h2"),
                                    SemanticTokenModifier::new("h3"),
                                    SemanticTokenModifier::new("bold"),
                                    SemanticTokenModifier::new("italic"),
                                ],
                            },
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            range: None,
                            work_done_progress_options: Default::default(),
                        },
                    ),
                ),
                inlay_hint_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("noted-lsp started successfully");
        self.client
            .log_message(MessageType::INFO, "noted-lsp started successfully")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        self.documents
            .write()
            .await
            .insert(uri.clone(), params.text_document.text);
        self.publish_connected_diagnostic(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().last() {
            self.documents.write().await.insert(uri.clone(), change.text);
        }
        self.publish_connected_diagnostic(uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.write().await.remove(&params.text_document.uri);
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let documents = self.documents.read().await;
        let text = match documents.get(&params.text_document.uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);

        let data = compute_semantic_tokens(&text);
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data,
        })))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let documents = self.documents.read().await;
        let text = match documents.get(&params.text_document.uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);

        let mut hints = Vec::new();
        for (line_idx, line) in text.lines().enumerate() {
            let ln = line_idx as u32;
            if let Some(col) = checkbox_hint_col(line, true) {
                hints.push(InlayHint {
                    position: Position { line: ln, character: col },
                    label: InlayHintLabel::String(" ✓".to_string()),
                    kind: Some(InlayHintKind::PARAMETER),
                    text_edits: None,
                    tooltip: None,
                    padding_left: None,
                    padding_right: None,
                    data: None,
                });
            } else if let Some(col) = checkbox_hint_col(line, false) {
                hints.push(InlayHint {
                    position: Position { line: ln, character: col },
                    label: InlayHintLabel::String(" ○".to_string()),
                    kind: Some(InlayHintKind::PARAMETER),
                    text_edits: None,
                    tooltip: None,
                    padding_left: None,
                    padding_right: None,
                    data: None,
                });
            }
        }

        Ok(Some(hints))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Hello from noted LSP!".to_string(),
            }),
            range: None,
        }))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| NotedLsp {
        client,
        documents: Arc::new(RwLock::new(HashMap::new())),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
