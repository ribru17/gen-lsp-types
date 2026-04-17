mod generated;

pub use generated::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum Version {
    #[serde(rename = "2.0")]
    TwoPointZero,
}

/// A unique ID used to correlate requests and responses together.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Id {
    /// Numeric ID.
    Number(i64),
    /// String ID.
    String(String),
    /// Null ID.
    ///
    /// The use of Null as a value for the id member in a Request object is discouraged, because
    /// this specification uses a value of Null for Responses with an unknown id. Also, because
    /// JSON-RPC 1.0 uses an id value of Null for Notifications this could cause confusion in
    /// handling.
    Null,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
/// A JSON-RPC request or notification.
pub struct RequestObject {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    jsonrpc: Version,
    /// An identifier established by the Client that MUST contain a String, Number, or NULL value if
    /// included. If it is not included it is assumed to be a notification. The value SHOULD
    /// normally not be Null and Numbers SHOULD NOT contain fractional parts.
    #[serde(default, deserialize_with = "deserialize_some")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Id>,
    /// A String containing the name of the method to be invoked. Method names that begin with the
    /// word rpc followed by a period character (U+002E or ASCII 46) are reserved for rpc-internal
    /// methods and extensions and MUST NOT be used for anything else.
    #[serde(default)]
    method: String,
    /// A Structured value that holds the parameter values to be used during the invocation of the
    /// method. This member MAY be omitted.
    ///
    /// If present, parameters for the rpc call MUST be provided as a Structured value. Either
    /// by-position through an Array or by-name through an Object.
    #[serde(default, deserialize_with = "deserialize_some")]
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
}

impl RequestObject {
    /// Creates a JSON-RPC Request object from a server request.
    pub fn from_request<R>(id: Id, params: R::Params) -> Self
    where
        R: Request,
    {
        let params = serde_json::to_value(params).expect("Invalid request params");
        let params = match params {
            Value::Null => None,
            Value::Array(_) | Value::Object(_) => Some(params),
            _ => panic!("Parameters must be an object or array, if not omitted."),
        };
        RequestObject {
            jsonrpc: Version::TwoPointZero,
            id: Some(id),
            params,
            method: R::METHOD.to_string(),
        }
    }

    /// Creates a JSON-RPC Request object from a server notification.
    pub fn from_notification<N>(params: N::Params) -> Self
    where
        N: Notification,
    {
        let params = serde_json::to_value(params).expect("Invalid request params");
        let params = match params {
            Value::Null => None,
            Value::Array(_) | Value::Object(_) => Some(params),
            _ => panic!("Parameters must be an object or array, if not omitted."),
        };
        RequestObject {
            jsonrpc: Version::TwoPointZero,
            method: N::METHOD.to_string(),
            params,
            id: None,
        }
    }

    /// Returns the method to be invoked.
    pub fn method(&self) -> &str {
        self.method.as_ref()
    }

    /// Returns the id.
    pub fn id(&self) -> Option<&Id> {
        self.id.as_ref()
    }

    /// Returns the params.
    pub fn params(&self) -> Option<&Value> {
        self.params.as_ref()
    }

    /// Splits the request into the method name, request ID, and the parameters.
    pub fn into_parts(self) -> (String, Option<Id>, Option<Value>) {
        (self.method, self.id, self.params)
    }
}

#[cfg(test)]
mod test {
    #![allow(deprecated)]
    use std::collections::{HashMap, HashSet};

    use crate::{
        ColorPresentation, CreateFile, DeleteFile, DocumentChange, DocumentSymbol,
        ExitNotification, FoldingRangeKind, Id, InitializeParams, InitializedNotification,
        InitializedParams, LspNotificationMethods, LspRequestMethods, MarkupKind, Position, Range,
        RequestObject, SymbolKind, TextDocumentRegistrationOptions, TypeDefinitionParams,
        TypeDefinitionRequest, WatchKind, WorkDoneProgressEnd, WorkspaceFoldersInitializeParams,
        WorkspaceFoldersRequest, WorkspaceFoldersServerCapabilities,
    };

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

        assert_eq!(wfip_str, r#"{}"#);

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
            name: Default::default(),
            detail: Default::default(),
            tags: Default::default(),
            deprecated: Default::default(),
            range: Default::default(),
            selection_range: Default::default(),
            children: Default::default(),
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

        let method = LspRequestMethods::TextDocumentOnTypeFormatting.to_string();
        assert_eq!(method, "textDocument/onTypeFormatting");
        let method = LspRequestMethods::Shutdown.to_string();
        assert_eq!(method, "shutdown");
        let method = LspRequestMethods::Custom("foo".into()).to_string();
        assert_eq!(method, "foo");
        let method = LspNotificationMethods::Custom("foo".into()).to_string();
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
            uri: "file:///foo.txt".to_string(),
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
                uri: "file:///foo.txt".to_string(),
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

        let frk = FoldingRangeKind::Custom("foo".into());
        let ser = serde_json::to_string(&frk).unwrap();

        assert_eq!(ser, "\"foo\"");
        assert_eq!(
            serde_json::from_str::<FoldingRangeKind>(&ser).unwrap(),
            FoldingRangeKind::Custom("foo".into())
        );

        let mk = MarkupKind::PlainText;
        assert_eq!("\"plaintext\"", serde_json::to_string(&mk).unwrap());
        assert!(serde_json::from_str::<MarkupKind>("foo").is_err());
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
        let req =
            RequestObject::from_request::<TypeDefinitionRequest>(Id::Number(123), params.clone());

        let ser = serde_json::to_string(&req).unwrap();

        assert_eq!(
            ser,
            r#"{"jsonrpc":"2.0","id":123,"method":"textDocument/typeDefinition","params":{"position":{"character":0,"line":0},"textDocument":{"uri":"foo"}}}"#
        );
        assert_eq!(req.id(), Some(&Id::Number(123)));
        assert_eq!(req.method(), "textDocument/typeDefinition");
        assert_eq!(req.params(), Some(&serde_json::to_value(params).unwrap()));
    }

    #[test]
    fn request_object_from_request_no_params() {
        let req =
            RequestObject::from_request::<WorkspaceFoldersRequest>(Id::String("foo".into()), ());

        let ser = serde_json::to_string(&req).unwrap();

        assert_eq!(
            ser,
            r#"{"jsonrpc":"2.0","id":"foo","method":"workspace/workspaceFolders"}"#
        );
    }

    #[test]
    fn request_object_from_notification() {
        let noti =
            RequestObject::from_notification::<InitializedNotification>(InitializedParams {});

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
        let noti = RequestObject::from_notification::<ExitNotification>(());

        let ser = serde_json::to_string(&noti).unwrap();

        assert_eq!(ser, r#"{"jsonrpc":"2.0","method":"exit"}"#);
    }
}
