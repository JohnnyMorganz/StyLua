use std::convert::TryInto;

use lsp_server::{Connection, ErrorCode, Message, Response};
use lsp_textdocument::{FullTextDocument, TextDocuments};
use lsp_types::{
    request::{Formatting, RangeFormatting, Request},
    DocumentFormattingParams, DocumentRangeFormattingParams, InitializeResult, OneOf, Position,
    Range, ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextEdit, Uri,
};
use similar::{DiffOp, TextDiff};
use stylua_lib::{format_code, OutputVerification};

use crate::{config::ConfigResolver, opt};

fn diffop_to_textedit(op: DiffOp, formatted_contents: &str) -> Option<TextEdit> {
    match op {
        DiffOp::Equal {
            old_index: _,
            new_index: _,
            len: _,
        } => None,
        DiffOp::Delete {
            old_index,
            old_len,
            new_index: _,
        } => Some(TextEdit {
            range: Range {
                start: Position {
                    line: old_index.try_into().expect("usize fits into u32"),
                    character: 0,
                },
                end: Position {
                    line: (old_index + old_len)
                        .try_into()
                        .expect("usize fits into u32"),
                    character: 0,
                },
            },
            new_text: String::new(),
        }),
        DiffOp::Insert {
            old_index,
            new_index,
            new_len,
        } => {
            let insert_position = Position {
                line: old_index.try_into().expect("usize fits into u32"),
                character: 0,
            };
            Some(TextEdit {
                range: Range {
                    start: insert_position,
                    end: insert_position,
                },
                new_text: formatted_contents
                    .lines()
                    .skip(new_index)
                    .take(new_len)
                    .collect::<Vec<_>>()
                    .join("\n"),
            })
        }
        DiffOp::Replace {
            old_index,
            old_len,
            new_index,
            new_len,
        } => Some(TextEdit {
            range: Range {
                start: Position {
                    line: old_index.try_into().expect("usize fits into u32"),
                    character: 0,
                },
                end: Position {
                    line: (old_index + old_len)
                        .try_into()
                        .expect("usize fits into u32"),
                    character: 0,
                },
            },
            new_text: formatted_contents
                .lines()
                .skip(new_index)
                .take(new_len)
                .collect::<Vec<_>>()
                .join("\n"),
        }),
    }
}

fn handle_formatting(
    uri: &Uri,
    document: &FullTextDocument,
    range: Option<stylua_lib::Range>,
    config_resolver: &mut ConfigResolver,
) -> Option<Vec<TextEdit>> {
    if document.language_id() != "lua" && document.language_id() != "luau" {
        return None;
    }

    let contents = document.get_content(None);

    let config = config_resolver
        .load_configuration(uri.path().as_str().as_ref())
        .unwrap_or_default();

    let formatted_contents = format_code(contents, config, range, OutputVerification::None).ok()?;

    let operations = TextDiff::from_lines(contents, &formatted_contents).grouped_ops(0);
    let edits = operations
        .into_iter()
        .flat_map(|operations| {
            operations
                .into_iter()
                .filter_map(|op| diffop_to_textedit(op, &formatted_contents))
        })
        .collect();
    Some(edits)
}

fn handle_request(
    request: lsp_server::Request,
    documents: &TextDocuments,
    config_resolver: &mut ConfigResolver,
) -> Response {
    match request.method.as_str() {
        Formatting::METHOD => {
            match serde_json::from_value::<DocumentFormattingParams>(request.params) {
                Ok(params) => {
                    let Some(document) = documents.get_document(&params.text_document.uri) else {
                        return Response::new_err(
                            request.id,
                            ErrorCode::RequestFailed as i32,
                            format!(
                                "no document found for '{}'",
                                params.text_document.uri.as_str()
                            ),
                        );
                    };

                    match handle_formatting(
                        &params.text_document.uri,
                        document,
                        None,
                        config_resolver,
                    ) {
                        Some(edits) => Response::new_ok(request.id, edits),
                        None => Response::new_ok(request.id, serde_json::Value::Null),
                    }
                }
                Err(err) => Response::new_err(
                    request.id,
                    lsp_server::ErrorCode::RequestFailed as i32,
                    err.to_string(),
                ),
            }
        }
        RangeFormatting::METHOD => {
            match serde_json::from_value::<DocumentRangeFormattingParams>(request.params) {
                Ok(params) => {
                    let Some(document) = documents.get_document(&params.text_document.uri) else {
                        return Response::new_err(
                            request.id,
                            1,
                            format!(
                                "no document found for '{}'",
                                params.text_document.uri.as_str()
                            ),
                        );
                    };

                    let range = stylua_lib::Range::from_values(
                        Some(document.offset_at(params.range.start).try_into().unwrap()),
                        Some(document.offset_at(params.range.end).try_into().unwrap()),
                    );

                    match handle_formatting(
                        &params.text_document.uri,
                        document,
                        Some(range),
                        config_resolver,
                    ) {
                        Some(edits) => Response::new_ok(request.id, edits),
                        None => Response::new_ok(request.id, serde_json::Value::Null),
                    }
                }
                Err(err) => Response::new_err(
                    request.id,
                    lsp_server::ErrorCode::RequestFailed as i32,
                    err.to_string(),
                ),
            }
        }
        _ => Response::new_err(
            request.id,
            lsp_server::ErrorCode::MethodNotFound as i32,
            format!("server does not support method '{}'", request.method),
        ),
    }
}

fn main_loop(connection: Connection, config_resolver: &mut ConfigResolver) -> anyhow::Result<()> {
    let initialize_result = InitializeResult {
        capabilities: ServerCapabilities {
            document_formatting_provider: Some(OneOf::Left(true)),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            ..Default::default()
        },
        server_info: Some(ServerInfo {
            name: "stylua".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
    };

    let (id, _) = connection.initialize_start()?;
    connection.initialize_finish(id, serde_json::to_value(initialize_result)?)?;

    let mut documents = TextDocuments::new();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }

                let response = handle_request(req, &documents, config_resolver);
                connection.sender.send(Message::Response(response))?
            }
            Message::Response(_) => {}
            Message::Notification(notification) => {
                documents.listen(notification.method.as_str(), &notification.params);
            }
        }
    }
    Ok(())
}

pub fn run(opt: opt::Opt) -> anyhow::Result<()> {
    // Load the configuration
    let opt_for_config_resolver = opt.clone();
    let mut config_resolver = ConfigResolver::new(&opt_for_config_resolver)?;

    let (connection, io_threads) = Connection::stdio();

    main_loop(connection, &mut config_resolver)?;

    io_threads.join()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use clap::Parser;
    use crossbeam_channel::Receiver;
    use lsp_server::Connection;

    use lsp_server::{ErrorCode, Message, Notification, Request, RequestId, Response};
    use lsp_types::{
        notification::{DidOpenTextDocument, Exit, Initialized, Notification as NotificationType},
        request::{Formatting, Initialize, RangeFormatting, Request as RequestType, Shutdown},
        DidOpenTextDocumentParams, DocumentFormattingParams, DocumentRangeFormattingParams,
        FormattingOptions, InitializeParams, Position, Range, TextDocumentIdentifier,
        TextDocumentItem, TextEdit, Uri, WorkDoneProgressParams,
    };
    use lsp_types::{
        OneOf, ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    };
    use serde::de::DeserializeOwned;
    use serde_json::to_value;

    use crate::{config::ConfigResolver, lsp::main_loop, opt::Opt};

    fn initialize(id: i32) -> Message {
        Message::Request(Request {
            id: RequestId::from(id),
            method: <Initialize as lsp_types::request::Request>::METHOD.to_string(),
            params: to_value(InitializeParams::default()).unwrap(),
        })
    }

    fn initialized() -> Message {
        Message::Notification(Notification {
            method: Initialized::METHOD.to_string(),
            params: serde_json::Value::Null,
        })
    }

    fn shutdown(id: i32) -> Message {
        Message::Request(Request {
            id: RequestId::from(id),
            method: Shutdown::METHOD.to_string(),
            params: serde_json::Value::Null,
        })
    }

    fn exit() -> Message {
        Message::Notification(Notification {
            method: Exit::METHOD.to_string(),
            params: serde_json::Value::Null,
        })
    }

    fn expect_server_initialized(receiver: &Receiver<Message>, response_id: i32) {
        match receiver.recv().unwrap() {
            Message::Response(Response {
                id,
                result: Some(result),
                error: None,
            }) if id == RequestId::from(response_id)
                && result
                    == serde_json::json!({
                    "capabilities": ServerCapabilities {
                        document_formatting_provider: Some(OneOf::Left(true)),
                        text_document_sync: Some(TextDocumentSyncCapability::Kind(
                            TextDocumentSyncKind::INCREMENTAL,
                        )),
                        ..Default::default()
                    },
                    "serverInfo": Some(ServerInfo {
                        name: "stylua".to_string(),
                        version: Some(env!("CARGO_PKG_VERSION").to_string()),
                    }),
                    }) => {}
            _ => panic!("assertion failed"),
        }
    }

    fn expect_response<T: DeserializeOwned>(receiver: &Receiver<Message>, response_id: i32) -> T {
        match receiver.recv().unwrap() {
            Message::Response(Response {
                id,
                result: Some(result),
                error: None,
            }) if id == RequestId::from(response_id) => {
                serde_json::from_value::<T>(result).unwrap()
            }
            _ => panic!("assertion failed"),
        }
    }

    fn expect_server_shutdown(receiver: &Receiver<Message>, response_id: i32) {
        match receiver.recv().unwrap() {
            Message::Response(Response {
                id,
                result: Some(result),
                error: None,
            }) if id == RequestId::from(response_id) && result == serde_json::Value::Null => {}
            _ => panic!("assertion failed"),
        }
    }

    #[test]
    fn test_lsp_initialize() {
        let opt = Opt::parse_from(vec!["BINARY_NAME"]);
        let mut config_resolver = ConfigResolver::new(&opt).unwrap();

        let (server, client) = Connection::memory();
        client.sender.send(initialize(1)).unwrap();
        client.sender.send(initialized()).unwrap();
        client.sender.send(shutdown(2)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);
        expect_server_shutdown(&client.receiver, 2);
        assert!(client.receiver.is_empty());
    }

    #[test]
    fn test_lsp_document_formatting() {
        let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();
        let contents = "local  x  =  1";

        let opt = Opt::parse_from(vec!["BINARY_NAME"]);
        let mut config_resolver = ConfigResolver::new(&opt).unwrap();

        let (server, client) = Connection::memory();
        client.sender.send(initialize(1)).unwrap();
        client.sender.send(initialized()).unwrap();
        client
            .sender
            .send(Message::Notification(Notification {
                method: DidOpenTextDocument::METHOD.to_string(),
                params: to_value(DidOpenTextDocumentParams {
                    text_document: TextDocumentItem {
                        uri: uri.clone(),
                        language_id: "lua".to_string(),
                        version: 0,
                        text: contents.to_string(),
                    },
                })
                .unwrap(),
            }))
            .unwrap();
        client
            .sender
            .send(Message::Request(Request {
                id: RequestId::from(2),
                method: Formatting::METHOD.to_string(),
                params: to_value(DocumentFormattingParams {
                    text_document: TextDocumentIdentifier { uri },
                    options: FormattingOptions::default(),
                    work_done_progress_params: WorkDoneProgressParams::default(),
                })
                .unwrap(),
            }))
            .unwrap();
        client.sender.send(shutdown(3)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);

        let edits: Vec<TextEdit> = expect_response(&client.receiver, 2);
        assert_eq!(edits.len(), 1);
        assert_eq!(
            edits[0],
            TextEdit {
                range: Range {
                    start: Position::new(0, 0),
                    end: Position::new(0, 14),
                },
                new_text: "local x = 1\n".to_string(),
            }
        );

        expect_server_shutdown(&client.receiver, 3);
        assert!(client.receiver.is_empty());
    }

    #[test]
    fn test_lsp_range_formatting() {
        let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();
        let contents = "local  x  =  1\nlocal    y   =   2";

        let opt = Opt::parse_from(vec!["BINARY_NAME"]);
        let mut config_resolver = ConfigResolver::new(&opt).unwrap();

        let (server, client) = Connection::memory();
        client.sender.send(initialize(1)).unwrap();
        client.sender.send(initialized()).unwrap();
        client
            .sender
            .send(Message::Notification(Notification {
                method: DidOpenTextDocument::METHOD.to_string(),
                params: to_value(DidOpenTextDocumentParams {
                    text_document: TextDocumentItem {
                        uri: uri.clone(),
                        language_id: "lua".to_string(),
                        version: 0,
                        text: contents.to_string(),
                    },
                })
                .unwrap(),
            }))
            .unwrap();
        client
            .sender
            .send(Message::Request(Request {
                id: RequestId::from(2),
                method: RangeFormatting::METHOD.to_string(),
                params: to_value(DocumentRangeFormattingParams {
                    text_document: TextDocumentIdentifier { uri },
                    range: Range::new(Position::new(1, 0), Position::new(1, 18)),
                    options: FormattingOptions::default(),
                    work_done_progress_params: WorkDoneProgressParams::default(),
                })
                .unwrap(),
            }))
            .unwrap();
        client.sender.send(shutdown(3)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);

        let edits: Vec<TextEdit> = expect_response(&client.receiver, 2);
        assert_eq!(edits.len(), 1);
        assert_eq!(
            edits[0],
            TextEdit {
                range: Range {
                    start: Position::new(0, 0),
                    end: Position::new(1, 18),
                },
                new_text: "local  x  =  1\nlocal y = 2\n".to_string(),
            }
        );

        expect_server_shutdown(&client.receiver, 3);
        assert!(client.receiver.is_empty());
    }

    #[test]
    fn test_lsp_ignore_formatting_for_non_lua_files() {
        let uri = Uri::from_str("file:///home/documents/file.txt").unwrap();
        let contents = "local x";

        let opt = Opt::parse_from(vec!["BINARY_NAME"]);
        let mut config_resolver = ConfigResolver::new(&opt).unwrap();

        let (server, client) = Connection::memory();
        client.sender.send(initialize(1)).unwrap();
        client.sender.send(initialized()).unwrap();
        client
            .sender
            .send(Message::Notification(Notification {
                method: DidOpenTextDocument::METHOD.to_string(),
                params: to_value(DidOpenTextDocumentParams {
                    text_document: TextDocumentItem {
                        uri: uri.clone(),
                        language_id: "txt".to_string(),
                        version: 0,
                        text: contents.to_string(),
                    },
                })
                .unwrap(),
            }))
            .unwrap();
        client
            .sender
            .send(Message::Request(Request {
                id: RequestId::from(2),
                method: RangeFormatting::METHOD.to_string(),
                params: to_value(DocumentRangeFormattingParams {
                    text_document: TextDocumentIdentifier { uri },
                    range: Range::new(Position::new(1, 0), Position::new(1, 18)),
                    options: FormattingOptions::default(),
                    work_done_progress_params: WorkDoneProgressParams::default(),
                })
                .unwrap(),
            }))
            .unwrap();
        client.sender.send(shutdown(3)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);

        let edits: serde_json::Value = expect_response(&client.receiver, 2);
        assert_eq!(edits, serde_json::Value::Null);

        expect_server_shutdown(&client.receiver, 3);
        assert!(client.receiver.is_empty());
    }

    #[test]
    fn test_lsp_fails_for_unknown_files() {
        let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();

        let opt = Opt::parse_from(vec!["BINARY_NAME"]);
        let mut config_resolver = ConfigResolver::new(&opt).unwrap();

        let (server, client) = Connection::memory();
        client.sender.send(initialize(1)).unwrap();
        client.sender.send(initialized()).unwrap();
        client
            .sender
            .send(Message::Request(Request {
                id: RequestId::from(2),
                method: Formatting::METHOD.to_string(),
                params: to_value(DocumentFormattingParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    options: FormattingOptions::default(),
                    work_done_progress_params: WorkDoneProgressParams::default(),
                })
                .unwrap(),
            }))
            .unwrap();
        client.sender.send(shutdown(3)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);

        let Message::Response(Response {
            id,
            result: None,
            error: Some(error),
        }) = client.receiver.recv().unwrap()
        else {
            unreachable!()
        };
        assert!(id == RequestId::from(2));
        assert_eq!(error.code, ErrorCode::RequestFailed as i32);
        assert_eq!(
            error.message,
            format!("no document found for '{}'", uri.as_str())
        );

        expect_server_shutdown(&client.receiver, 3);
        assert!(client.receiver.is_empty());
    }
}
