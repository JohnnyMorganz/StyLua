use std::{convert::TryInto, path::PathBuf};

use lsp_server::{Connection, ErrorCode, Message, Response};
use lsp_textdocument::{FullTextDocument, TextDocuments};
use lsp_types::{
    request::{Formatting, RangeFormatting, Request},
    DocumentFormattingParams, DocumentRangeFormattingParams, FormattingOptions, InitializeParams,
    InitializeResult, OneOf, Range, ServerCapabilities, ServerInfo, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextEdit, Uri, WorkspaceFolder,
};
use serde::{Deserialize, Serialize};
use similar::{DiffOp, TextDiff};
use stylua_lib::{format_code, IndentType, OutputVerification};

use crate::{config::ConfigResolver, opt, stylua_ignore};

fn diffop_to_textedit(
    op: DiffOp,
    document: &FullTextDocument,
    original_contents: &str,
    formatted_contents: &str,
) -> Option<TextEdit> {
    let range = |start: usize, len: usize| {
        let byte_start = original_contents
            .char_indices()
            .nth(start)
            .map(|(i, _)| i)
            .unwrap_or(original_contents.len());
        let byte_end = original_contents
            .char_indices()
            .nth(start + len)
            .map(|(i, _)| i)
            .unwrap_or(original_contents.len());
        Range {
            start: document.position_at(byte_start.try_into().expect("usize fits into u32")),
            end: document.position_at(byte_end.try_into().expect("usize fits into u32")),
        }
    };

    let lookup = |start: usize, len: usize| {
        formatted_contents
            .chars()
            .skip(start)
            .take(len)
            .collect::<String>()
    };

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
            range: range(old_index, old_len),
            new_text: String::new(),
        }),
        DiffOp::Insert {
            old_index,
            new_index,
            new_len,
        } => Some(TextEdit {
            range: range(old_index, 0),
            new_text: lookup(new_index, new_len),
        }),
        DiffOp::Replace {
            old_index,
            old_len,
            new_index,
            new_len,
        } => Some(TextEdit {
            range: range(old_index, old_len),
            new_text: lookup(new_index, new_len),
        }),
    }
}

trait ToFilePath {
    fn to_file_path(&self) -> PathBuf;
}

impl ToFilePath for Uri {
    fn to_file_path(&self) -> PathBuf {
        // Based on https://github.com/tower-lsp-community/ls-types/blob/d2cc799f26da91354d2c4a1f7e5ef3f4a90797eb/src/uri.rs#L165-L207
        //
        // Made some modification because `lsp-types` currently use 0.1.4 instead of
        // 0.4.1 from the source.
        let path = self.path().as_str();

        #[cfg(windows)]
        {
            let auth_host = self
                .authority()
                .map(|auth| auth.host().as_str())
                .unwrap_or_default();

            if auth_host.is_empty() {
                let host = path.to_string();
                let host = host.get(1..).unwrap();
                return PathBuf::from(host);
            }

            PathBuf::from(format!("{auth_host}:"))
                .components()
                .chain(PathBuf::from(path).components())
                .collect()
        }
        #[cfg(not(windows))]
        {
            PathBuf::from(path)
        }
    }
}

struct LanguageServer<'a> {
    documents: TextDocuments,
    workspace_folders: Vec<WorkspaceFolder>,
    root_uri: Option<Uri>,
    search_parent_directories: bool,
    respect_editor_formatting_options: bool,
    config_resolver: &'a mut ConfigResolver<'a>,
}

enum FormattingError {
    StyLuaError,
    NotLuaDocument,
    DocumentNotFound,
    FileIsIgnored,
}

impl LanguageServer<'_> {
    fn new<'a>(
        workspace_folders: Vec<WorkspaceFolder>,
        root_uri: Option<Uri>,
        search_parent_directories: bool,
        respect_editor_formatting_options: bool,
        config_resolver: &'a mut ConfigResolver<'a>,
    ) -> LanguageServer<'a> {
        LanguageServer {
            documents: TextDocuments::new(),
            workspace_folders,
            root_uri,
            search_parent_directories,
            respect_editor_formatting_options,
            config_resolver,
        }
    }

    fn find_config_root(&self, uri: &Uri) -> PathBuf {
        let mut best_workspace = None;
        let mut best_len = 0;
        let check_str = uri.as_str();
        for workspace in &self.workspace_folders {
            if *uri == workspace.uri {
                return workspace.uri.to_file_path();
            }

            let prefix_str = workspace.uri.as_str();
            if prefix_str.len() < best_len {
                continue;
            }

            if check_str.starts_with(prefix_str) {
                best_workspace = Some(&workspace.uri);
                best_len = prefix_str.len()
            }
        }

        match best_workspace {
            Some(workspace) => workspace.to_file_path(),
            None => match &self.root_uri {
                Some(root_uri) => root_uri.to_file_path(),
                None => std::env::current_dir().expect("Could not find current directory"),
            },
        }
    }

    fn handle_formatting(
        &mut self,
        uri: &Uri,
        range: Option<Range>,
        formatting_options: Option<&FormattingOptions>,
    ) -> Result<Vec<TextEdit>, FormattingError> {
        let Some(document) = self.documents.get_document(uri) else {
            return Err(FormattingError::DocumentNotFound);
        };

        let range = range.map(|lsp_range| {
            stylua_lib::Range::from_values(
                Some(document.offset_at(lsp_range.start).try_into().unwrap()),
                Some(document.offset_at(lsp_range.end).try_into().unwrap()),
            )
        });

        if document.language_id() != "lua" && document.language_id() != "luau" {
            return Err(FormattingError::NotLuaDocument);
        }

        let search_root = Some(self.find_config_root(uri));
        let path = &uri.to_file_path();

        if stylua_ignore::path_is_stylua_ignored(
            path,
            self.search_parent_directories,
            search_root.clone(),
        )
        .unwrap_or(false)
        {
            return Err(FormattingError::FileIsIgnored);
        }

        let contents = document.get_content(None);

        let mut config = self
            .config_resolver
            .load_configuration_with_search_root(path, search_root)
            .unwrap_or_default();

        if let Some(formatting_options) = formatting_options {
            config.indent_width = formatting_options
                .tab_size
                .try_into()
                .expect("u32 fits into usize");
            config.indent_type = if formatting_options.insert_spaces {
                IndentType::Spaces
            } else {
                IndentType::Tabs
            };
        }

        let Ok(formatted_contents) = format_code(contents, config, range, OutputVerification::None)
        else {
            return Err(FormattingError::StyLuaError);
        };

        let operations = TextDiff::from_chars(contents, &formatted_contents).grouped_ops(0);

        let edits = operations
            .into_iter()
            .flat_map(|operations| {
                operations.into_iter().filter_map(|op| {
                    diffop_to_textedit(op, document, contents, &formatted_contents)
                })
            })
            .collect();
        Ok(edits)
    }

    fn handle_request(&mut self, request: lsp_server::Request) -> Response {
        match request.method.as_str() {
            Formatting::METHOD => {
                match serde_json::from_value::<DocumentFormattingParams>(request.params) {
                    Ok(params) => {
                        match self.handle_formatting(
                            &params.text_document.uri,
                            None,
                            self.respect_editor_formatting_options
                                .then_some(&params.options),
                        ) {
                            Ok(edits) => Response::new_ok(request.id, edits),
                            Err(FormattingError::StyLuaError)
                            | Err(FormattingError::NotLuaDocument)
                            | Err(FormattingError::FileIsIgnored) => {
                                Response::new_ok(request.id, serde_json::Value::Null)
                            }
                            Err(FormattingError::DocumentNotFound) => Response::new_err(
                                request.id,
                                ErrorCode::RequestFailed as i32,
                                format!(
                                    "no document found for '{}'",
                                    params.text_document.uri.as_str()
                                ),
                            ),
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
                        match self.handle_formatting(
                            &params.text_document.uri,
                            Some(params.range),
                            self.respect_editor_formatting_options
                                .then_some(&params.options),
                        ) {
                            Ok(edits) => Response::new_ok(request.id, edits),
                            Err(FormattingError::StyLuaError)
                            | Err(FormattingError::NotLuaDocument)
                            | Err(FormattingError::FileIsIgnored) => {
                                Response::new_ok(request.id, serde_json::Value::Null)
                            }
                            Err(FormattingError::DocumentNotFound) => Response::new_err(
                                request.id,
                                ErrorCode::RequestFailed as i32,
                                format!(
                                    "no document found for '{}'",
                                    params.text_document.uri.as_str()
                                ),
                            ),
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

    fn handle_notification(&mut self, notification: lsp_server::Notification) {
        self.documents
            .listen(notification.method.as_str(), &notification.params);
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
struct InitializationOptions {
    respect_editor_formatting_options: Option<bool>,
}

fn main_loop<'a>(
    connection: Connection,
    search_parent_directories: bool,
    config_resolver: &'a mut ConfigResolver<'a>,
) -> anyhow::Result<()> {
    let initialize_result = InitializeResult {
        capabilities: ServerCapabilities {
            document_range_formatting_provider: Some(OneOf::Left(true)),
            document_formatting_provider: Some(OneOf::Left(true)),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            ..Default::default()
        },
        server_info: Some(ServerInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
    };

    let (id, initialize_params) = connection.initialize_start()?;

    let initialize_params = serde_json::from_value::<InitializeParams>(initialize_params)?;
    let respect_editor_formatting_options = initialize_params
        .initialization_options
        .and_then(|opt| serde_json::from_value::<InitializationOptions>(opt).ok())
        .and_then(|opt| opt.respect_editor_formatting_options)
        .unwrap_or_default();

    connection.initialize_finish(id, serde_json::to_value(initialize_result)?)?;

    let mut language_server = LanguageServer::new(
        initialize_params.workspace_folders.unwrap_or_default(),
        #[allow(deprecated)]
        initialize_params.root_uri,
        search_parent_directories,
        respect_editor_formatting_options,
        config_resolver,
    );

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }

                let response = language_server.handle_request(req);
                connection.sender.send(Message::Response(response))?
            }
            Message::Response(_) => {}
            Message::Notification(notification) => {
                language_server.handle_notification(notification)
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

    main_loop(
        connection,
        opt.search_parent_directories,
        &mut config_resolver,
    )?;

    io_threads.join()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::convert::TryInto;
    use std::path::Path;
    use std::str::FromStr;

    use clap::Parser;
    use crossbeam_channel::Receiver;

    use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
    use lsp_types::{
        notification::{DidOpenTextDocument, Exit, Initialized, Notification as NotificationType},
        request::{Formatting, Initialize, RangeFormatting, Request as RequestType, Shutdown},
        DidOpenTextDocumentParams, DocumentFormattingParams, DocumentRangeFormattingParams,
        FormattingOptions, InitializeParams, OneOf, Position, Range, ServerCapabilities,
        ServerInfo, TextDocumentIdentifier, TextDocumentItem, TextDocumentSyncCapability,
        TextDocumentSyncKind, TextEdit, Uri, WorkDoneProgressParams,
    };
    use serde::de::DeserializeOwned;
    use serde_json::to_value;

    use crate::{
        config::ConfigResolver,
        lsp::{main_loop, InitializationOptions},
        opt::Opt,
    };

    use assert_fs::prelude::*;

    macro_rules! construct_tree {
        ({ $($file_name:literal:$file_contents:literal,)* }) => {{
            let cwd = assert_fs::TempDir::new().unwrap();

            $(
                cwd.child($file_name).write_str($file_contents).unwrap();
            )*

            cwd
        }};
        ({ $($file_name:literal:$file_contents:expr,)* }) => {{
            let cwd = assert_fs::TempDir::new().unwrap();

            $(
                cwd.child($file_name).write_str($file_contents).unwrap();
            )*

            cwd
        }};
    }

    macro_rules! lsp_test {
        ([$( $arguments:expr ),*], [$( $messages:expr ),*], [$( $tests:expr ),*]) => {
            let opt = Opt::parse_from(vec!["BINARY_NAME", "--lsp", $($arguments)*]);
            let mut config_resolver = ConfigResolver::new(&opt).unwrap();

            let (server, client) = Connection::memory();
            $(
                client.sender.send($messages).unwrap();
            )*

            main_loop(server, false, &mut config_resolver).unwrap();

            $(
                $tests(&client.receiver);
            )*
        };
    }

    fn initialize(id: i32, root_path: Option<&Path>) -> Message {
        Message::Request(Request {
            id: RequestId::from(id),
            method: <Initialize as lsp_types::request::Request>::METHOD.to_string(),
            params: to_value(InitializeParams {
                #[allow(deprecated)]
                root_uri: root_path.map(|path| Uri::from_str(path.to_str().unwrap()).unwrap()),
                ..Default::default()
            })
            .unwrap(),
        })
    }

    fn initialize_with_options(id: i32, options: InitializationOptions) -> Message {
        Message::Request(Request {
            id: RequestId::from(id),
            method: <Initialize as lsp_types::request::Request>::METHOD.to_string(),
            params: to_value(InitializeParams {
                initialization_options: Some(to_value(options).unwrap()),
                ..Default::default()
            })
            .unwrap(),
        })
    }

    fn initialized() -> Message {
        Message::Notification(Notification {
            method: Initialized::METHOD.to_string(),
            params: serde_json::Value::Null,
        })
    }

    fn open_text_document(uri: Uri, text: String) -> Message {
        Message::Notification(Notification {
            method: DidOpenTextDocument::METHOD.to_string(),
            params: to_value(DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri,
                    language_id: "lua".to_string(),
                    version: 0,
                    text,
                },
            })
            .unwrap(),
        })
    }

    fn format_document(id: i32, uri: Uri, options: FormattingOptions) -> Message {
        Message::Request(Request {
            id: RequestId::from(id),
            method: Formatting::METHOD.to_string(),
            params: to_value(DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options,
                work_done_progress_params: WorkDoneProgressParams::default(),
            })
            .unwrap(),
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
                        document_range_formatting_provider: Some(OneOf::Left(true)),
                        document_formatting_provider: Some(OneOf::Left(true)),
                        text_document_sync: Some(TextDocumentSyncCapability::Kind(
                            TextDocumentSyncKind::INCREMENTAL,
                        )),
                        ..Default::default()
                    },
                    "serverInfo": Some(ServerInfo {
                        name: env!("CARGO_PKG_NAME").to_string(),
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
        client.sender.send(initialize(1, None)).unwrap();
        client.sender.send(initialized()).unwrap();
        client.sender.send(shutdown(2)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, false, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);
        expect_server_shutdown(&client.receiver, 2);
        assert!(client.receiver.is_empty());
    }

    fn apply_text_edits_to(text: &str, mut edits: Vec<TextEdit>) -> String {
        edits.sort_by(|a, b| match a.range.start.line.cmp(&b.range.start.line) {
            Ordering::Equal => a
                .range
                .start
                .character
                .cmp(&b.range.start.character)
                .reverse(),
            order => order.reverse(),
        });
        let mut text = text.to_string();
        for edit in edits {
            let start = text
                .lines()
                .take(edit.range.start.line.try_into().unwrap())
                .map(|line| line.len() + '\n'.len_utf8())
                .sum::<usize>()
                + <u32 as TryInto<usize>>::try_into(edit.range.start.character).unwrap();
            let end = text
                .lines()
                .take(edit.range.end.line.try_into().unwrap())
                .map(|line| line.len() + '\n'.len_utf8())
                .sum::<usize>()
                + <u32 as TryInto<usize>>::try_into(edit.range.end.character).unwrap();
            text.replace_range(start..end, &edit.new_text);
        }
        text
    }

    #[test]
    fn test_lsp_document_formatting() {
        let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();
        let contents = "local  x  =  1";

        let opt = Opt::parse_from(vec!["BINARY_NAME"]);
        let mut config_resolver = ConfigResolver::new(&opt).unwrap();

        let (server, client) = Connection::memory();
        client.sender.send(initialize(1, None)).unwrap();
        client.sender.send(initialized()).unwrap();
        client
            .sender
            .send(open_text_document(uri.clone(), contents.to_string()))
            .unwrap();
        client
            .sender
            .send(format_document(
                2,
                uri.clone(),
                FormattingOptions::default(),
            ))
            .unwrap();
        client.sender.send(shutdown(3)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, false, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);

        let edits: Vec<TextEdit> = expect_response(&client.receiver, 2);
        assert_eq!(
            edits,
            [
                TextEdit {
                    range: Range::new(Position::new(0, 6), Position::new(0, 7)),
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range::new(Position::new(0, 8), Position::new(0, 9)),
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range::new(Position::new(0, 12), Position::new(0, 13)),
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range::new(Position::new(0, 14), Position::new(0, 14)),
                    new_text: "\n".to_string()
                },
            ]
        );
        let formatted = apply_text_edits_to(contents, edits);
        assert_eq!(formatted, "local x = 1\n");

        expect_server_shutdown(&client.receiver, 3);
        assert!(client.receiver.is_empty());
    }

    #[test]
    fn test_lsp_document_formatting_with_unicode() {
        let uri = Uri::from_str("file:///home/documents/file.lua").unwrap();
        let contents = "local  x  =  1 -- 测试\nlocal    y   =2";

        let opt = Opt::parse_from(vec!["BINARY_NAME"]);
        let mut config_resolver = ConfigResolver::new(&opt).unwrap();

        let (server, client) = Connection::memory();
        client.sender.send(initialize(1, None)).unwrap();
        client.sender.send(initialized()).unwrap();
        client
            .sender
            .send(open_text_document(uri.clone(), contents.to_string()))
            .unwrap();
        client
            .sender
            .send(format_document(
                2,
                uri.clone(),
                FormattingOptions::default(),
            ))
            .unwrap();
        client.sender.send(shutdown(3)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, false, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);

        let edits: Vec<TextEdit> = expect_response(&client.receiver, 2);
        assert_eq!(
            edits,
            [
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 6
                        },
                        end: Position {
                            line: 0,
                            character: 7
                        }
                    },
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 8
                        },
                        end: Position {
                            line: 0,
                            character: 9
                        }
                    },
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 11
                        },
                        end: Position {
                            line: 0,
                            character: 12
                        }
                    },
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 5
                        },
                        end: Position {
                            line: 1,
                            character: 7
                        }
                    },
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 8
                        },
                        end: Position {
                            line: 1,
                            character: 9
                        }
                    },
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 10
                        },
                        end: Position {
                            line: 1,
                            character: 12
                        }
                    },
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 14
                        },
                        end: Position {
                            line: 1,
                            character: 14
                        }
                    },
                    new_text: " ".to_string()
                },
                TextEdit {
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 15
                        },
                        end: Position {
                            line: 1,
                            character: 15
                        }
                    },
                    new_text: "\n".to_string()
                }
            ]
        );
        let formatted = apply_text_edits_to(contents, edits);
        assert_eq!(formatted, "local x = 1 -- 测试\nlocal y = 2\n");

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
        client.sender.send(initialize(1, None)).unwrap();
        client.sender.send(initialized()).unwrap();
        client
            .sender
            .send(open_text_document(uri.clone(), contents.to_string()))
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

        main_loop(server, false, &mut config_resolver).unwrap();

        expect_server_initialized(&client.receiver, 1);

        let edits: Vec<TextEdit> = expect_response(&client.receiver, 2);
        assert_eq!(
            edits,
            [
                TextEdit {
                    range: Range::new(Position::new(1, 6), Position::new(1, 9)),
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range::new(Position::new(1, 10), Position::new(1, 11)),
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range::new(Position::new(1, 12), Position::new(1, 13)),
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range::new(Position::new(1, 14), Position::new(1, 15)),
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range::new(Position::new(1, 16), Position::new(1, 17)),
                    new_text: "".to_string()
                },
                TextEdit {
                    range: Range::new(Position::new(1, 18), Position::new(1, 18)),
                    new_text: "\n".to_string()
                },
            ]
        );
        let formatted = apply_text_edits_to(contents, edits);
        assert_eq!(formatted, "local  x  =  1\nlocal y = 2\n");

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
        client.sender.send(initialize(1, None)).unwrap();
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

        main_loop(server, false, &mut config_resolver).unwrap();

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
        client.sender.send(initialize(1, None)).unwrap();
        client.sender.send(initialized()).unwrap();
        client
            .sender
            .send(format_document(
                2,
                uri.clone(),
                FormattingOptions::default(),
            ))
            .unwrap();
        client.sender.send(shutdown(3)).unwrap();
        client.sender.send(exit()).unwrap();

        main_loop(server, false, &mut config_resolver).unwrap();

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

    #[test]
    fn test_lsp_respects_configuration_in_root_path() {
        let contents = "local x = \"hello\"";
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "foo.lua": contents,
        });

        let uri = Uri::from_str(cwd.child("foo.lua").to_str().unwrap()).unwrap();

        lsp_test!(
            [],
            [
                initialize(1, Some(cwd.path())),
                initialized(),
                open_text_document(uri.clone(), contents.to_string()),
                format_document(2, uri.clone(), FormattingOptions::default()),
                shutdown(3),
                exit()
            ],
            [
                |receiver| expect_server_initialized(receiver, 1),
                |receiver| {
                    let edits: Vec<TextEdit> = expect_response(receiver, 2);
                    let formatted = apply_text_edits_to(contents, edits);
                    assert_eq!(formatted, "local x = 'hello'\n");
                },
                |receiver| expect_server_shutdown(receiver, 3)
            ]
        );
    }

    #[test]
    fn test_lsp_cwd_configuration_respected_for_nested_file() {
        let contents = "local x = \"hello\"";
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": contents,
        });

        let uri = Uri::from_str(cwd.child("foo.lua").to_str().unwrap()).unwrap();

        lsp_test!(
            [],
            [
                initialize(1, Some(cwd.path())),
                initialized(),
                open_text_document(uri.clone(), contents.to_string()),
                format_document(2, uri.clone(), FormattingOptions::default()),
                shutdown(3),
                exit()
            ],
            [
                |receiver| expect_server_initialized(receiver, 1),
                |receiver| {
                    let edits: Vec<TextEdit> = expect_response(receiver, 2);
                    let formatted = apply_text_edits_to(contents, edits);
                    assert_eq!(formatted, "local x = 'hello'\n");
                },
                |receiver| expect_server_shutdown(receiver, 3)
            ]
        );
    }

    #[test]
    fn test_lsp_configuration_is_not_used_outside_of_cwd() {
        let contents = "local x = \"hello\"";
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": contents,
        });

        let cwd = cwd.child("build");
        let uri = Uri::from_str(cwd.child("foo.lua").to_str().unwrap()).unwrap();

        lsp_test!(
            [],
            [
                initialize(1, Some(cwd.path())),
                initialized(),
                open_text_document(uri.clone(), contents.to_string()),
                format_document(2, uri.clone(), FormattingOptions::default()),
                shutdown(3),
                exit()
            ],
            [
                |receiver| expect_server_initialized(receiver, 1),
                |receiver| {
                    let edits: Vec<TextEdit> = expect_response(receiver, 2);
                    let formatted = apply_text_edits_to(contents, edits);
                    assert_eq!(formatted, "local x = \"hello\"\n");
                },
                |receiver| expect_server_shutdown(receiver, 3)
            ]
        );
    }

    #[test]
    fn test_lsp_configuration_used_outside_of_cwd_when_search_parent_directories_is_enabled() {
        let contents = "local x = \"hello\"";
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": contents,
        });

        let cwd = cwd.child("build");
        let uri = Uri::from_str(cwd.child("foo.lua").to_str().unwrap()).unwrap();

        lsp_test!(
            ["--search-parent-directories"],
            [
                initialize(1, Some(cwd.path())),
                initialized(),
                open_text_document(uri.clone(), contents.to_string()),
                format_document(2, uri.clone(), FormattingOptions::default()),
                shutdown(3),
                exit()
            ],
            [
                |receiver| expect_server_initialized(receiver, 1),
                |receiver| {
                    let edits: Vec<TextEdit> = expect_response(receiver, 2);
                    let formatted = apply_text_edits_to(contents, edits);
                    assert_eq!(formatted, "local x = 'hello'\n");
                },
                |receiver| expect_server_shutdown(receiver, 3)
            ]
        );
    }

    #[test]
    fn test_lsp_configuration_is_searched_next_to_file() {
        let contents = "local x = \"hello\"";
        let cwd = construct_tree!({
            "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": contents,
        });

        let uri = Uri::from_str(cwd.child("build/foo.lua").to_str().unwrap()).unwrap();

        lsp_test!(
            [],
            [
                initialize(1, Some(cwd.path())),
                initialized(),
                open_text_document(uri.clone(), contents.to_string()),
                format_document(2, uri.clone(), FormattingOptions::default()),
                shutdown(3),
                exit()
            ],
            [
                |receiver| expect_server_initialized(receiver, 1),
                |receiver| {
                    let edits: Vec<TextEdit> = expect_response(receiver, 2);
                    let formatted = apply_text_edits_to(contents, edits);
                    assert_eq!(formatted, "local x = 'hello'\n");
                },
                |receiver| expect_server_shutdown(receiver, 3)
            ]
        );
    }

    #[test]
    fn test_lsp_does_not_use_editor_formatting_options() {
        let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();
        let contents = "do print(1) end";

        lsp_test!(
            [],
            [
                initialize_with_options(
                    1,
                    InitializationOptions {
                        respect_editor_formatting_options: None,
                    }
                ),
                initialized(),
                open_text_document(uri.clone(), contents.to_string()),
                format_document(
                    2,
                    uri.clone(),
                    FormattingOptions {
                        tab_size: 2,
                        insert_spaces: true,
                        ..Default::default()
                    }
                ),
                shutdown(3),
                exit()
            ],
            [
                |receiver| expect_server_initialized(receiver, 1),
                |receiver| {
                    let edits: Vec<TextEdit> = expect_response(receiver, 2);
                    let formatted = apply_text_edits_to(contents, edits);
                    assert_eq!(formatted, "do\n\tprint(1)\nend\n");
                },
                |receiver| expect_server_shutdown(receiver, 3)
            ]
        );
    }

    #[test]
    fn test_lsp_respects_editor_formatting_options_if_enabled() {
        let uri = Uri::from_str("file:///home/documents/file.luau").unwrap();
        let contents = "do print(1) end";

        lsp_test!(
            [],
            [
                initialize_with_options(
                    1,
                    InitializationOptions {
                        respect_editor_formatting_options: Some(true)
                    }
                ),
                initialized(),
                open_text_document(uri.clone(), contents.to_string()),
                format_document(
                    2,
                    uri.clone(),
                    FormattingOptions {
                        tab_size: 2,
                        insert_spaces: true,
                        ..Default::default()
                    }
                ),
                shutdown(3),
                exit()
            ],
            [
                |receiver| expect_server_initialized(receiver, 1),
                |receiver| {
                    let edits: Vec<TextEdit> = expect_response(receiver, 2);
                    let formatted = apply_text_edits_to(contents, edits);
                    assert_eq!(formatted, "do\n  print(1)\nend\n");
                },
                |receiver| expect_server_shutdown(receiver, 3)
            ]
        );
    }

    #[test]
    fn test_lsp_stylua_ignore() {
        let contents = "local   x    =   1";
        let cwd = construct_tree!({
            ".styluaignore": "ignored/",
            "foo.lua": contents,
            "ignored/bar.lua": contents,
        });

        let foo_uri = Uri::from_str(cwd.child("foo.lua").to_str().unwrap()).unwrap();
        let bar_uri = Uri::from_str(cwd.child("ignored/bar.lua").to_str().unwrap()).unwrap();

        lsp_test!(
            [],
            [
                initialize(1, Some(cwd.path())),
                initialized(),
                open_text_document(foo_uri.clone(), contents.to_string()),
                open_text_document(bar_uri.clone(), contents.to_string()),
                format_document(2, foo_uri.clone(), FormattingOptions::default()),
                format_document(3, bar_uri.clone(), FormattingOptions::default()),
                shutdown(4),
                exit()
            ],
            [
                |receiver| expect_server_initialized(receiver, 1),
                |receiver| {
                    let edits: Vec<TextEdit> = expect_response(receiver, 2);
                    let formatted = apply_text_edits_to(contents, edits);
                    assert_eq!(formatted, "local x = 1\n");
                },
                |receiver| {
                    let edits: serde_json::Value = expect_response(receiver, 3);
                    assert_eq!(edits, serde_json::Value::Null);
                },
                |receiver| expect_server_shutdown(receiver, 4)
            ]
        );
    }
}
