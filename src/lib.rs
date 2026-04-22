mod generated;
pub mod json_rpc;

pub use generated::*;

/// Tests for default features.
#[cfg(test)]
#[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
mod test {
    #![allow(deprecated)]
    use std::{
        borrow::Cow,
        collections::{HashMap, HashSet},
    };

    use crate::*;

    #[test]
    fn nullable_field() {
        let tdro = TextDocumentRegistrationOptions {
            document_selector: None,
        };
        let tdro_str = serde_json::to_string(&tdro).unwrap();
        assert_eq!(tdro_str, r#"{"documentSelector":null}"#);

        let tdro = serde_json::from_str::<TextDocumentRegistrationOptions>(&tdro_str).unwrap();
        assert_eq!(
            tdro,
            TextDocumentRegistrationOptions {
                document_selector: None
            }
        );

        // Missing documentSelector. Be liberal on deserialization, even though this is technically a
        // noncompliant representation.
        assert_eq!(
            serde_json::from_str::<TextDocumentRegistrationOptions>("{}").unwrap(),
            tdro
        );
    }

    #[test]
    fn nullable_field_default() {
        let ip = InitializeParams::default();

        let ip_str = serde_json::to_string(&ip).unwrap();

        assert_eq!(
            ip_str,
            r#"{"processId":null,"rootUri":null,"capabilities":{}}"#
        );

        let ip = serde_json::from_str::<InitializeParams>(&ip_str).unwrap();

        assert_eq!(ip, InitializeParams::default());

        // Missing processId. Be liberal on deserialization, even though this is technically a
        // noncompliant representation.
        let bad_ip_str = r#"{"rootUri":null,"capabilities":{}}"#;
        let bad_ip = serde_json::from_str::<InitializeParams>(bad_ip_str);
        assert_eq!(bad_ip.unwrap(), InitializeParams::default());
    }

    #[test]
    fn optional_field() {
        let cp = ColorPresentation {
            label: "Label".to_string(),
            text_edit: None,
            ..Default::default()
        };
        let cp_str = serde_json::to_string(&cp).unwrap();
        assert_eq!(cp_str, r#"{"label":"Label"}"#);

        let cp = serde_json::from_str::<ColorPresentation>(&cp_str).unwrap();
        assert_eq!(
            cp,
            ColorPresentation {
                label: "Label".to_string(),
                text_edit: None,
                ..Default::default()
            }
        );
        // While technically illegal in the spec, this library chooses to be more liberal with
        // DEserialization, in order to be tolerant of slightly spec-noncompliant communicators.
        assert_eq!(
            serde_json::from_str::<ColorPresentation>(r#"{"label":"Label","textEdit":null}"#)
                .unwrap(),
            cp
        );
    }

    #[test]
    fn optional_nullable_field() {
        let wfip = WorkspaceFoldersInitializeParams {
            workspace_folders: None,
        };

        let wfip_str = serde_json::to_string(&wfip).unwrap();

        assert_eq!(wfip_str, r"{}");

        assert_eq!(
            serde_json::from_str::<WorkspaceFoldersInitializeParams>(&wfip_str).unwrap(),
            wfip
        );

        let wfip = WorkspaceFoldersInitializeParams {
            workspace_folders: Some(crate::WorkspaceFolders::Null),
        };
        let wfip_str = serde_json::to_string(&wfip).unwrap();
        assert_eq!(wfip_str, r#"{"workspaceFolders":null}"#);
        assert_eq!(
            serde_json::from_str::<WorkspaceFoldersInitializeParams>(&wfip_str).unwrap(),
            wfip
        );

        let wfip = WorkspaceFoldersInitializeParams {
            workspace_folders: Some(crate::WorkspaceFolders::WorkspaceFolderList(Vec::new())),
        };
        let wfip_str = serde_json::to_string(&wfip).unwrap();
        assert_eq!(wfip_str, r#"{"workspaceFolders":[]}"#);
        assert_eq!(
            serde_json::from_str::<WorkspaceFoldersInitializeParams>(&wfip_str).unwrap(),
            wfip
        );
    }

    #[test]
    fn derives() {
        let pos = Position::default();
        let table = HashMap::from([(pos, 123)]);
        assert_eq!(table.get(&pos), Some(&123));

        let range = Range::default();
        let table = HashSet::from([range]);
        assert!(table.contains(&range));

        let doc_sym = DocumentSymbol {
            kind: crate::SymbolKind::Function,
            name: String::default(),
            detail: Option::default(),
            tags: Option::default(),
            deprecated: Option::default(),
            range: Range::default(),
            selection_range: Range::default(),
            children: Option::default(),
        };
        let table = HashSet::from([doc_sym.clone()]);
        assert!(table.contains(&doc_sym));

        // From
        let wfip = WorkspaceFoldersInitializeParams {
            workspace_folders: Some(Vec::new().into()),
        };
        let wfip_str = serde_json::to_string(&wfip).unwrap();
        assert_eq!(wfip_str, r#"{"workspaceFolders":[]}"#);
        assert_eq!(
            serde_json::from_str::<WorkspaceFoldersInitializeParams>(&wfip_str).unwrap(),
            wfip
        );
        assert_eq!(WorkspaceFolders::Null, ().into());
        let wfip = WorkspaceFoldersInitializeParams {
            workspace_folders: Some(().into()),
        };
        let wfip_str = serde_json::to_string(&wfip).unwrap();
        assert_eq!(wfip_str, r#"{"workspaceFolders":null}"#);
        assert_eq!(
            serde_json::from_str::<WorkspaceFoldersInitializeParams>(&wfip_str).unwrap(),
            wfip
        );
        let wfsc = WorkspaceFoldersServerCapabilities {
            change_notifications: Some("some-noti-id".into()),
            ..Default::default()
        };
        let wfsc_str = serde_json::to_string(&wfsc).unwrap();
        assert_eq!(wfsc_str, r#"{"changeNotifications":"some-noti-id"}"#);
        let wfsc = WorkspaceFoldersServerCapabilities {
            change_notifications: Some(String::from("some-noti-id").into()),
            ..Default::default()
        };
        let wfsc_str = serde_json::to_string(&wfsc).unwrap();
        assert_eq!(wfsc_str, r#"{"changeNotifications":"some-noti-id"}"#);
        let wfsc = WorkspaceFoldersServerCapabilities {
            change_notifications: Some(false.into()),
            ..Default::default()
        };
        let wfsc_str = serde_json::to_string(&wfsc).unwrap();
        assert_eq!(wfsc_str, r#"{"changeNotifications":false}"#);
        let wfsc = WorkspaceFoldersServerCapabilities {
            change_notifications: Some('f'.into()),
            ..Default::default()
        };
        let wfsc_str = serde_json::to_string(&wfsc).unwrap();
        assert_eq!(wfsc_str, r#"{"changeNotifications":"f"}"#);
        let boxed_str: Box<str> = Box::from("foo");
        let wfsc = WorkspaceFoldersServerCapabilities {
            change_notifications: Some(boxed_str.into()),
            ..Default::default()
        };
        let wfsc_str = serde_json::to_string(&wfsc).unwrap();
        assert_eq!(wfsc_str, r#"{"changeNotifications":"foo"}"#);
    }

    #[test]
    fn special_derives() {
        let pos = Position {
            line: 2,
            character: 0,
        };
        let pos2 = Position {
            line: 1,
            character: 9,
        };
        assert!(pos2 < pos);

        // Copy
        let pos3 = pos2;
        assert_eq!(pos3, pos2);

        let range = Range {
            start: pos2,
            end: pos,
        };
        // Copy
        let range2 = range;
        assert_eq!(range2, range);

        let range3 = Range {
            start: Position::default(),
            end: Position {
                line: 999,
                character: 999,
            },
        };
        assert!(range3 < range2);

        let range4 = Range::default();
        assert!(range4 < range3);

        let method: String = LspRequestMethods::TextDocumentOnTypeFormatting.into();
        assert_eq!(method, "textDocument/onTypeFormatting");
        let method = LspRequestMethods::Shutdown.to_string();
        assert_eq!(method, "shutdown");
        let method = LspRequestMethods::Custom(Cow::Borrowed("foo")).to_string();
        assert_eq!(method, "foo");
        let method = LspNotificationMethods::Custom(Cow::Borrowed("foo")).to_string();
        assert_eq!(method, "foo");
        let method = LspNotificationMethods::CancelRequest.to_string();
        assert_eq!(method, "$/cancelRequest");
        let method = LspNotificationMethods::WorkspaceDidChangeWatchedFiles.to_string();
        assert_eq!(method, "workspace/didChangeWatchedFiles");
    }

    #[test]
    fn string_literal_field() {
        let wdpe = WorkDoneProgressEnd {
            message: Some("change da world. my final message. goodbye".to_string()),
        };

        let ser = serde_json::to_string(&wdpe).unwrap();
        assert_eq!(
            ser,
            r#"{"message":"change da world. my final message. goodbye","kind":"end"}"#
        );

        let deser = serde_json::from_str::<WorkDoneProgressEnd>(&ser).unwrap();
        assert_eq!(deser, wdpe);

        let fake_ser = r#"{"message":"change da world. my final message. goodbye","kind":"begin"}"#;
        assert!(serde_json::from_str::<WorkDoneProgressEnd>(fake_ser).is_err());

        let doc_change = CreateFile {
            uri: "file:///foo.txt".to_string().into(),
            options: None,
            annotation_id: None,
        };
        let ser = serde_json::to_string(&doc_change).unwrap();
        assert_eq!(ser, r#"{"uri":"file:///foo.txt","kind":"create"}"#);

        let ser = r#"{"uri":"file:///foo.txt","kind":"create"}"#;
        let deser = serde_json::from_str::<DocumentChange>(ser).unwrap();
        assert_eq!(deser, doc_change.into());
        let ser = r#"{"uri":"file:///foo.txt","kind":"delete","annotationId":"foo"}"#;
        let deser = serde_json::from_str::<DocumentChange>(ser).unwrap();
        assert_eq!(
            deser,
            DocumentChange::DeleteFile(DeleteFile {
                uri: crate::Uri("file:///foo.txt".to_string()),
                options: None,
                annotation_id: Some(String::from("foo"))
            })
        );
        let bad_ser = r#"{"uri":"file:///foo.txt","kind":"delet"}"#;
        assert!(serde_json::from_str::<DocumentChange>(bad_ser).is_err());
    }

    #[test]
    fn string_enum() {
        let frk = FoldingRangeKind::Comment;
        let ser = serde_json::to_string(&frk).unwrap();

        assert_eq!(ser, "\"comment\"");
        assert_eq!(
            serde_json::from_str::<FoldingRangeKind>(&ser).unwrap(),
            FoldingRangeKind::Comment
        );

        let frk = FoldingRangeKind::Custom(Cow::Borrowed("foo"));
        let ser = serde_json::to_string(&frk).unwrap();

        assert_eq!(ser, "\"foo\"");
        assert_eq!(
            serde_json::from_str::<FoldingRangeKind>(&ser).unwrap(),
            FoldingRangeKind::Custom(Cow::Borrowed("foo"))
        );

        let mk = MarkupKind::PlainText;
        assert_eq!("\"plaintext\"", serde_json::to_string(&mk).unwrap());
        assert!(serde_json::from_str::<MarkupKind>("foo").is_err());
    }

    #[test]
    fn str_enum_into() {
        const CONSTANT: &str = "my_custom_variant";
        let _parsed: LspNotificationMethods = CONSTANT.into();
    }

    #[test]
    fn int_enum() {
        let sk = SymbolKind::Namespace;
        let ser = serde_json::to_string(&sk).unwrap();

        assert_eq!(ser, "3");
        assert_eq!(
            serde_json::from_str::<SymbolKind>(&ser).unwrap(),
            SymbolKind::Namespace
        );
        assert!(serde_json::from_str::<SymbolKind>("299").is_err());

        let wk = WatchKind::Custom(123);
        let ser = serde_json::to_string(&wk).unwrap();

        assert_eq!(ser, "123");
        assert_eq!(wk, serde_json::from_str::<WatchKind>(&ser).unwrap());
        assert_eq!(
            WatchKind::Change,
            serde_json::from_str::<WatchKind>("2").unwrap()
        );
    }

    #[test]
    fn request_object_from_request() {
        let params = TypeDefinitionParams {
            work_done_progress_params: crate::WorkDoneProgressParams {
                work_done_token: None,
            },
            partial_result_params: crate::PartialResultParams {
                partial_result_token: None,
            },
            text_document_position_params: crate::TextDocumentPositionParams {
                text_document: crate::TextDocumentIdentifier { uri: "foo".into() },
                position: Position::default(),
            },
        };
        let req = json_rpc::RequestObject::from_request::<TypeDefinitionRequest>(
            json_rpc::Id::Number(123),
            params.clone(),
        );

        let ser = serde_json::to_string(&req).unwrap();

        assert_eq!(
            ser,
            r#"{"jsonrpc":"2.0","id":123,"method":"textDocument/typeDefinition","params":{"position":{"character":0,"line":0},"textDocument":{"uri":"foo"}}}"#
        );
        assert_eq!(req.id(), Some(&json_rpc::Id::Number(123)));
        assert_eq!(req.method(), "textDocument/typeDefinition");
        assert_eq!(req.params(), Some(&serde_json::to_value(params).unwrap()));
    }

    #[test]
    fn request_object_from_request_no_params() {
        let req = json_rpc::RequestObject::from_request::<WorkspaceFoldersRequest>(
            json_rpc::Id::String("foo".into()),
            (),
        );

        let ser = serde_json::to_string(&req).unwrap();

        assert_eq!(
            ser,
            r#"{"jsonrpc":"2.0","id":"foo","method":"workspace/workspaceFolders"}"#
        );
    }

    #[test]
    fn request_object_from_notification() {
        let noti = json_rpc::RequestObject::from_notification::<InitializedNotification>(
            InitializedParams {},
        );

        let ser = serde_json::to_string(&noti).unwrap();

        assert_eq!(
            ser,
            r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#
        );
        assert_eq!(noti.id(), None);
        assert_eq!(noti.method(), "initialized");
        assert_eq!(
            noti.params(),
            Some(&serde_json::to_value(InitializedParams {}).unwrap())
        );
    }

    #[test]
    fn request_object_from_notification_no_params() {
        let noti = json_rpc::RequestObject::from_notification::<ExitNotification>(());

        let ser = serde_json::to_string(&noti).unwrap();

        assert_eq!(ser, r#"{"jsonrpc":"2.0","method":"exit"}"#);
    }

    #[test]
    fn response_object_from_success() {
        let id = json_rpc::Id::Number(123);

        let res = json_rpc::ResponseObject::from_success::<ImplementationRequest>(
            id.clone(),
            Some(ImplementationResponse::DefinitionLinkList(Vec::new())),
        );

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(r#"{"jsonrpc":"2.0","result":[],"id":123}"#, &ser);
        assert_eq!(res, serde_json::from_str(&ser).unwrap());

        let res = json_rpc::ResponseObject::from_success::<ImplementationRequest>(id.clone(), None);

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(r#"{"jsonrpc":"2.0","result":null,"id":123}"#, &ser);
        assert_eq!(res, serde_json::from_str(&ser).unwrap());

        let res = json_rpc::ResponseObject::from_success::<ImplementationRequest>(id.clone(), None);

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(r#"{"jsonrpc":"2.0","result":null,"id":123}"#, &ser);
        assert_eq!(res, serde_json::from_str(&ser).unwrap());

        let res = json_rpc::ResponseObject::from_success::<ShowMessageRequest>(id.clone(), None);

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(r#"{"jsonrpc":"2.0","result":null,"id":123}"#, &ser);
        assert_eq!(res, serde_json::from_str(&ser).unwrap());

        let res = json_rpc::ResponseObject::from_success::<ShowMessageRequest>(
            id.clone(),
            Some(crate::MessageActionItem {
                title: "foo".into(),
            }),
        );

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(
            r#"{"jsonrpc":"2.0","result":{"title":"foo"},"id":123}"#,
            &ser
        );
        assert_eq!(res, serde_json::from_str(&ser).unwrap());

        let res = json_rpc::ResponseObject::from_success::<CodeLensRefreshRequest>(id, ());

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(r#"{"jsonrpc":"2.0","result":null,"id":123}"#, &ser);
        assert_eq!(res, serde_json::from_str(&ser).unwrap());
    }

    #[test]
    fn response_object_from_error() {
        let id = json_rpc::Id::Null;
        let res = json_rpc::ResponseObject::from_error(
            id,
            json_rpc::Error {
                code: crate::ErrorCodes::ParseError,
                message: "invalid format".into(),
                data: None,
            },
        );

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(
            r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"invalid format"},"id":null}"#,
            &ser
        );

        let id = json_rpc::Id::String("foo-req".into());
        let res = json_rpc::ResponseObject::from_error(
            id,
            json_rpc::Error {
                code: crate::ErrorCodes::Custom(-32803),
                message: "failed to foo the bar".into(),
                data: Some(serde_json::to_value(String::from("hi")).unwrap()),
            },
        );

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(
            r#"{"jsonrpc":"2.0","error":{"code":-32803,"message":"failed to foo the bar","data":"hi"},"id":"foo-req"}"#,
            &ser
        );

        let id = json_rpc::Id::String("foo-req".into());
        let res = json_rpc::ResponseObject::from_error(
            id,
            json_rpc::Error {
                code: crate::ErrorCodes::Custom(crate::LspErrorCodes::ContentModified.into()),
                message: "failed to foo the bar".into(),
                data: Some(serde_json::to_value(String::from("hi")).unwrap()),
            },
        );

        let ser = serde_json::to_string(&res).unwrap();
        assert_eq!(
            r#"{"jsonrpc":"2.0","error":{"code":-32801,"message":"failed to foo the bar","data":"hi"},"id":"foo-req"}"#,
            &ser
        );
    }

    #[test]
    fn structures_new_constructor() {
        assert_eq!(
            Position::new(1, 2),
            Position {
                line: 1,
                character: 2
            }
        );
        assert_eq!(
            Range::new(Position::new(0, 0), Position::new(0, 99)),
            Range {
                start: Position::default(),
                end: Position {
                    line: 0,
                    character: 99
                }
            }
        );

        assert_eq!(
            Location::new(Uri("foo".into()), Range::default()),
            Location {
                uri: Uri("foo".into()),
                range: Range::default()
            }
        );

        // For some structures, the new() constructor does not help as much.
        assert_eq!(
            Diagnostic::new(
                Range::default(),
                Some(DiagnosticSeverity::Warning),
                None,
                None,
                None,
                "bad".into(),
                None,
                None,
                None
            ),
            Diagnostic {
                range: Range::default(),
                message: "bad".into(),
                severity: Some(DiagnosticSeverity::Warning),
                ..Default::default()
            }
        );
    }

    #[test]
    fn custom_request_object_methods() {
        struct ParentModule;
        impl Request for ParentModule {
            type Params = ();
            type Result = Option<DefinitionResponse>;
            const METHOD: LspRequestMethods =
                LspRequestMethods::Custom(Cow::Borrowed("experimental/parentModule"));
            const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
        }

        let req =
            json_rpc::RequestObject::from_request::<ParentModule>(json_rpc::Id::Number(123), ());
        assert_eq!(
            r#"{"jsonrpc":"2.0","id":123,"method":"experimental/parentModule"}"#,
            serde_json::to_string(&req).unwrap()
        );

        struct ServerStatusNotification;
        impl Notification for ServerStatusNotification {
            type Params = ();
            const METHOD: LspNotificationMethods =
                LspNotificationMethods::Custom(Cow::Borrowed("experimental/serverStatus"));
            const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
        }

        let noti = json_rpc::RequestObject::from_notification::<ServerStatusNotification>(());
        assert_eq!(
            r#"{"jsonrpc":"2.0","method":"experimental/serverStatus"}"#,
            serde_json::to_string(&noti).unwrap()
        );
    }

    #[test]
    fn semantic_tokens() {
        let ste = SemanticTokensEdit {
            start: 0,
            delete_count: 1,
            data: None,
        };
        let ste_ser = r#"{"start":0,"deleteCount":1}"#;
        assert_eq!(serde_json::to_string(&ste).unwrap(), ste_ser);
        assert_eq!(ste, serde_json::from_str(ste_ser).unwrap());

        let ste = SemanticTokensEdit {
            start: 0,
            delete_count: 1,
            data: None,
        };
        let ste_ser_fake = r#"{"start":0,"deleteCount":1,"data":null}"#;
        assert_eq!(serde_json::to_string(&ste).unwrap(), ste_ser);
        // Be permissive on technically incorrect deserialization.
        assert_eq!(ste, serde_json::from_str(ste_ser_fake).unwrap());

        let ste = SemanticTokensEdit {
            start: 0,
            delete_count: 1,
            data: Some(vec![
                SemanticToken {
                    delta_line: 2,
                    delta_start: 5,
                    length: 3,
                    token_type: 0,
                    token_modifiers_bitset: 3,
                },
                SemanticToken {
                    delta_line: 0,
                    delta_start: 5,
                    length: 4,
                    token_type: 1,
                    token_modifiers_bitset: 0,
                },
            ]),
        };
        let ste_ser = r#"{"start":0,"deleteCount":1,"data":[2,5,3,0,3,0,5,4,1,0]}"#;
        assert_eq!(serde_json::to_string(&ste).unwrap(), ste_ser);
        assert_eq!(ste, serde_json::from_str(ste_ser).unwrap());

        let ste = SemanticTokensEdit {
            start: 0,
            delete_count: 1,
            data: Some(Vec::new()),
        };
        let ste_ser = r#"{"start":0,"deleteCount":1,"data":[]}"#;
        assert_eq!(serde_json::to_string(&ste).unwrap(), ste_ser);
        assert_eq!(ste, serde_json::from_str(ste_ser).unwrap());

        let st = SemanticTokens {
            result_id: None,
            data: Vec::default(),
        };
        let st_ser = r#"{"data":[]}"#;
        assert_eq!(serde_json::to_string(&st).unwrap(), st_ser);
        assert_eq!(st, serde_json::from_str(st_ser).unwrap());

        let st = SemanticTokens {
            result_id: None,
            data: vec![
                SemanticToken {
                    delta_line: 2,
                    delta_start: 5,
                    length: 3,
                    token_type: 0,
                    token_modifiers_bitset: 3,
                },
                SemanticToken {
                    delta_line: 0,
                    delta_start: 5,
                    length: 4,
                    token_type: 1,
                    token_modifiers_bitset: 0,
                },
            ],
        };
        let st_ser = r#"{"data":[2,5,3,0,3,0,5,4,1,0]}"#;
        assert_eq!(serde_json::to_string(&st).unwrap(), st_ser);
        assert_eq!(st, serde_json::from_str(st_ser).unwrap());

        let stpr = SemanticTokensPartialResult {
            data: vec![
                SemanticToken {
                    delta_line: 2,
                    delta_start: 5,
                    length: 3,
                    token_type: 0,
                    token_modifiers_bitset: 3,
                },
                SemanticToken {
                    delta_line: 0,
                    delta_start: 5,
                    length: 4,
                    token_type: 1,
                    token_modifiers_bitset: 0,
                },
            ],
        };
        let stpr_ser = r#"{"data":[2,5,3,0,3,0,5,4,1,0]}"#;
        assert_eq!(serde_json::to_string(&stpr).unwrap(), stpr_ser);
        assert_eq!(stpr, serde_json::from_str(stpr_ser).unwrap());

        let stpr = SemanticTokensPartialResult {
            data: Vec::default(),
        };
        let stpr_ser = r#"{"data":[]}"#;
        assert_eq!(serde_json::to_string(&stpr).unwrap(), stpr_ser);
        assert_eq!(stpr, serde_json::from_str(stpr_ser).unwrap());
    }
}

/// Tests for the "url" feature.
#[cfg(test)]
#[cfg(all(feature = "url", not(feature = "fluent-uri")))]
mod test {
    use crate::*;

    #[test]
    fn url_feature() {
        let url = url::Url::parse("file://tmp/foo.txt/").unwrap();
        let tdi = TextDocumentIdentifier { uri: url };
        let ser = r#"{"uri":"file://tmp/foo.txt/"}"#;

        assert_eq!(ser, serde_json::to_string(&tdi).unwrap());
        assert_eq!(tdi, serde_json::from_str(ser).unwrap());
    }
}

/// Tests for the "fluent-uri" feature.
#[cfg(test)]
#[cfg(all(feature = "fluent-uri", not(feature = "url")))]
mod test {
    use crate::*;

    #[test]
    fn url_feature() {
        let uri = fluent_uri::Uri::try_from("file://tmp/foo.txt/".to_string()).unwrap();
        let tdi = TextDocumentIdentifier { uri };
        let ser = r#"{"uri":"file://tmp/foo.txt/"}"#;

        assert_eq!(ser, serde_json::to_string(&tdi).unwrap());
        assert_eq!(tdi, serde_json::from_str(ser).unwrap());
    }
}
