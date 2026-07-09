use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// Indicates in which direction a message is sent in the protocol.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
pub enum MessageDirection {
    ClientToServer,
    ServerToClient,
    Both,
}
pub trait Notification {
    type Params: DeserializeOwned + Serialize + Send + Sync + 'static;
    const METHOD: LspNotificationMethod<'static>;
    const MESSAGE_DIRECTION: MessageDirection;
}
pub trait Request {
    type Params: DeserializeOwned + Serialize + Send + Sync + 'static;
    type Result: DeserializeOwned + Serialize + Send + Sync + 'static;
    const METHOD: LspRequestMethod<'static>;
    const MESSAGE_DIRECTION: MessageDirection;
}
pub trait RequestWithPartialResults: Request {
    type PartialResult: DeserializeOwned + Serialize + Send + Sync + 'static;
}
#[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
/// URIs are transferred as strings. The URI's format is defined in https://tools.ietf.org/html/rfc3986.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uri(pub String);
#[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
impl From<String> for Uri {
    fn from(s: String) -> Self {
        Self(s)
    }
}
#[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}
#[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
impl From<Box<str>> for Uri {
    fn from(s: Box<str>) -> Self {
        Self(s.into())
    }
}
#[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
impl From<Cow<'_, str>> for Uri {
    fn from(s: Cow<'_, str>) -> Self {
        Self(s.into())
    }
}
#[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[cfg(feature = "url")]
pub type Uri = url::Url;
#[cfg(all(feature = "fluent-uri", not(feature = "url")))]
pub type Uri = fluent_uri::Uri<String>;
#[cfg(all(feature = "url", feature = "fluent-uri"))]
compile_error!(
    "Features 'url' and 'fluent-uri' are mutually exclusive and cannot be enabled together."
);

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(into = "String", from = "&'a str")]
pub enum LspRequestMethod<'a> {
    TextDocumentImplementation,
    TextDocumentTypeDefinition,
    WorkspaceWorkspaceFolders,
    WorkspaceConfiguration,
    TextDocumentDocumentColor,
    TextDocumentColorPresentation,
    TextDocumentFoldingRange,
    WorkspaceFoldingRangeRefresh,
    TextDocumentDeclaration,
    TextDocumentSelectionRange,
    WindowWorkDoneProgressCreate,
    TextDocumentPrepareCallHierarchy,
    CallHierarchyIncomingCalls,
    CallHierarchyOutgoingCalls,
    TextDocumentSemanticTokensFull,
    TextDocumentSemanticTokensFullDelta,
    TextDocumentSemanticTokensRange,
    WorkspaceSemanticTokensRefresh,
    WindowShowDocument,
    TextDocumentLinkedEditingRange,
    WorkspaceWillCreateFiles,
    WorkspaceWillRenameFiles,
    WorkspaceWillDeleteFiles,
    TextDocumentMoniker,
    TextDocumentPrepareTypeHierarchy,
    TypeHierarchySupertypes,
    TypeHierarchySubtypes,
    TextDocumentInlineValue,
    WorkspaceInlineValueRefresh,
    TextDocumentInlayHint,
    InlayHintResolve,
    WorkspaceInlayHintRefresh,
    TextDocumentDiagnostic,
    WorkspaceDiagnostic,
    WorkspaceDiagnosticRefresh,
    TextDocumentInlineCompletion,
    WorkspaceTextDocumentContent,
    WorkspaceTextDocumentContentRefresh,
    ClientRegisterCapability,
    ClientUnregisterCapability,
    Initialize,
    Shutdown,
    WindowShowMessageRequest,
    TextDocumentWillSaveWaitUntil,
    TextDocumentCompletion,
    CompletionItemResolve,
    TextDocumentHover,
    TextDocumentSignatureHelp,
    TextDocumentDefinition,
    TextDocumentReferences,
    TextDocumentDocumentHighlight,
    TextDocumentDocumentSymbol,
    TextDocumentCodeAction,
    CodeActionResolve,
    WorkspaceSymbol,
    WorkspaceSymbolResolve,
    TextDocumentCodeLens,
    CodeLensResolve,
    WorkspaceCodeLensRefresh,
    TextDocumentDocumentLink,
    DocumentLinkResolve,
    TextDocumentFormatting,
    TextDocumentRangeFormatting,
    TextDocumentRangesFormatting,
    TextDocumentOnTypeFormatting,
    TextDocumentRename,
    TextDocumentPrepareRename,
    WorkspaceExecuteCommand,
    WorkspaceApplyEdit,
    Custom(&'a str),
}
impl<'a> LspRequestMethod<'a> {
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        match self {
            Self::TextDocumentImplementation => "textDocument/implementation",
            Self::TextDocumentTypeDefinition => "textDocument/typeDefinition",
            Self::WorkspaceWorkspaceFolders => "workspace/workspaceFolders",
            Self::WorkspaceConfiguration => "workspace/configuration",
            Self::TextDocumentDocumentColor => "textDocument/documentColor",
            Self::TextDocumentColorPresentation => "textDocument/colorPresentation",
            Self::TextDocumentFoldingRange => "textDocument/foldingRange",
            Self::WorkspaceFoldingRangeRefresh => "workspace/foldingRange/refresh",
            Self::TextDocumentDeclaration => "textDocument/declaration",
            Self::TextDocumentSelectionRange => "textDocument/selectionRange",
            Self::WindowWorkDoneProgressCreate => "window/workDoneProgress/create",
            Self::TextDocumentPrepareCallHierarchy => "textDocument/prepareCallHierarchy",
            Self::CallHierarchyIncomingCalls => "callHierarchy/incomingCalls",
            Self::CallHierarchyOutgoingCalls => "callHierarchy/outgoingCalls",
            Self::TextDocumentSemanticTokensFull => "textDocument/semanticTokens/full",
            Self::TextDocumentSemanticTokensFullDelta => {
                "textDocument/semanticTokens/full/delta"
            }
            Self::TextDocumentSemanticTokensRange => "textDocument/semanticTokens/range",
            Self::WorkspaceSemanticTokensRefresh => "workspace/semanticTokens/refresh",
            Self::WindowShowDocument => "window/showDocument",
            Self::TextDocumentLinkedEditingRange => "textDocument/linkedEditingRange",
            Self::WorkspaceWillCreateFiles => "workspace/willCreateFiles",
            Self::WorkspaceWillRenameFiles => "workspace/willRenameFiles",
            Self::WorkspaceWillDeleteFiles => "workspace/willDeleteFiles",
            Self::TextDocumentMoniker => "textDocument/moniker",
            Self::TextDocumentPrepareTypeHierarchy => "textDocument/prepareTypeHierarchy",
            Self::TypeHierarchySupertypes => "typeHierarchy/supertypes",
            Self::TypeHierarchySubtypes => "typeHierarchy/subtypes",
            Self::TextDocumentInlineValue => "textDocument/inlineValue",
            Self::WorkspaceInlineValueRefresh => "workspace/inlineValue/refresh",
            Self::TextDocumentInlayHint => "textDocument/inlayHint",
            Self::InlayHintResolve => "inlayHint/resolve",
            Self::WorkspaceInlayHintRefresh => "workspace/inlayHint/refresh",
            Self::TextDocumentDiagnostic => "textDocument/diagnostic",
            Self::WorkspaceDiagnostic => "workspace/diagnostic",
            Self::WorkspaceDiagnosticRefresh => "workspace/diagnostic/refresh",
            Self::TextDocumentInlineCompletion => "textDocument/inlineCompletion",
            Self::WorkspaceTextDocumentContent => "workspace/textDocumentContent",
            Self::WorkspaceTextDocumentContentRefresh => {
                "workspace/textDocumentContent/refresh"
            }
            Self::ClientRegisterCapability => "client/registerCapability",
            Self::ClientUnregisterCapability => "client/unregisterCapability",
            Self::Initialize => "initialize",
            Self::Shutdown => "shutdown",
            Self::WindowShowMessageRequest => "window/showMessageRequest",
            Self::TextDocumentWillSaveWaitUntil => "textDocument/willSaveWaitUntil",
            Self::TextDocumentCompletion => "textDocument/completion",
            Self::CompletionItemResolve => "completionItem/resolve",
            Self::TextDocumentHover => "textDocument/hover",
            Self::TextDocumentSignatureHelp => "textDocument/signatureHelp",
            Self::TextDocumentDefinition => "textDocument/definition",
            Self::TextDocumentReferences => "textDocument/references",
            Self::TextDocumentDocumentHighlight => "textDocument/documentHighlight",
            Self::TextDocumentDocumentSymbol => "textDocument/documentSymbol",
            Self::TextDocumentCodeAction => "textDocument/codeAction",
            Self::CodeActionResolve => "codeAction/resolve",
            Self::WorkspaceSymbol => "workspace/symbol",
            Self::WorkspaceSymbolResolve => "workspaceSymbol/resolve",
            Self::TextDocumentCodeLens => "textDocument/codeLens",
            Self::CodeLensResolve => "codeLens/resolve",
            Self::WorkspaceCodeLensRefresh => "workspace/codeLens/refresh",
            Self::TextDocumentDocumentLink => "textDocument/documentLink",
            Self::DocumentLinkResolve => "documentLink/resolve",
            Self::TextDocumentFormatting => "textDocument/formatting",
            Self::TextDocumentRangeFormatting => "textDocument/rangeFormatting",
            Self::TextDocumentRangesFormatting => "textDocument/rangesFormatting",
            Self::TextDocumentOnTypeFormatting => "textDocument/onTypeFormatting",
            Self::TextDocumentRename => "textDocument/rename",
            Self::TextDocumentPrepareRename => "textDocument/prepareRename",
            Self::WorkspaceExecuteCommand => "workspace/executeCommand",
            Self::WorkspaceApplyEdit => "workspace/applyEdit",
            Self::Custom(custom) => custom,
        }
    }
    /// Creates a new [LspRequestMethod]. The created variant will **always** be [LspRequestMethod::Custom].
    #[must_use]
    pub const fn new(value: &'a str) -> Self {
        Self::Custom(value)
    }
}
impl<'a> From<&'a str> for LspRequestMethod<'a> {
    /// Creates a new [LspRequestMethod] from a `&str`. The created variant will be
    /// [LspRequestMethod::Custom] **if and only if** the `&str` does not match an
    /// existing [LspRequestMethod].
    fn from(value: &'a str) -> Self {
        match value {
            "textDocument/implementation" => Self::TextDocumentImplementation,
            "textDocument/typeDefinition" => Self::TextDocumentTypeDefinition,
            "workspace/workspaceFolders" => Self::WorkspaceWorkspaceFolders,
            "workspace/configuration" => Self::WorkspaceConfiguration,
            "textDocument/documentColor" => Self::TextDocumentDocumentColor,
            "textDocument/colorPresentation" => Self::TextDocumentColorPresentation,
            "textDocument/foldingRange" => Self::TextDocumentFoldingRange,
            "workspace/foldingRange/refresh" => Self::WorkspaceFoldingRangeRefresh,
            "textDocument/declaration" => Self::TextDocumentDeclaration,
            "textDocument/selectionRange" => Self::TextDocumentSelectionRange,
            "window/workDoneProgress/create" => Self::WindowWorkDoneProgressCreate,
            "textDocument/prepareCallHierarchy" => Self::TextDocumentPrepareCallHierarchy,
            "callHierarchy/incomingCalls" => Self::CallHierarchyIncomingCalls,
            "callHierarchy/outgoingCalls" => Self::CallHierarchyOutgoingCalls,
            "textDocument/semanticTokens/full" => Self::TextDocumentSemanticTokensFull,
            "textDocument/semanticTokens/full/delta" => {
                Self::TextDocumentSemanticTokensFullDelta
            }
            "textDocument/semanticTokens/range" => Self::TextDocumentSemanticTokensRange,
            "workspace/semanticTokens/refresh" => Self::WorkspaceSemanticTokensRefresh,
            "window/showDocument" => Self::WindowShowDocument,
            "textDocument/linkedEditingRange" => Self::TextDocumentLinkedEditingRange,
            "workspace/willCreateFiles" => Self::WorkspaceWillCreateFiles,
            "workspace/willRenameFiles" => Self::WorkspaceWillRenameFiles,
            "workspace/willDeleteFiles" => Self::WorkspaceWillDeleteFiles,
            "textDocument/moniker" => Self::TextDocumentMoniker,
            "textDocument/prepareTypeHierarchy" => Self::TextDocumentPrepareTypeHierarchy,
            "typeHierarchy/supertypes" => Self::TypeHierarchySupertypes,
            "typeHierarchy/subtypes" => Self::TypeHierarchySubtypes,
            "textDocument/inlineValue" => Self::TextDocumentInlineValue,
            "workspace/inlineValue/refresh" => Self::WorkspaceInlineValueRefresh,
            "textDocument/inlayHint" => Self::TextDocumentInlayHint,
            "inlayHint/resolve" => Self::InlayHintResolve,
            "workspace/inlayHint/refresh" => Self::WorkspaceInlayHintRefresh,
            "textDocument/diagnostic" => Self::TextDocumentDiagnostic,
            "workspace/diagnostic" => Self::WorkspaceDiagnostic,
            "workspace/diagnostic/refresh" => Self::WorkspaceDiagnosticRefresh,
            "textDocument/inlineCompletion" => Self::TextDocumentInlineCompletion,
            "workspace/textDocumentContent" => Self::WorkspaceTextDocumentContent,
            "workspace/textDocumentContent/refresh" => {
                Self::WorkspaceTextDocumentContentRefresh
            }
            "client/registerCapability" => Self::ClientRegisterCapability,
            "client/unregisterCapability" => Self::ClientUnregisterCapability,
            "initialize" => Self::Initialize,
            "shutdown" => Self::Shutdown,
            "window/showMessageRequest" => Self::WindowShowMessageRequest,
            "textDocument/willSaveWaitUntil" => Self::TextDocumentWillSaveWaitUntil,
            "textDocument/completion" => Self::TextDocumentCompletion,
            "completionItem/resolve" => Self::CompletionItemResolve,
            "textDocument/hover" => Self::TextDocumentHover,
            "textDocument/signatureHelp" => Self::TextDocumentSignatureHelp,
            "textDocument/definition" => Self::TextDocumentDefinition,
            "textDocument/references" => Self::TextDocumentReferences,
            "textDocument/documentHighlight" => Self::TextDocumentDocumentHighlight,
            "textDocument/documentSymbol" => Self::TextDocumentDocumentSymbol,
            "textDocument/codeAction" => Self::TextDocumentCodeAction,
            "codeAction/resolve" => Self::CodeActionResolve,
            "workspace/symbol" => Self::WorkspaceSymbol,
            "workspaceSymbol/resolve" => Self::WorkspaceSymbolResolve,
            "textDocument/codeLens" => Self::TextDocumentCodeLens,
            "codeLens/resolve" => Self::CodeLensResolve,
            "workspace/codeLens/refresh" => Self::WorkspaceCodeLensRefresh,
            "textDocument/documentLink" => Self::TextDocumentDocumentLink,
            "documentLink/resolve" => Self::DocumentLinkResolve,
            "textDocument/formatting" => Self::TextDocumentFormatting,
            "textDocument/rangeFormatting" => Self::TextDocumentRangeFormatting,
            "textDocument/rangesFormatting" => Self::TextDocumentRangesFormatting,
            "textDocument/onTypeFormatting" => Self::TextDocumentOnTypeFormatting,
            "textDocument/rename" => Self::TextDocumentRename,
            "textDocument/prepareRename" => Self::TextDocumentPrepareRename,
            "workspace/executeCommand" => Self::WorkspaceExecuteCommand,
            "workspace/applyEdit" => Self::WorkspaceApplyEdit,
            _ => Self::Custom(value),
        }
    }
}
impl<'a> From<LspRequestMethod<'a>> for String {
    fn from(value: LspRequestMethod<'a>) -> Self {
        value.as_str().to_owned()
    }
}
impl fmt::Display for LspRequestMethod<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.as_str();
        write!(f, "{s}")
    }
}
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(into = "String", from = "&'a str")]
pub enum LspNotificationMethod<'a> {
    WorkspaceDidChangeWorkspaceFolders,
    WindowWorkDoneProgressCancel,
    WorkspaceDidCreateFiles,
    WorkspaceDidRenameFiles,
    WorkspaceDidDeleteFiles,
    NotebookDocumentDidOpen,
    NotebookDocumentDidChange,
    NotebookDocumentDidSave,
    NotebookDocumentDidClose,
    Initialized,
    Exit,
    WorkspaceDidChangeConfiguration,
    WindowShowMessage,
    WindowLogMessage,
    TelemetryEvent,
    TextDocumentDidOpen,
    TextDocumentDidChange,
    TextDocumentDidClose,
    TextDocumentDidSave,
    TextDocumentWillSave,
    WorkspaceDidChangeWatchedFiles,
    TextDocumentPublishDiagnostics,
    SetTrace,
    LogTrace,
    CancelRequest,
    Progress,
    Custom(&'a str),
}
impl<'a> LspNotificationMethod<'a> {
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        match self {
            Self::WorkspaceDidChangeWorkspaceFolders => {
                "workspace/didChangeWorkspaceFolders"
            }
            Self::WindowWorkDoneProgressCancel => "window/workDoneProgress/cancel",
            Self::WorkspaceDidCreateFiles => "workspace/didCreateFiles",
            Self::WorkspaceDidRenameFiles => "workspace/didRenameFiles",
            Self::WorkspaceDidDeleteFiles => "workspace/didDeleteFiles",
            Self::NotebookDocumentDidOpen => "notebookDocument/didOpen",
            Self::NotebookDocumentDidChange => "notebookDocument/didChange",
            Self::NotebookDocumentDidSave => "notebookDocument/didSave",
            Self::NotebookDocumentDidClose => "notebookDocument/didClose",
            Self::Initialized => "initialized",
            Self::Exit => "exit",
            Self::WorkspaceDidChangeConfiguration => "workspace/didChangeConfiguration",
            Self::WindowShowMessage => "window/showMessage",
            Self::WindowLogMessage => "window/logMessage",
            Self::TelemetryEvent => "telemetry/event",
            Self::TextDocumentDidOpen => "textDocument/didOpen",
            Self::TextDocumentDidChange => "textDocument/didChange",
            Self::TextDocumentDidClose => "textDocument/didClose",
            Self::TextDocumentDidSave => "textDocument/didSave",
            Self::TextDocumentWillSave => "textDocument/willSave",
            Self::WorkspaceDidChangeWatchedFiles => "workspace/didChangeWatchedFiles",
            Self::TextDocumentPublishDiagnostics => "textDocument/publishDiagnostics",
            Self::SetTrace => "$/setTrace",
            Self::LogTrace => "$/logTrace",
            Self::CancelRequest => "$/cancelRequest",
            Self::Progress => "$/progress",
            Self::Custom(custom) => custom,
        }
    }
    /// Creates a new [LspNotificationMethod]. The created variant will **always** be [LspNotificationMethod::Custom].
    #[must_use]
    pub const fn new(value: &'a str) -> Self {
        Self::Custom(value)
    }
}
impl<'a> From<&'a str> for LspNotificationMethod<'a> {
    /// Creates a new [LspNotificationMethod] from a `&str`. The created variant will be
    /// [LspNotificationMethod::Custom] **if and only if** the `&str` does not match an
    /// existing [LspNotificationMethod].
    fn from(value: &'a str) -> Self {
        match value {
            "workspace/didChangeWorkspaceFolders" => {
                Self::WorkspaceDidChangeWorkspaceFolders
            }
            "window/workDoneProgress/cancel" => Self::WindowWorkDoneProgressCancel,
            "workspace/didCreateFiles" => Self::WorkspaceDidCreateFiles,
            "workspace/didRenameFiles" => Self::WorkspaceDidRenameFiles,
            "workspace/didDeleteFiles" => Self::WorkspaceDidDeleteFiles,
            "notebookDocument/didOpen" => Self::NotebookDocumentDidOpen,
            "notebookDocument/didChange" => Self::NotebookDocumentDidChange,
            "notebookDocument/didSave" => Self::NotebookDocumentDidSave,
            "notebookDocument/didClose" => Self::NotebookDocumentDidClose,
            "initialized" => Self::Initialized,
            "exit" => Self::Exit,
            "workspace/didChangeConfiguration" => Self::WorkspaceDidChangeConfiguration,
            "window/showMessage" => Self::WindowShowMessage,
            "window/logMessage" => Self::WindowLogMessage,
            "telemetry/event" => Self::TelemetryEvent,
            "textDocument/didOpen" => Self::TextDocumentDidOpen,
            "textDocument/didChange" => Self::TextDocumentDidChange,
            "textDocument/didClose" => Self::TextDocumentDidClose,
            "textDocument/didSave" => Self::TextDocumentDidSave,
            "textDocument/willSave" => Self::TextDocumentWillSave,
            "workspace/didChangeWatchedFiles" => Self::WorkspaceDidChangeWatchedFiles,
            "textDocument/publishDiagnostics" => Self::TextDocumentPublishDiagnostics,
            "$/setTrace" => Self::SetTrace,
            "$/logTrace" => Self::LogTrace,
            "$/cancelRequest" => Self::CancelRequest,
            "$/progress" => Self::Progress,
            _ => Self::Custom(value),
        }
    }
}
impl<'a> From<LspNotificationMethod<'a>> for String {
    fn from(value: LspNotificationMethod<'a>) -> Self {
        value.as_str().to_owned()
    }
}
impl fmt::Display for LspNotificationMethod<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.as_str();
        write!(f, "{s}")
    }
}

/// Get the [`Request`] type for a request method.
///
/// Example:
///
/// ```
/// use gen_lsp_types::{Request, lsp_request};
/// let params: <lsp_request!("textDocument/formatting") as Request>::Params;
/// ```
#[macro_export]
macro_rules! lsp_request {
    ("textDocument/implementation") => {
        $crate::ImplementationRequest
    };
    ("textDocument/typeDefinition") => {
        $crate::TypeDefinitionRequest
    };
    ("workspace/workspaceFolders") => {
        $crate::WorkspaceFoldersRequest
    };
    ("workspace/configuration") => {
        $crate::ConfigurationRequest
    };
    ("textDocument/documentColor") => {
        $crate::DocumentColorRequest
    };
    ("textDocument/colorPresentation") => {
        $crate::ColorPresentationRequest
    };
    ("textDocument/foldingRange") => {
        $crate::FoldingRangeRequest
    };
    ("workspace/foldingRange/refresh") => {
        $crate::FoldingRangeRefreshRequest
    };
    ("textDocument/declaration") => {
        $crate::DeclarationRequest
    };
    ("textDocument/selectionRange") => {
        $crate::SelectionRangeRequest
    };
    ("window/workDoneProgress/create") => {
        $crate::WorkDoneProgressCreateRequest
    };
    ("textDocument/prepareCallHierarchy") => {
        $crate::CallHierarchyPrepareRequest
    };
    ("callHierarchy/incomingCalls") => {
        $crate::CallHierarchyIncomingCallsRequest
    };
    ("callHierarchy/outgoingCalls") => {
        $crate::CallHierarchyOutgoingCallsRequest
    };
    ("textDocument/semanticTokens/full") => {
        $crate::SemanticTokensRequest
    };
    ("textDocument/semanticTokens/full/delta") => {
        $crate::SemanticTokensDeltaRequest
    };
    ("textDocument/semanticTokens/range") => {
        $crate::SemanticTokensRangeRequest
    };
    ("workspace/semanticTokens/refresh") => {
        $crate::SemanticTokensRefreshRequest
    };
    ("window/showDocument") => {
        $crate::ShowDocumentRequest
    };
    ("textDocument/linkedEditingRange") => {
        $crate::LinkedEditingRangeRequest
    };
    ("workspace/willCreateFiles") => {
        $crate::WillCreateFilesRequest
    };
    ("workspace/willRenameFiles") => {
        $crate::WillRenameFilesRequest
    };
    ("workspace/willDeleteFiles") => {
        $crate::WillDeleteFilesRequest
    };
    ("textDocument/moniker") => {
        $crate::MonikerRequest
    };
    ("textDocument/prepareTypeHierarchy") => {
        $crate::TypeHierarchyPrepareRequest
    };
    ("typeHierarchy/supertypes") => {
        $crate::TypeHierarchySupertypesRequest
    };
    ("typeHierarchy/subtypes") => {
        $crate::TypeHierarchySubtypesRequest
    };
    ("textDocument/inlineValue") => {
        $crate::InlineValueRequest
    };
    ("workspace/inlineValue/refresh") => {
        $crate::InlineValueRefreshRequest
    };
    ("textDocument/inlayHint") => {
        $crate::InlayHintRequest
    };
    ("inlayHint/resolve") => {
        $crate::InlayHintResolveRequest
    };
    ("workspace/inlayHint/refresh") => {
        $crate::InlayHintRefreshRequest
    };
    ("textDocument/diagnostic") => {
        $crate::DocumentDiagnosticRequest
    };
    ("workspace/diagnostic") => {
        $crate::WorkspaceDiagnosticRequest
    };
    ("workspace/diagnostic/refresh") => {
        $crate::DiagnosticRefreshRequest
    };
    ("textDocument/inlineCompletion") => {
        $crate::InlineCompletionRequest
    };
    ("workspace/textDocumentContent") => {
        $crate::TextDocumentContentRequest
    };
    ("workspace/textDocumentContent/refresh") => {
        $crate::TextDocumentContentRefreshRequest
    };
    ("client/registerCapability") => {
        $crate::RegistrationRequest
    };
    ("client/unregisterCapability") => {
        $crate::UnregistrationRequest
    };
    ("initialize") => {
        $crate::InitializeRequest
    };
    ("shutdown") => {
        $crate::ShutdownRequest
    };
    ("window/showMessageRequest") => {
        $crate::ShowMessageRequest
    };
    ("textDocument/willSaveWaitUntil") => {
        $crate::WillSaveTextDocumentWaitUntilRequest
    };
    ("textDocument/completion") => {
        $crate::CompletionRequest
    };
    ("completionItem/resolve") => {
        $crate::CompletionResolveRequest
    };
    ("textDocument/hover") => {
        $crate::HoverRequest
    };
    ("textDocument/signatureHelp") => {
        $crate::SignatureHelpRequest
    };
    ("textDocument/definition") => {
        $crate::DefinitionRequest
    };
    ("textDocument/references") => {
        $crate::ReferencesRequest
    };
    ("textDocument/documentHighlight") => {
        $crate::DocumentHighlightRequest
    };
    ("textDocument/documentSymbol") => {
        $crate::DocumentSymbolRequest
    };
    ("textDocument/codeAction") => {
        $crate::CodeActionRequest
    };
    ("codeAction/resolve") => {
        $crate::CodeActionResolveRequest
    };
    ("workspace/symbol") => {
        $crate::WorkspaceSymbolRequest
    };
    ("workspaceSymbol/resolve") => {
        $crate::WorkspaceSymbolResolveRequest
    };
    ("textDocument/codeLens") => {
        $crate::CodeLensRequest
    };
    ("codeLens/resolve") => {
        $crate::CodeLensResolveRequest
    };
    ("workspace/codeLens/refresh") => {
        $crate::CodeLensRefreshRequest
    };
    ("textDocument/documentLink") => {
        $crate::DocumentLinkRequest
    };
    ("documentLink/resolve") => {
        $crate::DocumentLinkResolveRequest
    };
    ("textDocument/formatting") => {
        $crate::DocumentFormattingRequest
    };
    ("textDocument/rangeFormatting") => {
        $crate::DocumentRangeFormattingRequest
    };
    ("textDocument/rangesFormatting") => {
        $crate::DocumentRangesFormattingRequest
    };
    ("textDocument/onTypeFormatting") => {
        $crate::DocumentOnTypeFormattingRequest
    };
    ("textDocument/rename") => {
        $crate::RenameRequest
    };
    ("textDocument/prepareRename") => {
        $crate::PrepareRenameRequest
    };
    ("workspace/executeCommand") => {
        $crate::ExecuteCommandRequest
    };
    ("workspace/applyEdit") => {
        $crate::ApplyWorkspaceEditRequest
    };
}

/// Provides generic access to [`crate::Uri`]s
pub trait WithUri {
    fn uri(&self) -> &Uri;
}
impl WithUri for Uri {
    fn uri(&self) -> &Uri {
        self
    }
}

/// Get the [`Notification`] type for a notification method.
///
/// Example:
///
/// ```
/// use gen_lsp_types::{Notification, lsp_notification};
/// let params: <lsp_notification!("textDocument/didChange") as Notification>::Params;
/// ```
#[macro_export]
macro_rules! lsp_notification {
    ("workspace/didChangeWorkspaceFolders") => {
        $crate::DidChangeWorkspaceFoldersNotification
    };
    ("window/workDoneProgress/cancel") => {
        $crate::WorkDoneProgressCancelNotification
    };
    ("workspace/didCreateFiles") => {
        $crate::DidCreateFilesNotification
    };
    ("workspace/didRenameFiles") => {
        $crate::DidRenameFilesNotification
    };
    ("workspace/didDeleteFiles") => {
        $crate::DidDeleteFilesNotification
    };
    ("notebookDocument/didOpen") => {
        $crate::DidOpenNotebookDocumentNotification
    };
    ("notebookDocument/didChange") => {
        $crate::DidChangeNotebookDocumentNotification
    };
    ("notebookDocument/didSave") => {
        $crate::DidSaveNotebookDocumentNotification
    };
    ("notebookDocument/didClose") => {
        $crate::DidCloseNotebookDocumentNotification
    };
    ("initialized") => {
        $crate::InitializedNotification
    };
    ("exit") => {
        $crate::ExitNotification
    };
    ("workspace/didChangeConfiguration") => {
        $crate::DidChangeConfigurationNotification
    };
    ("window/showMessage") => {
        $crate::ShowMessageNotification
    };
    ("window/logMessage") => {
        $crate::LogMessageNotification
    };
    ("telemetry/event") => {
        $crate::TelemetryEventNotification
    };
    ("textDocument/didOpen") => {
        $crate::DidOpenTextDocumentNotification
    };
    ("textDocument/didChange") => {
        $crate::DidChangeTextDocumentNotification
    };
    ("textDocument/didClose") => {
        $crate::DidCloseTextDocumentNotification
    };
    ("textDocument/didSave") => {
        $crate::DidSaveTextDocumentNotification
    };
    ("textDocument/willSave") => {
        $crate::WillSaveTextDocumentNotification
    };
    ("workspace/didChangeWatchedFiles") => {
        $crate::DidChangeWatchedFilesNotification
    };
    ("textDocument/publishDiagnostics") => {
        $crate::PublishDiagnosticsNotification
    };
    ("$/setTrace") => {
        $crate::SetTraceNotification
    };
    ("$/logTrace") => {
        $crate::LogTraceNotification
    };
    ("$/cancelRequest") => {
        $crate::CancelNotification
    };
    ("$/progress") => {
        $crate::ProgressNotification
    };
}
