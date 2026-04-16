#![allow(deprecated)]
use std::collections::{HashMap, HashSet};

use gen_lsp_types::{
    ColorPresentation, DocumentSymbol, InitializeParams, Position, Range,
    TextDocumentRegistrationOptions, WorkspaceFoldersInitializeParams,
    WorkspaceFoldersServerCapabilities,
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

    // Missing processId.
    let bad_ip_str = r#"{"rootUri":null,"capabilities":{}}"#;

    let bad_ip = serde_json::from_str::<InitializeParams>(bad_ip_str);

    assert!(bad_ip.is_err());
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
        workspace_folders: Some(gen_lsp_types::WorkspaceFolders::Null),
    };
    let wfip_str = serde_json::to_string(&wfip).unwrap();
    assert_eq!(wfip_str, r#"{"workspaceFolders":null}"#);
    assert_eq!(
        serde_json::from_str::<WorkspaceFoldersInitializeParams>(&wfip_str).unwrap(),
        wfip
    );

    let wfip = WorkspaceFoldersInitializeParams {
        workspace_folders: Some(gen_lsp_types::WorkspaceFolders::WorkspaceFolderList(
            Vec::new(),
        )),
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
        kind: gen_lsp_types::SymbolKind::Function,
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
}
