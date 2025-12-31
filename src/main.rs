mod color;
mod document;
use document::Document;

use std::collections::HashMap;
use tokio::sync::RwLock;

use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{LanguageServer, LspService, Server};

struct Backend {
    documents: RwLock<HashMap<Uri, Document>>,
}

impl Backend {
    fn new() -> Self {
        Self {
            documents: RwLock::new(HashMap::new()),
        }
    }
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        ..Default::default()
                    },
                )),
                color_provider: Some(ColorProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "chroma-ls".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;

        self.documents
            .write()
            .await
            .insert(uri, Document::from(content.as_str()));
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut docs = self.documents.write().await;

        // Get or create the document
        let doc = docs
            .entry(uri.clone())
            .or_insert_with(|| Document::from(""));

        // Apply all changes in order
        for change in params.content_changes {
            doc.edit(&change);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.write().await.remove(&uri);
    }

    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        Ok(self
            .documents
            .read()
            .await
            .get(&params.text_document.uri)
            .map(|doc| doc.get_colors())
            .unwrap_or_default())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|_| Backend::new());
    Server::new(stdin, stdout, socket).serve(service).await;
}
