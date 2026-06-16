use serde_json::{Value, json};
use std::collections::HashMap;
use std::io::{self, Read, Write};

fn read_message(mut stdin: &io::Stdin) -> Option<Value> {
    let mut line = String::new();

    // Read Content-Length header
    stdin.read_line(&mut line).ok()?;
    let content_length: usize = line.strip_prefix("Content-Length: ")?.trim().parse().ok()?;

    // Read \r\n
    line.clear();
    stdin.read_line(&mut line).ok()?;

    // Read body
    let mut body = vec![0u8; content_length];
    stdin.read_exact(&mut body).ok()?;

    serde_json::from_slice(&body).ok()
}

fn write_message(value: &Value) {
    let body = value.to_string();
    print!("Content-Length: {}\r\n\r\n{}", body.len(), body);
    io::stdout().flush().unwrap();
}

fn main() {
    let documents: HashMap<String, Document> = HashMap::new();
    let stdin = io::stdin();
    while let Some(msg) = read_message(&stdin) {
        let Some(method) = msg.get("method").and_then(|m| m.as_str()) else {
            break;
        };

        match method {
            "initialize" => {
                let id = msg["id"].clone();

                let response = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "capabilities": {
                            "openClose": true,
                            "textDocumentSync": 2, // Incremental
                        },
                        "serverInfo": {
                            "name": "chroma-ls",
                            "version": env!("CARGO_PKG_VERSION").to_string(),
                        }
                    }
                });

                write_message(&response);
            }

            "shutdown" => {
                let id = msg["id"].clone();
                let response = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": null
                });
                write_message(&response);
            }

            "textDocument/didOpen" => {}

            "exit" => {
                std::process::exit(0);
            }

            _ => {
                eprintln!("unknown method: {method}");
            }
        }
    }
}
