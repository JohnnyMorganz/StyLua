use std::convert::TryInto;

use lsp_server::{Connection, ErrorCode, Message, Response};
use lsp_textdocument::{FullTextDocument, TextDocuments};
use lsp_types::{
    request::{Formatting, RangeFormatting, Request},
    DocumentFormattingParams, DocumentRangeFormattingParams, OneOf, Position, Range,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Uri,
};
use stylua_lib::{format_code, OutputVerification};

use crate::{config::ConfigResolver, opt};

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

    let last_line_idx = document.line_count().saturating_sub(1);
    let last_line_offset = document.offset_at(Position::new(last_line_idx, 0));
    let last_col = document.content_len() - last_line_offset;

    // TODO: We can be smarter about this in the future, and update only the parts that changed (using output_diff)
    Some(vec![TextEdit {
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(last_line_idx, last_col),
        },
        new_text: formatted_contents,
    }])
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

pub fn run(opt: opt::Opt) -> anyhow::Result<()> {
    // Load the configuration
    let opt_for_config_resolver = opt.clone();
    let mut config_resolver = ConfigResolver::new(&opt_for_config_resolver)?;

    let (connection, io_threads) = Connection::stdio();

    let capabilities = ServerCapabilities {
        document_formatting_provider: Some(OneOf::Left(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        ..Default::default()
    };

    connection.initialize(serde_json::to_value(capabilities)?)?;

    let mut documents = TextDocuments::new();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }

                let response = handle_request(req, &documents, &mut config_resolver);
                connection.sender.send(Message::Response(response))?
            }
            Message::Response(_) => {}
            Message::Notification(notification) => {
                documents.listen(notification.method.as_str(), &notification.params);
            }
        }
    }

    io_threads.join()?;

    Ok(())
}
