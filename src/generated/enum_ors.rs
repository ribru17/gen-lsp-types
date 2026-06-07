use serde::{Deserialize, Serialize};
use std::borrow::Cow;
#[allow(clippy::wildcard_imports)]
use super::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum ActiveParameter {
    Int(u32),
    Null,
}
impl From<u32> for ActiveParameter {
    fn from(v: u32) -> Self {
        Self::Int(v)
    }
}
impl From<()> for ActiveParameter {
    fn from((): ()) -> Self {
        Self::Null
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum BaseUri {
    WorkspaceFolder(WorkspaceFolder),
    Uri(Uri),
}
impl From<WorkspaceFolder> for BaseUri {
    fn from(v: WorkspaceFolder) -> Self {
        Self::WorkspaceFolder(v)
    }
}
impl From<Uri> for BaseUri {
    fn from(v: Uri) -> Self {
        Self::Uri(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum CallHierarchyProvider {
    Bool(bool),
    CallHierarchyOptions(CallHierarchyOptions),
    CallHierarchyRegistrationOptions(CallHierarchyRegistrationOptions),
}
impl From<bool> for CallHierarchyProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<CallHierarchyOptions> for CallHierarchyProvider {
    fn from(v: CallHierarchyOptions) -> Self {
        Self::CallHierarchyOptions(v)
    }
}
impl From<CallHierarchyRegistrationOptions> for CallHierarchyProvider {
    fn from(v: CallHierarchyRegistrationOptions) -> Self {
        Self::CallHierarchyRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum ChangeNotifications {
    String(String),
    Bool(bool),
}
impl From<String> for ChangeNotifications {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for ChangeNotifications {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for ChangeNotifications {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for ChangeNotifications {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for ChangeNotifications {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<bool> for ChangeNotifications {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum ClientSemanticTokensRequestOptionsFull {
    Bool(bool),
    ClientSemanticTokensRequestFullDelta(ClientSemanticTokensRequestFullDelta),
}
impl From<bool> for ClientSemanticTokensRequestOptionsFull {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<ClientSemanticTokensRequestFullDelta>
for ClientSemanticTokensRequestOptionsFull {
    fn from(v: ClientSemanticTokensRequestFullDelta) -> Self {
        Self::ClientSemanticTokensRequestFullDelta(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum ClientSemanticTokensRequestOptionsRange {
    Bool(bool),
    Object(LspObject),
}
impl From<bool> for ClientSemanticTokensRequestOptionsRange {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<LspObject> for ClientSemanticTokensRequestOptionsRange {
    fn from(v: LspObject) -> Self {
        Self::Object(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Code {
    Int(i32),
    String(String),
}
impl From<i32> for Code {
    fn from(v: i32) -> Self {
        Self::Int(v)
    }
}
impl From<String> for Code {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Code {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for Code {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for Code {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for Code {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(untagged)]
pub enum CodeActionPartialResponse {
    Command(Command),
    CodeAction(CodeAction),
}
impl From<Command> for CodeActionPartialResponse {
    fn from(v: Command) -> Self {
        Self::Command(v)
    }
}
impl From<CodeAction> for CodeActionPartialResponse {
    fn from(v: CodeAction) -> Self {
        Self::CodeAction(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum CodeActionProvider {
    Bool(bool),
    CodeActionOptions(CodeActionOptions),
}
impl From<bool> for CodeActionProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<CodeActionOptions> for CodeActionProvider {
    fn from(v: CodeActionOptions) -> Self {
        Self::CodeActionOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(untagged)]
pub enum CodeActionResponse {
    Command(Command),
    CodeAction(CodeAction),
}
impl From<Command> for CodeActionResponse {
    fn from(v: Command) -> Self {
        Self::Command(v)
    }
}
impl From<CodeAction> for CodeActionResponse {
    fn from(v: CodeAction) -> Self {
        Self::CodeAction(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum ColorProvider {
    Bool(bool),
    DocumentColorOptions(DocumentColorOptions),
    DocumentColorRegistrationOptions(DocumentColorRegistrationOptions),
}
impl From<bool> for ColorProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<DocumentColorOptions> for ColorProvider {
    fn from(v: DocumentColorOptions) -> Self {
        Self::DocumentColorOptions(v)
    }
}
impl From<DocumentColorRegistrationOptions> for ColorProvider {
    fn from(v: DocumentColorRegistrationOptions) -> Self {
        Self::DocumentColorRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum CompletionItemTextEdit {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit),
}
impl From<TextEdit> for CompletionItemTextEdit {
    fn from(v: TextEdit) -> Self {
        Self::TextEdit(v)
    }
}
impl From<InsertReplaceEdit> for CompletionItemTextEdit {
    fn from(v: InsertReplaceEdit) -> Self {
        Self::InsertReplaceEdit(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum CompletionResponse {
    CompletionItemList(Vec<CompletionItem>),
    CompletionList(CompletionList),
}
impl From<Vec<CompletionItem>> for CompletionResponse {
    fn from(v: Vec<CompletionItem>) -> Self {
        Self::CompletionItemList(v)
    }
}
impl From<CompletionList> for CompletionResponse {
    fn from(v: CompletionList) -> Self {
        Self::CompletionList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Contents {
    MarkupContent(MarkupContent),
    MarkedString(MarkedString),
    MarkedStringList(Vec<MarkedString>),
}
impl From<MarkupContent> for Contents {
    fn from(v: MarkupContent) -> Self {
        Self::MarkupContent(v)
    }
}
impl From<MarkedString> for Contents {
    fn from(v: MarkedString) -> Self {
        Self::MarkedString(v)
    }
}
impl From<Vec<MarkedString>> for Contents {
    fn from(v: Vec<MarkedString>) -> Self {
        Self::MarkedStringList(v)
    }
}

/// The declaration of a symbol representation as one or many [locations][Location].
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Declaration {
    Location(Location),
    LocationList(Vec<Location>),
}
impl From<Location> for Declaration {
    fn from(v: Location) -> Self {
        Self::Location(v)
    }
}
impl From<Vec<Location>> for Declaration {
    fn from(v: Vec<Location>) -> Self {
        Self::LocationList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DeclarationPartialResponse {
    LocationList(Vec<Location>),
    DeclarationLinkList(Vec<DeclarationLink>),
}
impl From<Vec<Location>> for DeclarationPartialResponse {
    fn from(v: Vec<Location>) -> Self {
        Self::LocationList(v)
    }
}
impl From<Vec<DeclarationLink>> for DeclarationPartialResponse {
    fn from(v: Vec<DeclarationLink>) -> Self {
        Self::DeclarationLinkList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DeclarationProvider {
    Bool(bool),
    DeclarationOptions(DeclarationOptions),
    DeclarationRegistrationOptions(DeclarationRegistrationOptions),
}
impl From<bool> for DeclarationProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<DeclarationOptions> for DeclarationProvider {
    fn from(v: DeclarationOptions) -> Self {
        Self::DeclarationOptions(v)
    }
}
impl From<DeclarationRegistrationOptions> for DeclarationProvider {
    fn from(v: DeclarationRegistrationOptions) -> Self {
        Self::DeclarationRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DeclarationResponse {
    Declaration(Declaration),
    DeclarationLinkList(Vec<DeclarationLink>),
}
impl From<Declaration> for DeclarationResponse {
    fn from(v: Declaration) -> Self {
        Self::Declaration(v)
    }
}
impl From<Vec<DeclarationLink>> for DeclarationResponse {
    fn from(v: Vec<DeclarationLink>) -> Self {
        Self::DeclarationLinkList(v)
    }
}

/// The definition of a symbol represented as one or many [locations][Location].
/// For most programming languages there is only one location at which a symbol is
/// defined.
///
/// Servers should prefer returning `DefinitionLink` over `Definition` if supported
/// by the client.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Definition {
    Location(Location),
    LocationList(Vec<Location>),
}
impl From<Location> for Definition {
    fn from(v: Location) -> Self {
        Self::Location(v)
    }
}
impl From<Vec<Location>> for Definition {
    fn from(v: Vec<Location>) -> Self {
        Self::LocationList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DefinitionPartialResponse {
    LocationList(Vec<Location>),
    DefinitionLinkList(Vec<DefinitionLink>),
}
impl From<Vec<Location>> for DefinitionPartialResponse {
    fn from(v: Vec<Location>) -> Self {
        Self::LocationList(v)
    }
}
impl From<Vec<DefinitionLink>> for DefinitionPartialResponse {
    fn from(v: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum DefinitionProvider {
    Bool(bool),
    DefinitionOptions(DefinitionOptions),
}
impl From<bool> for DefinitionProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<DefinitionOptions> for DefinitionProvider {
    fn from(v: DefinitionOptions) -> Self {
        Self::DefinitionOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DefinitionResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
}
impl From<Definition> for DefinitionResponse {
    fn from(v: Definition) -> Self {
        Self::Definition(v)
    }
}
impl From<Vec<DefinitionLink>> for DefinitionResponse {
    fn from(v: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DiagnosticProvider {
    DiagnosticOptions(DiagnosticOptions),
    DiagnosticRegistrationOptions(DiagnosticRegistrationOptions),
}
impl From<DiagnosticOptions> for DiagnosticProvider {
    fn from(v: DiagnosticOptions) -> Self {
        Self::DiagnosticOptions(v)
    }
}
impl From<DiagnosticRegistrationOptions> for DiagnosticProvider {
    fn from(v: DiagnosticRegistrationOptions) -> Self {
        Self::DiagnosticRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DocumentChange {
    TextDocumentEdit(TextDocumentEdit),
    CreateFile(CreateFile),
    RenameFile(RenameFile),
    DeleteFile(DeleteFile),
}
impl From<TextDocumentEdit> for DocumentChange {
    fn from(v: TextDocumentEdit) -> Self {
        Self::TextDocumentEdit(v)
    }
}
impl From<CreateFile> for DocumentChange {
    fn from(v: CreateFile) -> Self {
        Self::CreateFile(v)
    }
}
impl From<RenameFile> for DocumentChange {
    fn from(v: RenameFile) -> Self {
        Self::RenameFile(v)
    }
}
impl From<DeleteFile> for DocumentChange {
    fn from(v: DeleteFile) -> Self {
        Self::DeleteFile(v)
    }
}

/// The result of a document diagnostic pull request. A report can
/// either be a full report containing all diagnostics for the
/// requested document or an unchanged report indicating that nothing
/// has changed in terms of diagnostics in comparison to the last
/// pull request.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(untagged)]
pub enum DocumentDiagnosticReport {
    RelatedFullDocumentDiagnosticReport(RelatedFullDocumentDiagnosticReport),
    RelatedUnchangedDocumentDiagnosticReport(RelatedUnchangedDocumentDiagnosticReport),
}
impl From<RelatedFullDocumentDiagnosticReport> for DocumentDiagnosticReport {
    fn from(v: RelatedFullDocumentDiagnosticReport) -> Self {
        Self::RelatedFullDocumentDiagnosticReport(v)
    }
}
impl From<RelatedUnchangedDocumentDiagnosticReport> for DocumentDiagnosticReport {
    fn from(v: RelatedUnchangedDocumentDiagnosticReport) -> Self {
        Self::RelatedUnchangedDocumentDiagnosticReport(v)
    }
}

/// A document filter describes a top level text document or
/// a notebook cell document.
///
/// @since 3.17.0 - support for NotebookCellTextDocumentFilter.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DocumentFilter {
    TextDocumentFilter(TextDocumentFilter),
    NotebookCellTextDocumentFilter(NotebookCellTextDocumentFilter),
}
impl From<TextDocumentFilter> for DocumentFilter {
    fn from(v: TextDocumentFilter) -> Self {
        Self::TextDocumentFilter(v)
    }
}
impl From<NotebookCellTextDocumentFilter> for DocumentFilter {
    fn from(v: NotebookCellTextDocumentFilter) -> Self {
        Self::NotebookCellTextDocumentFilter(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum DocumentFormattingProvider {
    Bool(bool),
    DocumentFormattingOptions(DocumentFormattingOptions),
}
impl From<bool> for DocumentFormattingProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<DocumentFormattingOptions> for DocumentFormattingProvider {
    fn from(v: DocumentFormattingOptions) -> Self {
        Self::DocumentFormattingOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum DocumentHighlightProvider {
    Bool(bool),
    DocumentHighlightOptions(DocumentHighlightOptions),
}
impl From<bool> for DocumentHighlightProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<DocumentHighlightOptions> for DocumentHighlightProvider {
    fn from(v: DocumentHighlightOptions) -> Self {
        Self::DocumentHighlightOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum DocumentRangeFormattingProvider {
    Bool(bool),
    DocumentRangeFormattingOptions(DocumentRangeFormattingOptions),
}
impl From<bool> for DocumentRangeFormattingProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<DocumentRangeFormattingOptions> for DocumentRangeFormattingProvider {
    fn from(v: DocumentRangeFormattingOptions) -> Self {
        Self::DocumentRangeFormattingOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DocumentSymbolPartialResponse {
    SymbolInformationList(Vec<SymbolInformation>),
    DocumentSymbolList(Vec<DocumentSymbol>),
}
impl From<Vec<SymbolInformation>> for DocumentSymbolPartialResponse {
    fn from(v: Vec<SymbolInformation>) -> Self {
        Self::SymbolInformationList(v)
    }
}
impl From<Vec<DocumentSymbol>> for DocumentSymbolPartialResponse {
    fn from(v: Vec<DocumentSymbol>) -> Self {
        Self::DocumentSymbolList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DocumentSymbolProvider {
    Bool(bool),
    DocumentSymbolOptions(DocumentSymbolOptions),
}
impl From<bool> for DocumentSymbolProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<DocumentSymbolOptions> for DocumentSymbolProvider {
    fn from(v: DocumentSymbolOptions) -> Self {
        Self::DocumentSymbolOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum DocumentSymbolResponse {
    SymbolInformationList(Vec<SymbolInformation>),
    DocumentSymbolList(Vec<DocumentSymbol>),
}
impl From<Vec<SymbolInformation>> for DocumentSymbolResponse {
    fn from(v: Vec<SymbolInformation>) -> Self {
        Self::SymbolInformationList(v)
    }
}
impl From<Vec<DocumentSymbol>> for DocumentSymbolResponse {
    fn from(v: Vec<DocumentSymbol>) -> Self {
        Self::DocumentSymbolList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Documentation {
    String(String),
    MarkupContent(MarkupContent),
}
impl From<String> for Documentation {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Documentation {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for Documentation {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for Documentation {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for Documentation {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<MarkupContent> for Documentation {
    fn from(v: MarkupContent) -> Self {
        Self::MarkupContent(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Edit {
    TextEdit(TextEdit),
    AnnotatedTextEdit(AnnotatedTextEdit),
    SnippetTextEdit(SnippetTextEdit),
}
impl From<TextEdit> for Edit {
    fn from(v: TextEdit) -> Self {
        Self::TextEdit(v)
    }
}
impl From<AnnotatedTextEdit> for Edit {
    fn from(v: AnnotatedTextEdit) -> Self {
        Self::AnnotatedTextEdit(v)
    }
}
impl From<SnippetTextEdit> for Edit {
    fn from(v: SnippetTextEdit) -> Self {
        Self::SnippetTextEdit(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum EditRange {
    Range(Range),
    EditRangeWithInsertReplace(EditRangeWithInsertReplace),
}
impl From<Range> for EditRange {
    fn from(v: Range) -> Self {
        Self::Range(v)
    }
}
impl From<EditRangeWithInsertReplace> for EditRange {
    fn from(v: EditRangeWithInsertReplace) -> Self {
        Self::EditRangeWithInsertReplace(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum FoldingRangeProvider {
    Bool(bool),
    FoldingRangeOptions(FoldingRangeOptions),
    FoldingRangeRegistrationOptions(FoldingRangeRegistrationOptions),
}
impl From<bool> for FoldingRangeProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<FoldingRangeOptions> for FoldingRangeProvider {
    fn from(v: FoldingRangeOptions) -> Self {
        Self::FoldingRangeOptions(v)
    }
}
impl From<FoldingRangeRegistrationOptions> for FoldingRangeProvider {
    fn from(v: FoldingRangeRegistrationOptions) -> Self {
        Self::FoldingRangeRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum Full {
    Bool(bool),
    SemanticTokensFullDelta(SemanticTokensFullDelta),
}
impl From<bool> for Full {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<SemanticTokensFullDelta> for Full {
    fn from(v: SemanticTokensFullDelta) -> Self {
        Self::SemanticTokensFullDelta(v)
    }
}

/// The glob pattern. Either a string pattern or a relative pattern.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum GlobPattern {
    Pattern(Pattern),
    RelativePattern(RelativePattern),
}
impl From<Pattern> for GlobPattern {
    fn from(v: Pattern) -> Self {
        Self::Pattern(v)
    }
}
impl From<RelativePattern> for GlobPattern {
    fn from(v: RelativePattern) -> Self {
        Self::RelativePattern(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum HoverProvider {
    Bool(bool),
    HoverOptions(HoverOptions),
}
impl From<bool> for HoverProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<HoverOptions> for HoverProvider {
    fn from(v: HoverOptions) -> Self {
        Self::HoverOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Id {
    Int(i32),
    String(String),
}
impl From<i32> for Id {
    fn from(v: i32) -> Self {
        Self::Int(v)
    }
}
impl From<String> for Id {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Id {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for Id {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for Id {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for Id {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum ImplementationPartialResponse {
    LocationList(Vec<Location>),
    DefinitionLinkList(Vec<DefinitionLink>),
}
impl From<Vec<Location>> for ImplementationPartialResponse {
    fn from(v: Vec<Location>) -> Self {
        Self::LocationList(v)
    }
}
impl From<Vec<DefinitionLink>> for ImplementationPartialResponse {
    fn from(v: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum ImplementationProvider {
    Bool(bool),
    ImplementationOptions(ImplementationOptions),
    ImplementationRegistrationOptions(ImplementationRegistrationOptions),
}
impl From<bool> for ImplementationProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<ImplementationOptions> for ImplementationProvider {
    fn from(v: ImplementationOptions) -> Self {
        Self::ImplementationOptions(v)
    }
}
impl From<ImplementationRegistrationOptions> for ImplementationProvider {
    fn from(v: ImplementationRegistrationOptions) -> Self {
        Self::ImplementationRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum ImplementationResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
}
impl From<Definition> for ImplementationResponse {
    fn from(v: Definition) -> Self {
        Self::Definition(v)
    }
}
impl From<Vec<DefinitionLink>> for ImplementationResponse {
    fn from(v: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum InlayHintProvider {
    Bool(bool),
    InlayHintOptions(InlayHintOptions),
    InlayHintRegistrationOptions(InlayHintRegistrationOptions),
}
impl From<bool> for InlayHintProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<InlayHintOptions> for InlayHintProvider {
    fn from(v: InlayHintOptions) -> Self {
        Self::InlayHintOptions(v)
    }
}
impl From<InlayHintRegistrationOptions> for InlayHintProvider {
    fn from(v: InlayHintRegistrationOptions) -> Self {
        Self::InlayHintRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum InlineCompletionProvider {
    Bool(bool),
    InlineCompletionOptions(InlineCompletionOptions),
}
impl From<bool> for InlineCompletionProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<InlineCompletionOptions> for InlineCompletionProvider {
    fn from(v: InlineCompletionOptions) -> Self {
        Self::InlineCompletionOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum InlineCompletionResponse {
    InlineCompletionList(InlineCompletionList),
    InlineCompletionItemList(Vec<InlineCompletionItem>),
}
impl From<InlineCompletionList> for InlineCompletionResponse {
    fn from(v: InlineCompletionList) -> Self {
        Self::InlineCompletionList(v)
    }
}
impl From<Vec<InlineCompletionItem>> for InlineCompletionResponse {
    fn from(v: Vec<InlineCompletionItem>) -> Self {
        Self::InlineCompletionItemList(v)
    }
}

/// Inline value information can be provided by different means:
/// - directly as a text value (class InlineValueText).
/// - as a name to use for a variable lookup (class InlineValueVariableLookup)
/// - as an evaluatable expression (class InlineValueEvaluatableExpression)
/// The InlineValue types combines all inline value types into one type.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum InlineValue {
    Text(InlineValueText),
    VariableLookup(InlineValueVariableLookup),
    EvaluatableExpression(InlineValueEvaluatableExpression),
}
impl From<InlineValueText> for InlineValue {
    fn from(v: InlineValueText) -> Self {
        Self::Text(v)
    }
}
impl From<InlineValueVariableLookup> for InlineValue {
    fn from(v: InlineValueVariableLookup) -> Self {
        Self::VariableLookup(v)
    }
}
impl From<InlineValueEvaluatableExpression> for InlineValue {
    fn from(v: InlineValueEvaluatableExpression) -> Self {
        Self::EvaluatableExpression(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum InlineValueProvider {
    Bool(bool),
    InlineValueOptions(InlineValueOptions),
    InlineValueRegistrationOptions(InlineValueRegistrationOptions),
}
impl From<bool> for InlineValueProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<InlineValueOptions> for InlineValueProvider {
    fn from(v: InlineValueOptions) -> Self {
        Self::InlineValueOptions(v)
    }
}
impl From<InlineValueRegistrationOptions> for InlineValueProvider {
    fn from(v: InlineValueRegistrationOptions) -> Self {
        Self::InlineValueRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum InsertText {
    String(String),
    StringValue(StringValue),
}
impl From<String> for InsertText {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for InsertText {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for InsertText {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for InsertText {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for InsertText {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<StringValue> for InsertText {
    fn from(v: StringValue) -> Self {
        Self::StringValue(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Label {
    String(String),
    InlayHintLabelPartList(Vec<InlayHintLabelPart>),
}
impl From<String> for Label {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Label {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for Label {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for Label {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for Label {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Vec<InlayHintLabelPart>> for Label {
    fn from(v: Vec<InlayHintLabelPart>) -> Self {
        Self::InlayHintLabelPartList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum LinkedEditingRangeProvider {
    Bool(bool),
    LinkedEditingRangeOptions(LinkedEditingRangeOptions),
    LinkedEditingRangeRegistrationOptions(LinkedEditingRangeRegistrationOptions),
}
impl From<bool> for LinkedEditingRangeProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<LinkedEditingRangeOptions> for LinkedEditingRangeProvider {
    fn from(v: LinkedEditingRangeOptions) -> Self {
        Self::LinkedEditingRangeOptions(v)
    }
}
impl From<LinkedEditingRangeRegistrationOptions> for LinkedEditingRangeProvider {
    fn from(v: LinkedEditingRangeRegistrationOptions) -> Self {
        Self::LinkedEditingRangeRegistrationOptions(v)
    }
}

/// MarkedString can be used to render human readable text. It is either a markdown string
/// or a code-block that provides a language and a code snippet. The language identifier
/// is semantically equal to the optional language identifier in fenced code blocks in GitHub
/// issues. See https://help.github.com/articles/creating-and-highlighting-code-blocks/#syntax-highlighting
///
/// The pair of a language and a value is an equivalent to markdown:
/// ```${language}
/// ${value}
/// ```
///
/// Note that markdown strings will be sanitized - that means html will be escaped.
/// @deprecated use MarkupContent instead.
#[deprecated(note = "use MarkupContent instead.")]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum MarkedString {
    String(String),
    MarkedStringWithLanguage(MarkedStringWithLanguage),
}
impl From<String> for MarkedString {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for MarkedString {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for MarkedString {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for MarkedString {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for MarkedString {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<MarkedStringWithLanguage> for MarkedString {
    fn from(v: MarkedStringWithLanguage) -> Self {
        Self::MarkedStringWithLanguage(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Message {
    String(String),
    MarkupContent(MarkupContent),
}
impl From<String> for Message {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Message {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for Message {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for Message {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for Message {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<MarkupContent> for Message {
    fn from(v: MarkupContent) -> Self {
        Self::MarkupContent(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum MonikerProvider {
    Bool(bool),
    MonikerOptions(MonikerOptions),
    MonikerRegistrationOptions(MonikerRegistrationOptions),
}
impl From<bool> for MonikerProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<MonikerOptions> for MonikerProvider {
    fn from(v: MonikerOptions) -> Self {
        Self::MonikerOptions(v)
    }
}
impl From<MonikerRegistrationOptions> for MonikerProvider {
    fn from(v: MonikerRegistrationOptions) -> Self {
        Self::MonikerRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Notebook {
    String(String),
    NotebookDocumentFilter(NotebookDocumentFilter),
}
impl From<String> for Notebook {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Notebook {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for Notebook {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for Notebook {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for Notebook {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<NotebookDocumentFilter> for Notebook {
    fn from(v: NotebookDocumentFilter) -> Self {
        Self::NotebookDocumentFilter(v)
    }
}

/// A notebook document filter denotes a notebook document by
/// different properties. The properties will be match
/// against the notebook's URI (same as with documents)
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum NotebookDocumentFilter {
    NotebookType(NotebookDocumentFilterNotebookType),
    Scheme(NotebookDocumentFilterScheme),
    Pattern(NotebookDocumentFilterPattern),
}
impl From<NotebookDocumentFilterNotebookType> for NotebookDocumentFilter {
    fn from(v: NotebookDocumentFilterNotebookType) -> Self {
        Self::NotebookType(v)
    }
}
impl From<NotebookDocumentFilterScheme> for NotebookDocumentFilter {
    fn from(v: NotebookDocumentFilterScheme) -> Self {
        Self::Scheme(v)
    }
}
impl From<NotebookDocumentFilterPattern> for NotebookDocumentFilter {
    fn from(v: NotebookDocumentFilterPattern) -> Self {
        Self::Pattern(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum NotebookDocumentSync {
    Options(NotebookDocumentSyncOptions),
    RegistrationOptions(NotebookDocumentSyncRegistrationOptions),
}
impl From<NotebookDocumentSyncOptions> for NotebookDocumentSync {
    fn from(v: NotebookDocumentSyncOptions) -> Self {
        Self::Options(v)
    }
}
impl From<NotebookDocumentSyncRegistrationOptions> for NotebookDocumentSync {
    fn from(v: NotebookDocumentSyncRegistrationOptions) -> Self {
        Self::RegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum NotebookSelector {
    NotebookDocumentFilterWithNotebook(NotebookDocumentFilterWithNotebook),
    NotebookDocumentFilterWithCells(NotebookDocumentFilterWithCells),
}
impl From<NotebookDocumentFilterWithNotebook> for NotebookSelector {
    fn from(v: NotebookDocumentFilterWithNotebook) -> Self {
        Self::NotebookDocumentFilterWithNotebook(v)
    }
}
impl From<NotebookDocumentFilterWithCells> for NotebookSelector {
    fn from(v: NotebookDocumentFilterWithCells) -> Self {
        Self::NotebookDocumentFilterWithCells(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum ParameterInformationLabel {
    String(String),
    Tuple((u32, u32)),
}
impl From<String> for ParameterInformationLabel {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for ParameterInformationLabel {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for ParameterInformationLabel {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for ParameterInformationLabel {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for ParameterInformationLabel {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<(u32, u32)> for ParameterInformationLabel {
    fn from(v: (u32, u32)) -> Self {
        Self::Tuple(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum PrepareRenameResult {
    Range(Range),
    PrepareRenamePlaceholder(PrepareRenamePlaceholder),
    PrepareRenameDefaultBehavior(PrepareRenameDefaultBehavior),
}
impl From<Range> for PrepareRenameResult {
    fn from(v: Range) -> Self {
        Self::Range(v)
    }
}
impl From<PrepareRenamePlaceholder> for PrepareRenameResult {
    fn from(v: PrepareRenamePlaceholder) -> Self {
        Self::PrepareRenamePlaceholder(v)
    }
}
impl From<PrepareRenameDefaultBehavior> for PrepareRenameResult {
    fn from(v: PrepareRenameDefaultBehavior) -> Self {
        Self::PrepareRenameDefaultBehavior(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum ProgressToken {
    Int(i32),
    String(String),
}
impl From<i32> for ProgressToken {
    fn from(v: i32) -> Self {
        Self::Int(v)
    }
}
impl From<String> for ProgressToken {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for ProgressToken {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for ProgressToken {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for ProgressToken {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for ProgressToken {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum ReferencesProvider {
    Bool(bool),
    ReferenceOptions(ReferenceOptions),
}
impl From<bool> for ReferencesProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<ReferenceOptions> for ReferencesProvider {
    fn from(v: ReferenceOptions) -> Self {
        Self::ReferenceOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum RelatedDocument {
    FullDocumentDiagnosticReport(FullDocumentDiagnosticReport),
    UnchangedDocumentDiagnosticReport(UnchangedDocumentDiagnosticReport),
}
impl From<FullDocumentDiagnosticReport> for RelatedDocument {
    fn from(v: FullDocumentDiagnosticReport) -> Self {
        Self::FullDocumentDiagnosticReport(v)
    }
}
impl From<UnchangedDocumentDiagnosticReport> for RelatedDocument {
    fn from(v: UnchangedDocumentDiagnosticReport) -> Self {
        Self::UnchangedDocumentDiagnosticReport(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum RenameProvider {
    Bool(bool),
    RenameOptions(RenameOptions),
}
impl From<bool> for RenameProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<RenameOptions> for RenameProvider {
    fn from(v: RenameOptions) -> Self {
        Self::RenameOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum RootPath {
    String(String),
    Null,
}
impl From<String> for RootPath {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for RootPath {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for RootPath {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for RootPath {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for RootPath {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<()> for RootPath {
    fn from((): ()) -> Self {
        Self::Null
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum Save {
    Bool(bool),
    SaveOptions(SaveOptions),
}
impl From<bool> for Save {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<SaveOptions> for Save {
    fn from(v: SaveOptions) -> Self {
        Self::SaveOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Section {
    String(String),
    StringList(Vec<String>),
}
impl From<String> for Section {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Section {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for Section {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for Section {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for Section {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Vec<String>> for Section {
    fn from(v: Vec<String>) -> Self {
        Self::StringList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum SelectionRangeProvider {
    Bool(bool),
    SelectionRangeOptions(SelectionRangeOptions),
    SelectionRangeRegistrationOptions(SelectionRangeRegistrationOptions),
}
impl From<bool> for SelectionRangeProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<SelectionRangeOptions> for SelectionRangeProvider {
    fn from(v: SelectionRangeOptions) -> Self {
        Self::SelectionRangeOptions(v)
    }
}
impl From<SelectionRangeRegistrationOptions> for SelectionRangeProvider {
    fn from(v: SelectionRangeRegistrationOptions) -> Self {
        Self::SelectionRangeRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum SemanticTokensDeltaPartialResponse {
    SemanticTokensPartialResult(SemanticTokensPartialResult),
    SemanticTokensDeltaPartialResult(SemanticTokensDeltaPartialResult),
}
impl From<SemanticTokensPartialResult> for SemanticTokensDeltaPartialResponse {
    fn from(v: SemanticTokensPartialResult) -> Self {
        Self::SemanticTokensPartialResult(v)
    }
}
impl From<SemanticTokensDeltaPartialResult> for SemanticTokensDeltaPartialResponse {
    fn from(v: SemanticTokensDeltaPartialResult) -> Self {
        Self::SemanticTokensDeltaPartialResult(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum SemanticTokensDeltaResponse {
    SemanticTokens(SemanticTokens),
    SemanticTokensDelta(SemanticTokensDelta),
}
impl From<SemanticTokens> for SemanticTokensDeltaResponse {
    fn from(v: SemanticTokens) -> Self {
        Self::SemanticTokens(v)
    }
}
impl From<SemanticTokensDelta> for SemanticTokensDeltaResponse {
    fn from(v: SemanticTokensDelta) -> Self {
        Self::SemanticTokensDelta(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum SemanticTokensOptionsRange {
    Bool(bool),
    Object(LspObject),
}
impl From<bool> for SemanticTokensOptionsRange {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<LspObject> for SemanticTokensOptionsRange {
    fn from(v: LspObject) -> Self {
        Self::Object(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum SemanticTokensProvider {
    SemanticTokensOptions(SemanticTokensOptions),
    SemanticTokensRegistrationOptions(SemanticTokensRegistrationOptions),
}
impl From<SemanticTokensOptions> for SemanticTokensProvider {
    fn from(v: SemanticTokensOptions) -> Self {
        Self::SemanticTokensOptions(v)
    }
}
impl From<SemanticTokensRegistrationOptions> for SemanticTokensProvider {
    fn from(v: SemanticTokensRegistrationOptions) -> Self {
        Self::SemanticTokensRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum TextDocumentContent {
    Options(TextDocumentContentOptions),
    RegistrationOptions(TextDocumentContentRegistrationOptions),
}
impl From<TextDocumentContentOptions> for TextDocumentContent {
    fn from(v: TextDocumentContentOptions) -> Self {
        Self::Options(v)
    }
}
impl From<TextDocumentContentRegistrationOptions> for TextDocumentContent {
    fn from(v: TextDocumentContentRegistrationOptions) -> Self {
        Self::RegistrationOptions(v)
    }
}

/// An event describing a change to a text document. If only a text is provided
/// it is considered to be the full content of the document.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum TextDocumentContentChangeEvent {
    TextDocumentContentChangePartial(TextDocumentContentChangePartial),
    TextDocumentContentChangeWholeDocument(TextDocumentContentChangeWholeDocument),
}
impl From<TextDocumentContentChangePartial> for TextDocumentContentChangeEvent {
    fn from(v: TextDocumentContentChangePartial) -> Self {
        Self::TextDocumentContentChangePartial(v)
    }
}
impl From<TextDocumentContentChangeWholeDocument> for TextDocumentContentChangeEvent {
    fn from(v: TextDocumentContentChangeWholeDocument) -> Self {
        Self::TextDocumentContentChangeWholeDocument(v)
    }
}

/// A document filter denotes a document by different properties like
/// the [language][`TextDocument::languageId`], the [scheme][`Uri::scheme`] of
/// its resource, or a glob-pattern that is applied to the [path][`TextDocument::fileName`].
///
/// Glob patterns can have the following syntax:
/// - `*` to match zero or more characters in a path segment
/// - `?` to match on one character in a path segment
/// - `**` to match any number of path segments, including none
/// - `{}` to group sub patterns into an OR expression. (e.g. `**/*.{ts,js}` matches all TypeScript and JavaScript files)
/// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
/// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
///
/// @sample A language filter that applies to typescript files on disk: `{ language: 'typescript', scheme: 'file' }`
/// @sample A language filter that applies to all package.json paths: `{ language: 'json', pattern: '**package.json' }`
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum TextDocumentFilter {
    Language(TextDocumentFilterLanguage),
    Scheme(TextDocumentFilterScheme),
    Pattern(TextDocumentFilterPattern),
}
impl From<TextDocumentFilterLanguage> for TextDocumentFilter {
    fn from(v: TextDocumentFilterLanguage) -> Self {
        Self::Language(v)
    }
}
impl From<TextDocumentFilterScheme> for TextDocumentFilter {
    fn from(v: TextDocumentFilterScheme) -> Self {
        Self::Scheme(v)
    }
}
impl From<TextDocumentFilterPattern> for TextDocumentFilter {
    fn from(v: TextDocumentFilterPattern) -> Self {
        Self::Pattern(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum TextDocumentSync {
    Options(TextDocumentSyncOptions),
    Kind(TextDocumentSyncKind),
}
impl From<TextDocumentSyncOptions> for TextDocumentSync {
    fn from(v: TextDocumentSyncOptions) -> Self {
        Self::Options(v)
    }
}
impl From<TextDocumentSyncKind> for TextDocumentSync {
    fn from(v: TextDocumentSyncKind) -> Self {
        Self::Kind(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum Tooltip {
    String(String),
    MarkupContent(MarkupContent),
}
impl From<String> for Tooltip {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for Tooltip {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}
impl From<char> for Tooltip {
    fn from(v: char) -> Self {
        Self::String(v.into())
    }
}
impl From<Box<str>> for Tooltip {
    fn from(v: Box<str>) -> Self {
        Self::String(v.into())
    }
}
impl From<Cow<'_, str>> for Tooltip {
    fn from(v: Cow<'_, str>) -> Self {
        Self::String(v.into())
    }
}
impl From<MarkupContent> for Tooltip {
    fn from(v: MarkupContent) -> Self {
        Self::MarkupContent(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum TypeDefinitionPartialResponse {
    LocationList(Vec<Location>),
    DefinitionLinkList(Vec<DefinitionLink>),
}
impl From<Vec<Location>> for TypeDefinitionPartialResponse {
    fn from(v: Vec<Location>) -> Self {
        Self::LocationList(v)
    }
}
impl From<Vec<DefinitionLink>> for TypeDefinitionPartialResponse {
    fn from(v: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum TypeDefinitionProvider {
    Bool(bool),
    TypeDefinitionOptions(TypeDefinitionOptions),
    TypeDefinitionRegistrationOptions(TypeDefinitionRegistrationOptions),
}
impl From<bool> for TypeDefinitionProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<TypeDefinitionOptions> for TypeDefinitionProvider {
    fn from(v: TypeDefinitionOptions) -> Self {
        Self::TypeDefinitionOptions(v)
    }
}
impl From<TypeDefinitionRegistrationOptions> for TypeDefinitionProvider {
    fn from(v: TypeDefinitionRegistrationOptions) -> Self {
        Self::TypeDefinitionRegistrationOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum TypeDefinitionResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
}
impl From<Definition> for TypeDefinitionResponse {
    fn from(v: Definition) -> Self {
        Self::Definition(v)
    }
}
impl From<Vec<DefinitionLink>> for TypeDefinitionResponse {
    fn from(v: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum TypeHierarchyProvider {
    Bool(bool),
    TypeHierarchyOptions(TypeHierarchyOptions),
    TypeHierarchyRegistrationOptions(TypeHierarchyRegistrationOptions),
}
impl From<bool> for TypeHierarchyProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<TypeHierarchyOptions> for TypeHierarchyProvider {
    fn from(v: TypeHierarchyOptions) -> Self {
        Self::TypeHierarchyOptions(v)
    }
}
impl From<TypeHierarchyRegistrationOptions> for TypeHierarchyProvider {
    fn from(v: TypeHierarchyRegistrationOptions) -> Self {
        Self::TypeHierarchyRegistrationOptions(v)
    }
}

/// A workspace diagnostic document report.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum WorkspaceDocumentDiagnosticReport {
    WorkspaceFullDocumentDiagnosticReport(WorkspaceFullDocumentDiagnosticReport),
    WorkspaceUnchangedDocumentDiagnosticReport(
        WorkspaceUnchangedDocumentDiagnosticReport,
    ),
}
impl From<WorkspaceFullDocumentDiagnosticReport> for WorkspaceDocumentDiagnosticReport {
    fn from(v: WorkspaceFullDocumentDiagnosticReport) -> Self {
        Self::WorkspaceFullDocumentDiagnosticReport(v)
    }
}
impl From<WorkspaceUnchangedDocumentDiagnosticReport>
for WorkspaceDocumentDiagnosticReport {
    fn from(v: WorkspaceUnchangedDocumentDiagnosticReport) -> Self {
        Self::WorkspaceUnchangedDocumentDiagnosticReport(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum WorkspaceFolders {
    WorkspaceFolderList(Vec<WorkspaceFolder>),
    Null,
}
impl From<Vec<WorkspaceFolder>> for WorkspaceFolders {
    fn from(v: Vec<WorkspaceFolder>) -> Self {
        Self::WorkspaceFolderList(v)
    }
}
impl From<()> for WorkspaceFolders {
    fn from((): ()) -> Self {
        Self::Null
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum WorkspaceSymbolLocation {
    Location(Location),
    LocationUriOnly(LocationUriOnly),
}
impl From<Location> for WorkspaceSymbolLocation {
    fn from(v: Location) -> Self {
        Self::Location(v)
    }
}
impl From<LocationUriOnly> for WorkspaceSymbolLocation {
    fn from(v: LocationUriOnly) -> Self {
        Self::LocationUriOnly(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum WorkspaceSymbolPartialResponse {
    SymbolInformationList(Vec<SymbolInformation>),
    WorkspaceSymbolList(Vec<WorkspaceSymbol>),
}
impl From<Vec<SymbolInformation>> for WorkspaceSymbolPartialResponse {
    fn from(v: Vec<SymbolInformation>) -> Self {
        Self::SymbolInformationList(v)
    }
}
impl From<Vec<WorkspaceSymbol>> for WorkspaceSymbolPartialResponse {
    fn from(v: Vec<WorkspaceSymbol>) -> Self {
        Self::WorkspaceSymbolList(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
#[serde(untagged)]
pub enum WorkspaceSymbolProvider {
    Bool(bool),
    WorkspaceSymbolOptions(WorkspaceSymbolOptions),
}
impl From<bool> for WorkspaceSymbolProvider {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<WorkspaceSymbolOptions> for WorkspaceSymbolProvider {
    fn from(v: WorkspaceSymbolOptions) -> Self {
        Self::WorkspaceSymbolOptions(v)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
#[serde(untagged)]
pub enum WorkspaceSymbolResponse {
    SymbolInformationList(Vec<SymbolInformation>),
    WorkspaceSymbolList(Vec<WorkspaceSymbol>),
}
impl From<Vec<SymbolInformation>> for WorkspaceSymbolResponse {
    fn from(v: Vec<SymbolInformation>) -> Self {
        Self::SymbolInformationList(v)
    }
}
impl From<Vec<WorkspaceSymbol>> for WorkspaceSymbolResponse {
    fn from(v: Vec<WorkspaceSymbol>) -> Self {
        Self::WorkspaceSymbolList(v)
    }
}
