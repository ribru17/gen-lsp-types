#[allow(clippy::wildcard_imports)]
use super::*;

/// A request to resolve the implementation locations of a symbol at a given text
/// document position. The request's parameter is of type [`TextDocumentPositionParams`]
/// the response is of type [`Definition`] or a Thenable that resolves to such.
#[derive(Debug)]
pub enum ImplementationRequest {}
impl Request for ImplementationRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentImplementation;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = ImplementationParams;
    type Result = Option<ImplementationResponse>;
}
impl RequestWithPartialResults for ImplementationRequest {
    type PartialResult = ImplementationPartialResponse;
}

/// A request to resolve the type definition locations of a symbol at a given text
/// document position. The request's parameter is of type [`TextDocumentPositionParams`]
/// the response is of type [`Definition`] or a Thenable that resolves to such.
#[derive(Debug)]
pub enum TypeDefinitionRequest {}
impl Request for TypeDefinitionRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentTypeDefinition;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = TypeDefinitionParams;
    type Result = Option<TypeDefinitionResponse>;
}
impl RequestWithPartialResults for TypeDefinitionRequest {
    type PartialResult = TypeDefinitionPartialResponse;
}

/// The `workspace/workspaceFolders` is sent from the server to the client to fetch the open workspace folders.
#[derive(Debug)]
pub enum WorkspaceFoldersRequest {}
impl Request for WorkspaceFoldersRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceWorkspaceFolders;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ();
    type Result = Option<Vec<WorkspaceFolder>>;
}

/// The 'workspace/configuration' request is sent from the server to the client to fetch a certain
/// configuration setting.
///
/// This pull model replaces the old push model were the client signaled configuration change via an
/// event. If the server still needs to react to configuration changes (since the server caches the
/// result of `workspace/configuration` requests) the server should register for an empty configuration
/// change event and empty the cache if such an event is received.
#[derive(Debug)]
pub enum ConfigurationRequest {}
impl Request for ConfigurationRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceConfiguration;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ConfigurationParams;
    type Result = Vec<LspAny>;
}

/// A request to list all color symbols found in a given text document. The request's
/// parameter is of type [`DocumentColorParams`] the
/// response is of type [ColorInformation[]][ColorInformation] or a Thenable
/// that resolves to such.
#[derive(Debug)]
pub enum DocumentColorRequest {}
impl Request for DocumentColorRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentDocumentColor;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentColorParams;
    type Result = Vec<ColorInformation>;
}
impl RequestWithPartialResults for DocumentColorRequest {
    type PartialResult = Vec<ColorInformation>;
}

/// A request to list all presentation for a color. The request's
/// parameter is of type [`ColorPresentationParams`] the
/// response is of type [ColorPresentation[]][ColorPresentation] or a Thenable
/// that resolves to such.
#[derive(Debug)]
pub enum ColorPresentationRequest {}
impl Request for ColorPresentationRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentColorPresentation;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = ColorPresentationParams;
    type Result = Vec<ColorPresentation>;
}
impl RequestWithPartialResults for ColorPresentationRequest {
    type PartialResult = Vec<ColorPresentation>;
}

/// A request to provide folding ranges in a document. The request's
/// parameter is of type [`FoldingRangeParams`], the
/// response is of type [`FoldingRangeList`] or a Thenable
/// that resolves to such.
#[derive(Debug)]
pub enum FoldingRangeRequest {}
impl Request for FoldingRangeRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentFoldingRange;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = FoldingRangeParams;
    type Result = Option<Vec<FoldingRange>>;
}
impl RequestWithPartialResults for FoldingRangeRequest {
    type PartialResult = Vec<FoldingRange>;
}

/// A request to refresh the folding ranges in a document.
///
/// @since 3.18.0
#[derive(Debug)]
pub enum FoldingRangeRefreshRequest {}
impl Request for FoldingRangeRefreshRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceFoldingRangeRefresh;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ();
    type Result = ();
}

/// A request to resolve the type definition locations of a symbol at a given text
/// document position. The request's parameter is of type [`TextDocumentPositionParams`]
/// the response is of type [`Declaration`] or a typed array of [`DeclarationLink`]
/// or a Thenable that resolves to such.
#[derive(Debug)]
pub enum DeclarationRequest {}
impl Request for DeclarationRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentDeclaration;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DeclarationParams;
    type Result = Option<DeclarationResponse>;
}
impl RequestWithPartialResults for DeclarationRequest {
    type PartialResult = DeclarationPartialResponse;
}

/// A request to provide selection ranges in a document. The request's
/// parameter is of type [`SelectionRangeParams`], the
/// response is of type [SelectionRange[]][SelectionRange] or a Thenable
/// that resolves to such.
#[derive(Debug)]
pub enum SelectionRangeRequest {}
impl Request for SelectionRangeRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentSelectionRange;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = SelectionRangeParams;
    type Result = Option<Vec<SelectionRange>>;
}
impl RequestWithPartialResults for SelectionRangeRequest {
    type PartialResult = Vec<SelectionRange>;
}

/// The `window/workDoneProgress/create` request is sent from the server to the client to initiate progress
/// reporting from the server.
#[derive(Debug)]
pub enum WorkDoneProgressCreateRequest {}
impl Request for WorkDoneProgressCreateRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WindowWorkDoneProgressCreate;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = WorkDoneProgressCreateParams;
    type Result = ();
}

/// A request to result a `CallHierarchyItem` in a document at a given position.
/// Can be used as an input to an incoming or outgoing call hierarchy.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum CallHierarchyPrepareRequest {}
impl Request for CallHierarchyPrepareRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentPrepareCallHierarchy;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CallHierarchyPrepareParams;
    type Result = Option<Vec<CallHierarchyItem>>;
}

/// A request to resolve the incoming calls for a given `CallHierarchyItem`.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum CallHierarchyIncomingCallsRequest {}
impl Request for CallHierarchyIncomingCallsRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::CallHierarchyIncomingCalls;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CallHierarchyIncomingCallsParams;
    type Result = Option<Vec<CallHierarchyIncomingCall>>;
}
impl RequestWithPartialResults for CallHierarchyIncomingCallsRequest {
    type PartialResult = Vec<CallHierarchyIncomingCall>;
}

/// A request to resolve the outgoing calls for a given `CallHierarchyItem`.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum CallHierarchyOutgoingCallsRequest {}
impl Request for CallHierarchyOutgoingCallsRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::CallHierarchyOutgoingCalls;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CallHierarchyOutgoingCallsParams;
    type Result = Option<Vec<CallHierarchyOutgoingCall>>;
}
impl RequestWithPartialResults for CallHierarchyOutgoingCallsRequest {
    type PartialResult = Vec<CallHierarchyOutgoingCall>;
}

/// @since 3.16.0
#[derive(Debug)]
pub enum SemanticTokensRequest {}
impl Request for SemanticTokensRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentSemanticTokensFull;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = SemanticTokensParams;
    type Result = Option<SemanticTokens>;
}
impl RequestWithPartialResults for SemanticTokensRequest {
    type PartialResult = SemanticTokensPartialResult;
}

/// @since 3.16.0
#[derive(Debug)]
pub enum SemanticTokensDeltaRequest {}
impl Request for SemanticTokensDeltaRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentSemanticTokensFullDelta;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = SemanticTokensDeltaParams;
    type Result = Option<SemanticTokensDeltaResponse>;
}
impl RequestWithPartialResults for SemanticTokensDeltaRequest {
    type PartialResult = SemanticTokensDeltaPartialResponse;
}

/// @since 3.16.0
#[derive(Debug)]
pub enum SemanticTokensRangeRequest {}
impl Request for SemanticTokensRangeRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentSemanticTokensRange;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = SemanticTokensRangeParams;
    type Result = Option<SemanticTokens>;
}
impl RequestWithPartialResults for SemanticTokensRangeRequest {
    type PartialResult = SemanticTokensPartialResult;
}

/// @since 3.16.0
#[derive(Debug)]
pub enum SemanticTokensRefreshRequest {}
impl Request for SemanticTokensRefreshRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceSemanticTokensRefresh;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ();
    type Result = ();
}

/// A request to show a document. This request might open an
/// external program depending on the value of the URI to open.
/// For example a request to open `https://code.visualstudio.com/`
/// will very likely open the URI in a WEB browser.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum ShowDocumentRequest {}
impl Request for ShowDocumentRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WindowShowDocument;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ShowDocumentParams;
    type Result = ShowDocumentResult;
}

/// A request to provide ranges that can be edited together.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum LinkedEditingRangeRequest {}
impl Request for LinkedEditingRangeRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentLinkedEditingRange;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = LinkedEditingRangeParams;
    type Result = Option<LinkedEditingRanges>;
}

/// The will create files request is sent from the client to the server before files are actually
/// created as long as the creation is triggered from within the client.
///
/// The request can return a `WorkspaceEdit` which will be applied to workspace before the
/// files are created. Hence the `WorkspaceEdit` can not manipulate the content of the file
/// to be created.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum WillCreateFilesRequest {}
impl Request for WillCreateFilesRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceWillCreateFiles;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CreateFilesParams;
    type Result = Option<WorkspaceEdit>;
}

/// The will rename files request is sent from the client to the server before files are actually
/// renamed as long as the rename is triggered from within the client.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum WillRenameFilesRequest {}
impl Request for WillRenameFilesRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceWillRenameFiles;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = RenameFilesParams;
    type Result = Option<WorkspaceEdit>;
}

/// The did delete files notification is sent from the client to the server when
/// files were deleted from within the client.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum WillDeleteFilesRequest {}
impl Request for WillDeleteFilesRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceWillDeleteFiles;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DeleteFilesParams;
    type Result = Option<WorkspaceEdit>;
}

/// A request to get the moniker of a symbol at a given text document position.
/// The request parameter is of type [`TextDocumentPositionParams`].
/// The response is of type [Moniker[]][Moniker] or `null`.
#[derive(Debug)]
pub enum MonikerRequest {}
impl Request for MonikerRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentMoniker;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = MonikerParams;
    type Result = Option<Vec<Moniker>>;
}
impl RequestWithPartialResults for MonikerRequest {
    type PartialResult = Vec<Moniker>;
}

/// A request to result a `TypeHierarchyItem` in a document at a given position.
/// Can be used as an input to a subtypes or supertypes type hierarchy.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum TypeHierarchyPrepareRequest {}
impl Request for TypeHierarchyPrepareRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentPrepareTypeHierarchy;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = TypeHierarchyPrepareParams;
    type Result = Option<Vec<TypeHierarchyItem>>;
}

/// A request to resolve the supertypes for a given `TypeHierarchyItem`.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum TypeHierarchySupertypesRequest {}
impl Request for TypeHierarchySupertypesRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TypeHierarchySupertypes;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = TypeHierarchySupertypesParams;
    type Result = Option<Vec<TypeHierarchyItem>>;
}
impl RequestWithPartialResults for TypeHierarchySupertypesRequest {
    type PartialResult = Vec<TypeHierarchyItem>;
}

/// A request to resolve the subtypes for a given `TypeHierarchyItem`.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum TypeHierarchySubtypesRequest {}
impl Request for TypeHierarchySubtypesRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TypeHierarchySubtypes;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = TypeHierarchySubtypesParams;
    type Result = Option<Vec<TypeHierarchyItem>>;
}
impl RequestWithPartialResults for TypeHierarchySubtypesRequest {
    type PartialResult = Vec<TypeHierarchyItem>;
}

/// A request to provide inline values in a document. The request's parameter is of
/// type [`InlineValueParams`], the response is of type
/// [InlineValue[]][InlineValue] or a Thenable that resolves to such.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum InlineValueRequest {}
impl Request for InlineValueRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentInlineValue;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = InlineValueParams;
    type Result = Option<Vec<InlineValue>>;
}
impl RequestWithPartialResults for InlineValueRequest {
    type PartialResult = Vec<InlineValue>;
}

/// @since 3.17.0
#[derive(Debug)]
pub enum InlineValueRefreshRequest {}
impl Request for InlineValueRefreshRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceInlineValueRefresh;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ();
    type Result = ();
}

/// A request to provide inlay hints in a document. The request's parameter is of
/// type [`InlayHintsParams`], the response is of type
/// [InlayHint[]][InlayHint] or a Thenable that resolves to such.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum InlayHintRequest {}
impl Request for InlayHintRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentInlayHint;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = InlayHintParams;
    type Result = Option<Vec<InlayHint>>;
}
impl RequestWithPartialResults for InlayHintRequest {
    type PartialResult = Vec<InlayHint>;
}

/// A request to resolve additional properties for an inlay hint.
/// The request's parameter is of type [`InlayHint`], the response is
/// of type [`InlayHint`] or a Thenable that resolves to such.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum InlayHintResolveRequest {}
impl Request for InlayHintResolveRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::InlayHintResolve;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = InlayHint;
    type Result = InlayHint;
}

/// @since 3.17.0
#[derive(Debug)]
pub enum InlayHintRefreshRequest {}
impl Request for InlayHintRefreshRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceInlayHintRefresh;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ();
    type Result = ();
}

/// The document diagnostic request definition.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum DocumentDiagnosticRequest {}
impl Request for DocumentDiagnosticRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentDiagnostic;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentDiagnosticParams;
    type Result = DocumentDiagnosticReport;
}
impl RequestWithPartialResults for DocumentDiagnosticRequest {
    type PartialResult = DocumentDiagnosticReportPartialResult;
}

/// The workspace diagnostic request definition.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum WorkspaceDiagnosticRequest {}
impl Request for WorkspaceDiagnosticRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceDiagnostic;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = WorkspaceDiagnosticParams;
    type Result = WorkspaceDiagnosticReport;
}
impl RequestWithPartialResults for WorkspaceDiagnosticRequest {
    type PartialResult = WorkspaceDiagnosticReportPartialResult;
}

/// The diagnostic refresh request definition.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum DiagnosticRefreshRequest {}
impl Request for DiagnosticRefreshRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceDiagnosticRefresh;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ();
    type Result = ();
}

/// A request to provide inline completions in a document. The request's parameter is of
/// type [`InlineCompletionParams`], the response is of type
/// [InlineCompletion[]][InlineCompletion] or a Thenable that resolves to such.
///
/// @since 3.18.0
#[derive(Debug)]
pub enum InlineCompletionRequest {}
impl Request for InlineCompletionRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentInlineCompletion;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = InlineCompletionParams;
    type Result = Option<InlineCompletionResponse>;
}
impl RequestWithPartialResults for InlineCompletionRequest {
    type PartialResult = Vec<InlineCompletionItem>;
}

/// The `workspace/textDocumentContent` request is sent from the client to the
/// server to request the content of a text document.
///
/// @since 3.18.0
#[derive(Debug)]
pub enum TextDocumentContentRequest {}
impl Request for TextDocumentContentRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceTextDocumentContent;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = TextDocumentContentParams;
    type Result = TextDocumentContentResult;
}

/// The `workspace/textDocumentContent` request is sent from the server to the client to refresh
/// the content of a specific text document.
///
/// @since 3.18.0
#[derive(Debug)]
pub enum TextDocumentContentRefreshRequest {}
impl Request for TextDocumentContentRefreshRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceTextDocumentContentRefresh;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = TextDocumentContentRefreshParams;
    type Result = ();
}

/// The `client/registerCapability` request is sent from the server to the client to register a new capability
/// handler on the client side.
#[derive(Debug)]
pub enum RegistrationRequest {}
impl Request for RegistrationRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::ClientRegisterCapability;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = RegistrationParams;
    type Result = ();
}

/// The `client/unregisterCapability` request is sent from the server to the client to unregister a previously registered capability
/// handler on the client side.
#[derive(Debug)]
pub enum UnregistrationRequest {}
impl Request for UnregistrationRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::ClientUnregisterCapability;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = UnregistrationParams;
    type Result = ();
}

/// The initialize request is sent from the client to the server.
/// It is sent once as the request after starting up the server.
/// The requests parameter is of type [`InitializeParams`]
/// the response if of type [`InitializeResult`] of a Thenable that
/// resolves to such.
#[derive(Debug)]
pub enum InitializeRequest {}
impl Request for InitializeRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::Initialize;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = InitializeParams;
    type Result = InitializeResult;
}

/// A shutdown request is sent from the client to the server.
/// It is sent once when the client decides to shutdown the
/// server. The only notification that is sent after a shutdown request
/// is the exit event.
#[derive(Debug)]
pub enum ShutdownRequest {}
impl Request for ShutdownRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::Shutdown;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = ();
    type Result = ();
}

/// The show message request is sent from the server to the client to show a message
/// and a set of options actions to the user.
#[derive(Debug)]
pub enum ShowMessageRequest {}
impl Request for ShowMessageRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WindowShowMessageRequest;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ShowMessageRequestParams;
    type Result = Option<MessageActionItem>;
}

/// A document will save request is sent from the client to the server before
/// the document is actually saved. The request can return an array of TextEdits
/// which will be applied to the text document before it is saved. Please note that
/// clients might drop results if computing the text edits took too long or if a
/// server constantly fails on this request. This is done to keep the save fast and
/// reliable.
#[derive(Debug)]
pub enum WillSaveTextDocumentWaitUntilRequest {}
impl Request for WillSaveTextDocumentWaitUntilRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentWillSaveWaitUntil;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = WillSaveTextDocumentParams;
    type Result = Option<Vec<TextEdit>>;
}

/// Request to request completion at a given text document position. The request's
/// parameter is of type [`TextDocumentPosition`] the response
/// is of type [CompletionItem[]][CompletionItem] or [`CompletionList`]
/// or a Thenable that resolves to such.
///
/// The request can delay the computation of the [`detail`][`CompletionItem::detail`]
/// and [`documentation`][`CompletionItem::documentation`] properties to the `completionItem/resolve`
/// request. However, properties that are needed for the initial sorting and filtering, like `sortText`,
/// `filterText`, `insertText`, and `textEdit`, must not be changed during resolve.
#[derive(Debug)]
pub enum CompletionRequest {}
impl Request for CompletionRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentCompletion;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CompletionParams;
    type Result = Option<CompletionResponse>;
}
impl RequestWithPartialResults for CompletionRequest {
    type PartialResult = Vec<CompletionItem>;
}

/// Request to resolve additional information for a given completion item.The request's
/// parameter is of type [`CompletionItem`] the response
/// is of type [`CompletionItem`] or a Thenable that resolves to such.
#[derive(Debug)]
pub enum CompletionResolveRequest {}
impl Request for CompletionResolveRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::CompletionItemResolve;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CompletionItem;
    type Result = CompletionItem;
}

/// Request to request hover information at a given text document position. The request's
/// parameter is of type [`TextDocumentPosition`] the response is of
/// type [`Hover`] or a Thenable that resolves to such.
#[derive(Debug)]
pub enum HoverRequest {}
impl Request for HoverRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentHover;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = HoverParams;
    type Result = Option<Hover>;
}

#[derive(Debug)]
pub enum SignatureHelpRequest {}
impl Request for SignatureHelpRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentSignatureHelp;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = SignatureHelpParams;
    type Result = Option<SignatureHelp>;
}

/// A request to resolve the definition location of a symbol at a given text
/// document position. The request's parameter is of type [`TextDocumentPosition`]
/// the response is of either type [`Definition`] or a typed array of
/// [`DefinitionLink`] or a Thenable that resolves to such.
#[derive(Debug)]
pub enum DefinitionRequest {}
impl Request for DefinitionRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentDefinition;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DefinitionParams;
    type Result = Option<DefinitionResponse>;
}
impl RequestWithPartialResults for DefinitionRequest {
    type PartialResult = DefinitionPartialResponse;
}

/// A request to resolve project-wide references for the symbol denoted
/// by the given text document position. The request's parameter is of
/// type [`ReferenceParams`] the response is of type
/// [Location[]][Location] or a Thenable that resolves to such.
#[derive(Debug)]
pub enum ReferencesRequest {}
impl Request for ReferencesRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentReferences;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = ReferenceParams;
    type Result = Option<Vec<Location>>;
}
impl RequestWithPartialResults for ReferencesRequest {
    type PartialResult = Vec<Location>;
}

/// Request to resolve a [`DocumentHighlight`] for a given
/// text document position. The request's parameter is of type [`TextDocumentPosition`]
/// the request response is an array of type [`DocumentHighlight`]
/// or a Thenable that resolves to such.
#[derive(Debug)]
pub enum DocumentHighlightRequest {}
impl Request for DocumentHighlightRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentDocumentHighlight;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentHighlightParams;
    type Result = Option<Vec<DocumentHighlight>>;
}
impl RequestWithPartialResults for DocumentHighlightRequest {
    type PartialResult = Vec<DocumentHighlight>;
}

/// A request to list all symbols found in a given text document. The request's
/// parameter is of type [`TextDocumentIdentifier`] the
/// response is of type [SymbolInformation[]][SymbolInformation] or a Thenable
/// that resolves to such.
#[derive(Debug)]
pub enum DocumentSymbolRequest {}
impl Request for DocumentSymbolRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentDocumentSymbol;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentSymbolParams;
    type Result = Option<DocumentSymbolResponse>;
}
impl RequestWithPartialResults for DocumentSymbolRequest {
    type PartialResult = DocumentSymbolPartialResponse;
}

/// A request to provide commands for the given text document and range.
#[derive(Debug)]
pub enum CodeActionRequest {}
impl Request for CodeActionRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentCodeAction;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CodeActionParams;
    type Result = Option<Vec<CodeActionResponse>>;
}
impl RequestWithPartialResults for CodeActionRequest {
    type PartialResult = Vec<CodeActionPartialResponse>;
}

/// Request to resolve additional information for a given code action.The request's
/// parameter is of type [`CodeAction`] the response
/// is of type [`CodeAction`] or a Thenable that resolves to such.
#[derive(Debug)]
pub enum CodeActionResolveRequest {}
impl Request for CodeActionResolveRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::CodeActionResolve;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CodeAction;
    type Result = CodeAction;
}

/// A request to list project-wide symbols matching the query string given
/// by the [`WorkspaceSymbolParams`]. The response is
/// of type [SymbolInformation[]][SymbolInformation] or a Thenable that
/// resolves to such.
///
/// @since 3.17.0 - support for WorkspaceSymbol in the returned data. Clients
///  need to advertise support for WorkspaceSymbols via the client capability
///  `workspace.symbol.resolveSupport`.
///
#[derive(Debug)]
pub enum WorkspaceSymbolRequest {}
impl Request for WorkspaceSymbolRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceSymbol;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = WorkspaceSymbolParams;
    type Result = Option<WorkspaceSymbolResponse>;
}
impl RequestWithPartialResults for WorkspaceSymbolRequest {
    type PartialResult = WorkspaceSymbolPartialResponse;
}

/// A request to resolve the range inside the workspace
/// symbol's location.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum WorkspaceSymbolResolveRequest {}
impl Request for WorkspaceSymbolResolveRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceSymbolResolve;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = WorkspaceSymbol;
    type Result = WorkspaceSymbol;
}

/// A request to provide code lens for the given text document.
#[derive(Debug)]
pub enum CodeLensRequest {}
impl Request for CodeLensRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentCodeLens;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CodeLensParams;
    type Result = Option<Vec<CodeLens>>;
}
impl RequestWithPartialResults for CodeLensRequest {
    type PartialResult = Vec<CodeLens>;
}

/// A request to resolve a command for a given code lens.
#[derive(Debug)]
pub enum CodeLensResolveRequest {}
impl Request for CodeLensResolveRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::CodeLensResolve;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CodeLens;
    type Result = CodeLens;
}

/// A request to refresh all code actions
///
/// @since 3.16.0
#[derive(Debug)]
pub enum CodeLensRefreshRequest {}
impl Request for CodeLensRefreshRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceCodeLensRefresh;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ();
    type Result = ();
}

/// A request to provide document links
#[derive(Debug)]
pub enum DocumentLinkRequest {}
impl Request for DocumentLinkRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentDocumentLink;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentLinkParams;
    type Result = Option<Vec<DocumentLink>>;
}
impl RequestWithPartialResults for DocumentLinkRequest {
    type PartialResult = Vec<DocumentLink>;
}

/// Request to resolve additional information for a given document link. The request's
/// parameter is of type [`DocumentLink`] the response
/// is of type [`DocumentLink`] or a Thenable that resolves to such.
#[derive(Debug)]
pub enum DocumentLinkResolveRequest {}
impl Request for DocumentLinkResolveRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::DocumentLinkResolve;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentLink;
    type Result = DocumentLink;
}

/// A request to format a whole document.
#[derive(Debug)]
pub enum DocumentFormattingRequest {}
impl Request for DocumentFormattingRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentFormatting;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentFormattingParams;
    type Result = Option<Vec<TextEdit>>;
}

/// A request to format a range in a document.
#[derive(Debug)]
pub enum DocumentRangeFormattingRequest {}
impl Request for DocumentRangeFormattingRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentRangeFormatting;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentRangeFormattingParams;
    type Result = Option<Vec<TextEdit>>;
}

/// A request to format ranges in a document.
///
/// @since 3.18.0
#[derive(Debug)]
pub enum DocumentRangesFormattingRequest {}
impl Request for DocumentRangesFormattingRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentRangesFormatting;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentRangesFormattingParams;
    type Result = Option<Vec<TextEdit>>;
}

/// A request to format a document on type.
#[derive(Debug)]
pub enum DocumentOnTypeFormattingRequest {}
impl Request for DocumentOnTypeFormattingRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentOnTypeFormatting;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DocumentOnTypeFormattingParams;
    type Result = Option<Vec<TextEdit>>;
}

/// A request to rename a symbol.
#[derive(Debug)]
pub enum RenameRequest {}
impl Request for RenameRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentRename;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = RenameParams;
    type Result = Option<WorkspaceEdit>;
}

/// A request to test and perform the setup necessary for a rename.
///
/// @since 3.16 - support for default behavior
#[derive(Debug)]
pub enum PrepareRenameRequest {}
impl Request for PrepareRenameRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::TextDocumentPrepareRename;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = PrepareRenameParams;
    type Result = Option<PrepareRenameResult>;
}

/// A request send from the client to the server to execute a command. The request might return
/// a workspace edit which the client will apply to the workspace.
#[derive(Debug)]
pub enum ExecuteCommandRequest {}
impl Request for ExecuteCommandRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceExecuteCommand;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = ExecuteCommandParams;
    type Result = Option<LspAny>;
}

/// A request sent from the server to the client to modified certain resources.
#[derive(Debug)]
pub enum ApplyWorkspaceEditRequest {}
impl Request for ApplyWorkspaceEditRequest {
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::WorkspaceApplyEdit;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ApplyWorkspaceEditParams;
    type Result = ApplyWorkspaceEditResult;
}
