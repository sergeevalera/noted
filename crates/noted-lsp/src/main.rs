mod code_actions;
mod completion;
mod definition;
mod diagnostics;
mod hover;
mod inlay_hints;
mod preview;
mod rename;
mod render;
mod semantic_tokens;
mod symbols;
mod vault;
mod workspace_symbols;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use camino::Utf8PathBuf;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use code_actions::compute_code_actions;
use completion::compute_completions;
use definition::find_definition;
use diagnostics::compute_diagnostics;
use hover::compute_hover;
use inlay_hints::compute_inlay_hints;
use preview::{start_preview_server, PreviewState};
use rename::{compute_rename, prepare_rename};
use render::render_markdown;
use semantic_tokens::{compute_semantic_tokens, compute_token_delta, tokens_to_flat};
use symbols::compute_document_symbols;
use vault::{build_index, parse_note, scan_vault, VaultIndex};
use workspace_symbols::compute_workspace_symbols;

struct NotedLsp {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, String>>>,
    index: Arc<RwLock<VaultIndex>>,
    vault_root: Arc<RwLock<Option<Utf8PathBuf>>>,
    /// Cached flat token data (5 u32 per token) for semantic tokens delta.
    token_cache: Arc<RwLock<HashMap<Url, Vec<u32>>>>,
    /// Preview server state (started lazily on first preview request).
    preview: PreviewState,
    /// URI of the document currently being previewed (None = preview inactive).
    preview_uri: Arc<RwLock<Option<Url>>>,
    /// Address of the preview server (None = not started yet).
    preview_addr: Arc<RwLock<Option<std::net::SocketAddr>>>,
    result_counter: Arc<AtomicU64>,
}

impl NotedLsp {
    fn next_result_id(&self) -> String {
        self.result_counter
            .fetch_add(1, Ordering::Relaxed)
            .to_string()
    }
}

impl NotedLsp {
    /// If the preview is active and the given URI matches, re-render and push to browser.
    async fn maybe_update_preview(&self, uri: &Url, text: &str) {
        let preview_uri = self.preview_uri.read().await;
        if preview_uri.as_ref() != Some(uri) {
            return;
        }
        drop(preview_uri);
        let html = render_markdown(text);
        self.preview.update(html).await;
    }

    /// Start the preview server (if not already running) and set the preview URI.
    /// Returns the preview URL.
    async fn start_preview(&self, uri: Url) -> String {
        // Start server if needed
        let mut addr_lock = self.preview_addr.write().await;
        let addr = if let Some(addr) = *addr_lock {
            addr
        } else {
            let addr = start_preview_server(self.preview.clone())
                .await
                .expect("failed to start preview server");
            *addr_lock = Some(addr);
            addr
        };
        drop(addr_lock);

        // Set the preview URI
        *self.preview_uri.write().await = Some(uri.clone());

        // Push initial content
        let documents = self.documents.read().await;
        if let Some(text) = documents.get(&uri) {
            let html = render_markdown(text);
            self.preview.update(html).await;
        }

        format!("http://{}", addr)
    }

    /// Publish broken-link diagnostics for a single document.
    async fn publish_diagnostics_for(&self, uri: Url, text: &str) {
        let index = self.index.read().await;
        let diags = compute_diagnostics(text, &index);
        drop(index);
        self.client.publish_diagnostics(uri, diags, None).await;
    }

    /// Scan and index the vault, then republish diagnostics for all open documents.
    async fn reindex_and_republish(
        root: Utf8PathBuf,
        index: Arc<RwLock<VaultIndex>>,
        documents: Arc<RwLock<HashMap<Url, String>>>,
        client: Client,
    ) {
        let t0 = Instant::now();
        let paths = scan_vault(&root);
        let notes = paths
            .iter()
            .filter_map(|p| std::fs::read_to_string(p).ok().map(|c| parse_note(p, &c)))
            .collect();
        let vault_index = build_index(notes);
        let n = vault_index.notes.len();
        *index.write().await = vault_index;
        tracing::info!(
            "Vault indexed: {} notes in {:.1}ms",
            n,
            t0.elapsed().as_secs_f64() * 1000.0
        );
        client
            .log_message(MessageType::INFO, format!("Vault indexed: {} notes", n))
            .await;

        // Republish diagnostics for every currently-open document
        let docs = documents.read().await;
        let idx = index.read().await;
        for (uri, text) in docs.iter() {
            let diags = compute_diagnostics(text, &idx);
            client.publish_diagnostics(uri.clone(), diags, None).await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for NotedLsp {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let has_sem = params
            .capabilities
            .text_document
            .as_ref()
            .and_then(|td| td.semantic_tokens.as_ref())
            .is_some();
        tracing::info!("Client semantic token support: {}", has_sem);

        let root = params
            .root_uri
            .as_ref()
            .and_then(|uri| uri.to_file_path().ok())
            .and_then(|p| Utf8PathBuf::from_path_buf(p).ok());

        if let Some(root) = root {
            *self.vault_root.write().await = Some(root.clone());
            let index = self.index.clone();
            let documents = self.documents.clone();
            let client = self.client.clone();
            tokio::spawn(async move {
                Self::reindex_and_republish(root, index, documents, client).await;
            });
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(false),
                        })),
                        ..Default::default()
                    },
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["[".to_string()]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: semantic_tokens::legend(),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            range: None,
                            work_done_progress_options: Default::default(),
                        },
                    ),
                ),
                inlay_hint_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["noted.openPreview".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("noted-lsp started");
        self.client
            .log_message(
                MessageType::INFO,
                "noted-lsp started (semantic tokens enabled)",
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text.clone();
        self.documents
            .write()
            .await
            .insert(uri.clone(), text.clone());
        self.publish_diagnostics_for(uri.clone(), &text).await;
        self.maybe_update_preview(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().last() {
            let text = change.text.clone();
            self.documents
                .write()
                .await
                .insert(uri.clone(), text.clone());
            self.publish_diagnostics_for(uri.clone(), &text).await;
            self.maybe_update_preview(&uri, &text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let root = self.vault_root.read().await.clone();
        let Some(root) = root else { return };
        let index = self.index.clone();
        let documents = self.documents.clone();
        let client = self.client.clone();
        let uri = params.text_document.uri.clone();
        tokio::spawn(async move {
            tracing::info!("Reindexing after save: {}", uri);
            Self::reindex_and_republish(root, index, documents, client).await;
        });
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.write().await.remove(&uri);
        self.token_cache.write().await.remove(&uri);
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.read().await;
        let text = match documents.get(uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let line_text = text.lines().nth(position.line as usize).unwrap_or("");
        let index = self.index.read().await;
        Ok(compute_hover(line_text, position.character, &index))
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let documents = self.documents.read().await;
        let text = match documents.get(uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let line_text = text.lines().nth(position.line as usize).unwrap_or("");
        let index = self.index.read().await;
        let items = compute_completions(position.line, line_text, position.character, &index);
        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(CompletionResponse::Array(items)))
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let documents = self.documents.read().await;
        let text = match documents.get(uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let line_text = text.lines().nth(position.line as usize).unwrap_or("");
        let index = self.index.read().await;
        Ok(
            find_definition(line_text, position.line, position.character, &index)
                .map(|link| GotoDefinitionResponse::Link(vec![link])),
        )
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let documents = self.documents.read().await;
        let text = match documents.get(&params.text_document.uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let syms = compute_document_symbols(&text);
        if syms.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DocumentSymbolResponse::Nested(syms)))
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let documents = self.documents.read().await;
        let text = match documents.get(&uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let index = self.index.read().await;
        let tokens = compute_semantic_tokens(&text, &index);
        drop(index);
        self.client
            .log_message(
                MessageType::INFO,
                format!("semantic_tokens_full: {} tokens", tokens.len()),
            )
            .await;
        let flat = tokens_to_flat(&tokens);
        let result_id = self.next_result_id();
        self.token_cache.write().await.insert(uri, flat);
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: Some(result_id),
            data: tokens,
        })))
    }

    async fn semantic_tokens_full_delta(
        &self,
        params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        let uri = params.text_document.uri;
        let documents = self.documents.read().await;
        let text = match documents.get(&uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let index = self.index.read().await;
        let new_tokens = compute_semantic_tokens(&text, &index);
        drop(index);
        let new_flat = tokens_to_flat(&new_tokens);
        let result_id = self.next_result_id();
        let mut cache = self.token_cache.write().await;
        let old_flat = cache.get(&uri).cloned().unwrap_or_default();
        let edits = compute_token_delta(&old_flat, &new_flat);
        cache.insert(uri, new_flat);
        Ok(Some(SemanticTokensFullDeltaResult::TokensDelta(
            SemanticTokensDelta {
                result_id: Some(result_id),
                edits,
            },
        )))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let documents = self.documents.read().await;
        let text = match documents.get(&params.text_document.uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        Ok(Some(compute_inlay_hints(&text)))
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;
        let documents = self.documents.read().await;
        let text = match documents.get(&uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let mut actions = compute_code_actions(&uri, params.range, &text);

        // Add "Open Preview" command action
        actions.push(CodeActionOrCommand::Command(Command {
            title: "Open Preview".to_string(),
            command: "noted.openPreview".to_string(),
            arguments: Some(vec![serde_json::Value::String(uri.to_string())]),
        }));

        Ok(Some(actions))
    }

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> Result<Option<serde_json::Value>> {
        if params.command == "noted.openPreview" {
            // The first argument is the document URI
            let uri = params
                .arguments
                .first()
                .and_then(|v| v.as_str())
                .and_then(|s| Url::parse(s).ok());

            let Some(uri) = uri else {
                self.client
                    .show_message(MessageType::ERROR, "No document URI provided")
                    .await;
                return Ok(None);
            };

            let url = self.start_preview(uri).await;
            self.client
                .show_message(MessageType::INFO, format!("Preview: {}", url))
                .await;
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("Preview server started at {}", url),
                )
                .await;

            return Ok(Some(serde_json::Value::String(url)));
        }

        Ok(None)
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        let uri = &params.text_document.uri;
        let position = params.position;
        let documents = self.documents.read().await;
        let text = match documents.get(uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let line_text = text.lines().nth(position.line as usize).unwrap_or("");
        Ok(prepare_rename(line_text, position.line, position.character))
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let index = self.index.read().await;
        let symbols = compute_workspace_symbols(&params.query, &index);
        if symbols.is_empty() {
            Ok(None)
        } else {
            Ok(Some(symbols))
        }
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let documents = self.documents.read().await;
        let text = match documents.get(uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        drop(documents);
        let line_text = text.lines().nth(position.line as usize).unwrap_or("");
        let index = self.index.read().await;
        Ok(compute_rename(
            line_text,
            position.character,
            &params.new_name,
            &index,
        ))
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
        index: Arc::new(RwLock::new(VaultIndex::default())),
        vault_root: Arc::new(RwLock::new(None)),
        token_cache: Arc::new(RwLock::new(HashMap::new())),
        result_counter: Arc::new(AtomicU64::new(0)),
        preview: PreviewState::new(),
        preview_uri: Arc::new(RwLock::new(None)),
        preview_addr: Arc::new(RwLock::new(None)),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
