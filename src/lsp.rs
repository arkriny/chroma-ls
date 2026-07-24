use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct Message {
    pub id: Option<Value>,
    pub method: String,
    pub params: Value,
}

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub jsonrpc: &'static str,
    pub id: Value,
    pub result: T,
}

#[derive(Deserialize)]
pub struct DidChangeTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[serde(rename = "contentChanges")]
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Deserialize)]
pub struct DocumentColorParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}

#[derive(Deserialize)]
pub struct DidCloseTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}

#[derive(Deserialize)]
pub struct DidOpenTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocument,
}

#[derive(Deserialize, Serialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Deserialize, Serialize)]
pub struct TextDocument {
    pub uri: String,
    pub text: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Deserialize)]
pub struct TextDocumentContentChangeEvent {
    pub range: Option<Range>,
    pub text: String,
}

#[derive(Clone, Serialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(Clone, Serialize)]
pub struct ColorInformation {
    pub range: Range,
    pub color: Color,
}
