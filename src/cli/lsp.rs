use lsp_server::{Connection, Message, RequestId, Response};
use lsp_textdocument::TextDocuments;
use lsp_types::{
    request::{Formatting, RangeFormatting, Request},
    DocumentFormattingParams, OneOf, Position, Range, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit,
};

use crate::{config::ConfigResolver, opt};

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

                match req.method.as_str() {
                    Formatting::METHOD => {
                        let result =
                            match serde_json::from_value::<DocumentFormattingParams>(req.params) {
                                Ok(params) => {
                                    let res = handle_formatting(
                                        req.id.clone(),
                                        params,
                                        &mut config_resolver,
                                        &documents,
                                    );

                                    res.unwrap_or(Response::new_ok(req.id, serde_json::Value::Null))
                                }
                                Err(err) => Response::new_err(req.id, 1, err.to_string()),
                            };

                        connection.sender.send(Message::Response(result))?;
                    }
                    RangeFormatting::METHOD => {
                        // TODO: Would be cool to support this in the future
                    }
                    _ => {}
                }
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

fn handle_formatting(
    id: RequestId,
    params: DocumentFormattingParams,
    config_resolver: &mut ConfigResolver,
    documents: &TextDocuments,
) -> Option<Response> {
    let uri = params.text_document.uri;
    let path = uri.path().as_str();

    let src = documents.get_document_content(&uri, None)?;

    let config = config_resolver
        .load_configuration(path.as_ref())
        .unwrap_or_default();

    let new_text =
        stylua_lib::format_code(src, config, None, stylua_lib::OutputVerification::None).ok()?;

    if new_text == src {
        return None;
    }

    // TODO: We can be smarter about this in the future, and update only the parts that changed
    let edit = TextEdit {
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(u32::MAX, 0),
        },
        new_text,
    };

    let edits: Vec<TextEdit> = vec![edit];

    Some(Response::new_ok(id, edits))
}
