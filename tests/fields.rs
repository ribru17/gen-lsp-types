use gen_lsp_types::TextDocumentRegistrationOptions;

#[test]
fn nullable_fields() {
    let tdro = TextDocumentRegistrationOptions {
        document_selector: None,
    };

    let tdro_str = serde_json::to_string(&tdro).unwrap();

    assert_eq!(tdro_str, r#"{"documentSelector":null}"#);
}
