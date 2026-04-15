use gen_lsp_types::{
    ColorPresentation, InitializeParams, Or2, TextDocumentRegistrationOptions,
    WorkspaceFoldersInitializeParams,
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
        workspace_folders: Some(Or2::U(())),
    };

    let wfip_str = serde_json::to_string(&wfip).unwrap();

    assert_eq!(wfip_str, r#"{"workspaceFolders":null}"#);

    assert_eq!(
        serde_json::from_str::<WorkspaceFoldersInitializeParams>(&wfip_str).unwrap(),
        wfip
    );
}
