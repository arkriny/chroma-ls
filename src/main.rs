mod color;
mod document;
mod lsp;

use document::Document;
use lsp::*;

use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::io;

fn read_message(stdin: &mut impl io::BufRead) -> Option<Message> {
    // Read Content-Length header
    let mut line = String::new();
    stdin.read_line(&mut line).ok()?;
    let content_length: usize = line.strip_prefix("Content-Length: ")?.trim().parse().ok()?;
    // Skip \r\n
    line.clear();
    stdin.read_line(&mut line).ok()?;
    // Read body
    let mut body = vec![0u8; content_length];
    stdin.read_exact(&mut body).ok()?;
    serde_json::from_slice(&body).ok()
}

fn write_message<T: Serialize>(stdout: &mut impl io::Write, resp: &Response<T>) {
    let body = serde_json::to_string(&resp).unwrap();
    print!("Content-Length: {}\r\n\r\n{}", body.len(), body);
    stdout.flush().unwrap();
}

fn main() {
    let mut documents: HashMap<String, Document> = HashMap::new();
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    while let Some(msg) = read_message(&mut stdin) {
        let method = msg.method.as_str();
        match method {
            "initialize" => {
                write_message(
                    &mut stdout,
                    &Response {
                        jsonrpc: "2.0",
                        id: msg.id.unwrap(),
                        result: json!({
                            "capabilities": {
                                "textDocumentSync": 2, // Incremental
                                "colorProvider": true,
                            },
                            "serverInfo": {
                                "name": "chroma-ls",
                                "version": env!("CARGO_PKG_VERSION").to_string(),
                            }
                        }),
                    },
                );
            }
            "initialized" => {
                eprintln!("initialized");
            }
            "shutdown" => write_message(
                &mut stdout,
                &Response {
                    jsonrpc: "2.0",
                    id: msg.id.unwrap(),
                    result: serde_json::Value::Null,
                },
            ),
            "textDocument/didOpen" => {
                let params: DidOpenTextDocumentParams = serde_json::from_value(msg.params).unwrap();
                let uri = params.text_document.uri;
                let text = params.text_document.text;
                documents.insert(uri, Document::from(text.as_str()));
            }
            "textDocument/didChange" => {
                let params: DidChangeTextDocumentParams =
                    serde_json::from_value(msg.params).unwrap();
                let uri = params.text_document.uri;

                let doc = documents.entry(uri).or_insert_with(|| Document::from(""));

                for change in params.content_changes {
                    doc.edit(&change);
                }
            }
            "textDocument/didClose" => {
                let params: DidCloseTextDocumentParams =
                    serde_json::from_value(msg.params).unwrap();
                let uri = params.text_document.uri;
                documents.remove(&uri);
            }
            "textDocument/documentColor" => {
                let params: DocumentColorParams = serde_json::from_value(msg.params).unwrap();
                let uri = params.text_document.uri;
                let colors: Vec<ColorInformation> = documents
                    .get(&uri)
                    .map(|doc| doc.get_colors())
                    .unwrap_or_default();

                write_message(
                    &mut stdout,
                    &Response {
                        jsonrpc: "2.0",
                        id: msg.id.unwrap(),
                        result: colors,
                    },
                );
            }
            "exit" => return,
            _ => {
                eprintln!("unsupported method: {method}");
            }
        }
    }
}
