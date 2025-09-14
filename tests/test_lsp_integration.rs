#![cfg(feature = "lsp")]
use std::str::FromStr;

use assert_cmd::Command;

use lsp_server::{ErrorCode, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    notification::{DidOpenTextDocument, Exit, Initialized, Notification as NotificationType},
    request::{Formatting, Initialize, RangeFormatting, Request as RequestType, Shutdown},
    DidOpenTextDocumentParams, DocumentFormattingParams, DocumentRangeFormattingParams,
    FormattingOptions, InitializeParams, OneOf, Position, Range, ServerCapabilities,
    TextDocumentIdentifier, TextDocumentItem, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextEdit, Uri, WorkDoneProgressParams,
};
use serde_json::to_value;

fn create_stylua() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn join_messages(input: Vec<Message>) -> Vec<u8> {
    let mut buffer = Vec::new();
    input
        .iter()
        .for_each(|message| message.clone().write(&mut buffer).unwrap());
    buffer
}

fn initialize(id: i32) -> Message {
    Message::Request(Request {
        id: RequestId::from(id),
        method: <Initialize as lsp_types::request::Request>::METHOD.to_string(),
        params: to_value(InitializeParams::default()).unwrap(),
    })
}

fn server_initialized(id: i32) -> Message {
    Message::Response(Response::new_ok(
        RequestId::from(id),
        serde_json::json!({
        "capabilities": ServerCapabilities {
            document_formatting_provider: Some(OneOf::Left(true)),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            ..Default::default()
        }}),
    ))
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

fn server_shutdown(id: i32) -> Message {
    Message::Response(Response::new_ok(RequestId::from(id), ()))
}

fn exit() -> Message {
    Message::Notification(Notification {
        method: Exit::METHOD.to_string(),
        params: serde_json::Value::Null,
    })
}

#[test]
fn test_lsp_initialize() {
    let input = join_messages(vec![initialize(1), initialized(), shutdown(2), exit()]);
    let expected_output = join_messages(vec![server_initialized(1), server_shutdown(2)]);

    let mut cmd = create_stylua();
    let output = cmd.arg("--lsp").write_stdin(input).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(expected_output).unwrap()
    )
}

#[test]
fn test_lsp_document_formatting() {
    let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();
    let contents = "local  x  =  1";

    let input = join_messages(vec![
        initialize(1),
        initialized(),
        Message::Notification(Notification {
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
        }),
        Message::Request(Request {
            id: RequestId::from(2),
            method: Formatting::METHOD.to_string(),
            params: to_value(DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options: FormattingOptions::default(),
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .unwrap(),
        }),
        shutdown(3),
        exit(),
    ]);

    let expected_output = join_messages(vec![
        server_initialized(1),
        Message::Response(Response::new_ok(
            RequestId::from(2),
            vec![TextEdit {
                range: Range {
                    start: Position::new(0, 0),
                    end: Position::new(0, 14),
                },
                new_text: "local x = 1\n".to_string(),
            }],
        )),
        server_shutdown(3),
    ]);

    let mut cmd = create_stylua();
    let output = cmd.arg("--lsp").write_stdin(input).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(expected_output).unwrap()
    )
}

#[test]
fn test_lsp_range_formatting() {
    let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();
    let contents = "local  x  =  1\nlocal    y   =   2";

    let input = join_messages(vec![
        initialize(1),
        initialized(),
        Message::Notification(Notification {
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
        }),
        Message::Request(Request {
            id: RequestId::from(2),
            method: RangeFormatting::METHOD.to_string(),
            params: to_value(DocumentRangeFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                range: Range::new(Position::new(1, 0), Position::new(1, 18)),
                options: FormattingOptions::default(),
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .unwrap(),
        }),
        shutdown(3),
        exit(),
    ]);

    let expected_output = join_messages(vec![
        server_initialized(1),
        Message::Response(Response::new_ok(
            RequestId::from(2),
            vec![TextEdit {
                range: Range {
                    start: Position::new(0, 0),
                    end: Position::new(1, 18),
                },
                new_text: "local  x  =  1\nlocal y = 2\n".to_string(),
            }],
        )),
        server_shutdown(3),
    ]);

    let mut cmd = create_stylua();
    let output = cmd.arg("--lsp").write_stdin(input).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(expected_output).unwrap()
    )
}

#[test]
fn test_lsp_ignore_formatting_for_non_lua_files() {
    let uri = Uri::from_str("file:///home/documents/file.txt").unwrap();
    let contents = "local x";

    let input = join_messages(vec![
        initialize(1),
        initialized(),
        Message::Notification(Notification {
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
        }),
        Message::Request(Request {
            id: RequestId::from(2),
            method: Formatting::METHOD.to_string(),
            params: to_value(DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options: FormattingOptions::default(),
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .unwrap(),
        }),
        shutdown(3),
        exit(),
    ]);

    let expected_output = join_messages(vec![
        server_initialized(1),
        Message::Response(Response::new_ok(
            RequestId::from(2),
            serde_json::Value::Null,
        )),
        server_shutdown(3),
    ]);

    let mut cmd = create_stylua();
    let output = cmd.arg("--lsp").write_stdin(input).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(expected_output).unwrap()
    )
}

#[test]
fn test_lsp_fails_for_unknown_files() {
    let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();

    let input = join_messages(vec![
        initialize(1),
        initialized(),
        Message::Request(Request {
            id: RequestId::from(2),
            method: Formatting::METHOD.to_string(),
            params: to_value(DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                options: FormattingOptions::default(),
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .unwrap(),
        }),
        shutdown(3),
        exit(),
    ]);

    let expected_output = join_messages(vec![
        server_initialized(1),
        Message::Response(Response::new_err(
            RequestId::from(2),
            ErrorCode::RequestFailed as i32,
            format!("no document found for '{}'", uri.as_str()),
        )),
        server_shutdown(3),
    ]);

    let mut cmd = create_stylua();
    let output = cmd.arg("--lsp").write_stdin(input).output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(expected_output).unwrap()
    )
}
