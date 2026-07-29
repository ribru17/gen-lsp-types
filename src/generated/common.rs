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
    (
        params "textDocument/implementation" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::ImplementationParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/implementation"($($expr:expr),* $(,)?)) => {
        $crate::ImplementationParams::new($($expr,)*)
    };
    ("textDocument/typeDefinition") => {
        $crate::TypeDefinitionRequest
    };
    (
        params "textDocument/typeDefinition" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::TypeDefinitionParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/typeDefinition"($($expr:expr),* $(,)?)) => {
        $crate::TypeDefinitionParams::new($($expr,)*)
    };
    ("workspace/workspaceFolders") => {
        $crate::WorkspaceFoldersRequest
    };
    (
        params "workspace/workspaceFolders" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        ()
    };
    (params "workspace/workspaceFolders"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("workspace/configuration") => {
        $crate::ConfigurationRequest
    };
    (
        params "workspace/configuration" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::ConfigurationParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/configuration"($($expr:expr),* $(,)?)) => {
        $crate::ConfigurationParams::new($($expr,)*)
    };
    ("textDocument/documentColor") => {
        $crate::DocumentColorRequest
    };
    (
        params "textDocument/documentColor" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DocumentColorParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/documentColor"($($expr:expr),* $(,)?)) => {
        $crate::DocumentColorParams::new($($expr,)*)
    };
    ("textDocument/colorPresentation") => {
        $crate::ColorPresentationRequest
    };
    (
        params "textDocument/colorPresentation" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::ColorPresentationParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/colorPresentation"($($expr:expr),* $(,)?)) => {
        $crate::ColorPresentationParams::new($($expr,)*)
    };
    ("textDocument/foldingRange") => {
        $crate::FoldingRangeRequest
    };
    (
        params "textDocument/foldingRange" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::FoldingRangeParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/foldingRange"($($expr:expr),* $(,)?)) => {
        $crate::FoldingRangeParams::new($($expr,)*)
    };
    ("workspace/foldingRange/refresh") => {
        $crate::FoldingRangeRefreshRequest
    };
    (
        params "workspace/foldingRange/refresh" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        ()
    };
    (params "workspace/foldingRange/refresh"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("textDocument/declaration") => {
        $crate::DeclarationRequest
    };
    (
        params "textDocument/declaration" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DeclarationParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/declaration"($($expr:expr),* $(,)?)) => {
        $crate::DeclarationParams::new($($expr,)*)
    };
    ("textDocument/selectionRange") => {
        $crate::SelectionRangeRequest
    };
    (
        params "textDocument/selectionRange" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::SelectionRangeParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/selectionRange"($($expr:expr),* $(,)?)) => {
        $crate::SelectionRangeParams::new($($expr,)*)
    };
    ("window/workDoneProgress/create") => {
        $crate::WorkDoneProgressCreateRequest
    };
    (
        params "window/workDoneProgress/create" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::WorkDoneProgressCreateParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "window/workDoneProgress/create"($($expr:expr),* $(,)?)) => {
        $crate::WorkDoneProgressCreateParams::new($($expr,)*)
    };
    ("textDocument/prepareCallHierarchy") => {
        $crate::CallHierarchyPrepareRequest
    };
    (
        params "textDocument/prepareCallHierarchy" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::CallHierarchyPrepareParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/prepareCallHierarchy"($($expr:expr),* $(,)?)) => {
        $crate::CallHierarchyPrepareParams::new($($expr,)*)
    };
    ("callHierarchy/incomingCalls") => {
        $crate::CallHierarchyIncomingCallsRequest
    };
    (
        params "callHierarchy/incomingCalls" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::CallHierarchyIncomingCallsParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "callHierarchy/incomingCalls"($($expr:expr),* $(,)?)) => {
        $crate::CallHierarchyIncomingCallsParams::new($($expr,)*)
    };
    ("callHierarchy/outgoingCalls") => {
        $crate::CallHierarchyOutgoingCallsRequest
    };
    (
        params "callHierarchy/outgoingCalls" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::CallHierarchyOutgoingCallsParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "callHierarchy/outgoingCalls"($($expr:expr),* $(,)?)) => {
        $crate::CallHierarchyOutgoingCallsParams::new($($expr,)*)
    };
    ("textDocument/semanticTokens/full") => {
        $crate::SemanticTokensRequest
    };
    (
        params "textDocument/semanticTokens/full" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::SemanticTokensParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/semanticTokens/full"($($expr:expr),* $(,)?)) => {
        $crate::SemanticTokensParams::new($($expr,)*)
    };
    ("textDocument/semanticTokens/full/delta") => {
        $crate::SemanticTokensDeltaRequest
    };
    (
        params "textDocument/semanticTokens/full/delta" { $($field:ident : $expr:expr),*
        $(,)? $(, .. $base:expr)? }
    ) => {
        $crate::SemanticTokensDeltaParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/semanticTokens/full/delta"($($expr:expr),* $(,)?)) => {
        $crate::SemanticTokensDeltaParams::new($($expr,)*)
    };
    ("textDocument/semanticTokens/range") => {
        $crate::SemanticTokensRangeRequest
    };
    (
        params "textDocument/semanticTokens/range" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::SemanticTokensRangeParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/semanticTokens/range"($($expr:expr),* $(,)?)) => {
        $crate::SemanticTokensRangeParams::new($($expr,)*)
    };
    ("workspace/semanticTokens/refresh") => {
        $crate::SemanticTokensRefreshRequest
    };
    (
        params "workspace/semanticTokens/refresh" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        ()
    };
    (params "workspace/semanticTokens/refresh"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("window/showDocument") => {
        $crate::ShowDocumentRequest
    };
    (
        params "window/showDocument" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::ShowDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "window/showDocument"($($expr:expr),* $(,)?)) => {
        $crate::ShowDocumentParams::new($($expr,)*)
    };
    ("textDocument/linkedEditingRange") => {
        $crate::LinkedEditingRangeRequest
    };
    (
        params "textDocument/linkedEditingRange" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::LinkedEditingRangeParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/linkedEditingRange"($($expr:expr),* $(,)?)) => {
        $crate::LinkedEditingRangeParams::new($($expr,)*)
    };
    ("workspace/willCreateFiles") => {
        $crate::WillCreateFilesRequest
    };
    (
        params "workspace/willCreateFiles" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CreateFilesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/willCreateFiles"($($expr:expr),* $(,)?)) => {
        $crate::CreateFilesParams::new($($expr,)*)
    };
    ("workspace/willRenameFiles") => {
        $crate::WillRenameFilesRequest
    };
    (
        params "workspace/willRenameFiles" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::RenameFilesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/willRenameFiles"($($expr:expr),* $(,)?)) => {
        $crate::RenameFilesParams::new($($expr,)*)
    };
    ("workspace/willDeleteFiles") => {
        $crate::WillDeleteFilesRequest
    };
    (
        params "workspace/willDeleteFiles" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DeleteFilesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/willDeleteFiles"($($expr:expr),* $(,)?)) => {
        $crate::DeleteFilesParams::new($($expr,)*)
    };
    ("textDocument/moniker") => {
        $crate::MonikerRequest
    };
    (
        params "textDocument/moniker" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::MonikerParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/moniker"($($expr:expr),* $(,)?)) => {
        $crate::MonikerParams::new($($expr,)*)
    };
    ("textDocument/prepareTypeHierarchy") => {
        $crate::TypeHierarchyPrepareRequest
    };
    (
        params "textDocument/prepareTypeHierarchy" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::TypeHierarchyPrepareParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/prepareTypeHierarchy"($($expr:expr),* $(,)?)) => {
        $crate::TypeHierarchyPrepareParams::new($($expr,)*)
    };
    ("typeHierarchy/supertypes") => {
        $crate::TypeHierarchySupertypesRequest
    };
    (
        params "typeHierarchy/supertypes" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::TypeHierarchySupertypesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "typeHierarchy/supertypes"($($expr:expr),* $(,)?)) => {
        $crate::TypeHierarchySupertypesParams::new($($expr,)*)
    };
    ("typeHierarchy/subtypes") => {
        $crate::TypeHierarchySubtypesRequest
    };
    (
        params "typeHierarchy/subtypes" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::TypeHierarchySubtypesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "typeHierarchy/subtypes"($($expr:expr),* $(,)?)) => {
        $crate::TypeHierarchySubtypesParams::new($($expr,)*)
    };
    ("textDocument/inlineValue") => {
        $crate::InlineValueRequest
    };
    (
        params "textDocument/inlineValue" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::InlineValueParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/inlineValue"($($expr:expr),* $(,)?)) => {
        $crate::InlineValueParams::new($($expr,)*)
    };
    ("workspace/inlineValue/refresh") => {
        $crate::InlineValueRefreshRequest
    };
    (
        params "workspace/inlineValue/refresh" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        ()
    };
    (params "workspace/inlineValue/refresh"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("textDocument/inlayHint") => {
        $crate::InlayHintRequest
    };
    (
        params "textDocument/inlayHint" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::InlayHintParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/inlayHint"($($expr:expr),* $(,)?)) => {
        $crate::InlayHintParams::new($($expr,)*)
    };
    ("inlayHint/resolve") => {
        $crate::InlayHintResolveRequest
    };
    (
        params "inlayHint/resolve" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::InlayHint { $($field : $expr,)* $(.. $base)? }
    };
    (params "inlayHint/resolve"($($expr:expr),* $(,)?)) => {
        $crate::InlayHint::new($($expr,)*)
    };
    ("workspace/inlayHint/refresh") => {
        $crate::InlayHintRefreshRequest
    };
    (
        params "workspace/inlayHint/refresh" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        ()
    };
    (params "workspace/inlayHint/refresh"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("textDocument/diagnostic") => {
        $crate::DocumentDiagnosticRequest
    };
    (
        params "textDocument/diagnostic" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DocumentDiagnosticParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/diagnostic"($($expr:expr),* $(,)?)) => {
        $crate::DocumentDiagnosticParams::new($($expr,)*)
    };
    ("workspace/diagnostic") => {
        $crate::WorkspaceDiagnosticRequest
    };
    (
        params "workspace/diagnostic" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::WorkspaceDiagnosticParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/diagnostic"($($expr:expr),* $(,)?)) => {
        $crate::WorkspaceDiagnosticParams::new($($expr,)*)
    };
    ("workspace/diagnostic/refresh") => {
        $crate::DiagnosticRefreshRequest
    };
    (
        params "workspace/diagnostic/refresh" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        ()
    };
    (params "workspace/diagnostic/refresh"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("textDocument/inlineCompletion") => {
        $crate::InlineCompletionRequest
    };
    (
        params "textDocument/inlineCompletion" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::InlineCompletionParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/inlineCompletion"($($expr:expr),* $(,)?)) => {
        $crate::InlineCompletionParams::new($($expr,)*)
    };
    ("workspace/textDocumentContent") => {
        $crate::TextDocumentContentRequest
    };
    (
        params "workspace/textDocumentContent" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::TextDocumentContentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/textDocumentContent"($($expr:expr),* $(,)?)) => {
        $crate::TextDocumentContentParams::new($($expr,)*)
    };
    ("workspace/textDocumentContent/refresh") => {
        $crate::TextDocumentContentRefreshRequest
    };
    (
        params "workspace/textDocumentContent/refresh" { $($field:ident : $expr:expr),*
        $(,)? $(, .. $base:expr)? }
    ) => {
        $crate::TextDocumentContentRefreshParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/textDocumentContent/refresh"($($expr:expr),* $(,)?)) => {
        $crate::TextDocumentContentRefreshParams::new($($expr,)*)
    };
    ("client/registerCapability") => {
        $crate::RegistrationRequest
    };
    (
        params "client/registerCapability" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::RegistrationParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "client/registerCapability"($($expr:expr),* $(,)?)) => {
        $crate::RegistrationParams::new($($expr,)*)
    };
    ("client/unregisterCapability") => {
        $crate::UnregistrationRequest
    };
    (
        params "client/unregisterCapability" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::UnregistrationParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "client/unregisterCapability"($($expr:expr),* $(,)?)) => {
        $crate::UnregistrationParams::new($($expr,)*)
    };
    ("initialize") => {
        $crate::InitializeRequest
    };
    (
        params "initialize" { $($field:ident : $expr:expr),* $(,)? $(, .. $base:expr)? }
    ) => {
        $crate::InitializeParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "initialize"($($expr:expr),* $(,)?)) => {
        $crate::InitializeParams::new($($expr,)*)
    };
    ("shutdown") => {
        $crate::ShutdownRequest
    };
    (params "shutdown" { $($field:ident : $expr:expr),* $(,)? $(, .. $base:expr)? }) => {
        ()
    };
    (params "shutdown"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("window/showMessageRequest") => {
        $crate::ShowMessageRequest
    };
    (
        params "window/showMessageRequest" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::ShowMessageRequestParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "window/showMessageRequest"($($expr:expr),* $(,)?)) => {
        $crate::ShowMessageRequestParams::new($($expr,)*)
    };
    ("textDocument/willSaveWaitUntil") => {
        $crate::WillSaveTextDocumentWaitUntilRequest
    };
    (
        params "textDocument/willSaveWaitUntil" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::WillSaveTextDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/willSaveWaitUntil"($($expr:expr),* $(,)?)) => {
        $crate::WillSaveTextDocumentParams::new($($expr,)*)
    };
    ("textDocument/completion") => {
        $crate::CompletionRequest
    };
    (
        params "textDocument/completion" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CompletionParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/completion"($($expr:expr),* $(,)?)) => {
        $crate::CompletionParams::new($($expr,)*)
    };
    ("completionItem/resolve") => {
        $crate::CompletionResolveRequest
    };
    (
        params "completionItem/resolve" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CompletionItem { $($field : $expr,)* $(.. $base)? }
    };
    (params "completionItem/resolve"($($expr:expr),* $(,)?)) => {
        $crate::CompletionItem::new($($expr,)*)
    };
    ("textDocument/hover") => {
        $crate::HoverRequest
    };
    (
        params "textDocument/hover" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::HoverParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/hover"($($expr:expr),* $(,)?)) => {
        $crate::HoverParams::new($($expr,)*)
    };
    ("textDocument/signatureHelp") => {
        $crate::SignatureHelpRequest
    };
    (
        params "textDocument/signatureHelp" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::SignatureHelpParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/signatureHelp"($($expr:expr),* $(,)?)) => {
        $crate::SignatureHelpParams::new($($expr,)*)
    };
    ("textDocument/definition") => {
        $crate::DefinitionRequest
    };
    (
        params "textDocument/definition" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DefinitionParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/definition"($($expr:expr),* $(,)?)) => {
        $crate::DefinitionParams::new($($expr,)*)
    };
    ("textDocument/references") => {
        $crate::ReferencesRequest
    };
    (
        params "textDocument/references" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::ReferenceParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/references"($($expr:expr),* $(,)?)) => {
        $crate::ReferenceParams::new($($expr,)*)
    };
    ("textDocument/documentHighlight") => {
        $crate::DocumentHighlightRequest
    };
    (
        params "textDocument/documentHighlight" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::DocumentHighlightParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/documentHighlight"($($expr:expr),* $(,)?)) => {
        $crate::DocumentHighlightParams::new($($expr,)*)
    };
    ("textDocument/documentSymbol") => {
        $crate::DocumentSymbolRequest
    };
    (
        params "textDocument/documentSymbol" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::DocumentSymbolParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/documentSymbol"($($expr:expr),* $(,)?)) => {
        $crate::DocumentSymbolParams::new($($expr,)*)
    };
    ("textDocument/codeAction") => {
        $crate::CodeActionRequest
    };
    (
        params "textDocument/codeAction" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CodeActionParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/codeAction"($($expr:expr),* $(,)?)) => {
        $crate::CodeActionParams::new($($expr,)*)
    };
    ("codeAction/resolve") => {
        $crate::CodeActionResolveRequest
    };
    (
        params "codeAction/resolve" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CodeAction { $($field : $expr,)* $(.. $base)? }
    };
    (params "codeAction/resolve"($($expr:expr),* $(,)?)) => {
        $crate::CodeAction::new($($expr,)*)
    };
    ("workspace/symbol") => {
        $crate::WorkspaceSymbolRequest
    };
    (
        params "workspace/symbol" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::WorkspaceSymbolParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/symbol"($($expr:expr),* $(,)?)) => {
        $crate::WorkspaceSymbolParams::new($($expr,)*)
    };
    ("workspaceSymbol/resolve") => {
        $crate::WorkspaceSymbolResolveRequest
    };
    (
        params "workspaceSymbol/resolve" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::WorkspaceSymbol { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspaceSymbol/resolve"($($expr:expr),* $(,)?)) => {
        $crate::WorkspaceSymbol::new($($expr,)*)
    };
    ("textDocument/codeLens") => {
        $crate::CodeLensRequest
    };
    (
        params "textDocument/codeLens" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CodeLensParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/codeLens"($($expr:expr),* $(,)?)) => {
        $crate::CodeLensParams::new($($expr,)*)
    };
    ("codeLens/resolve") => {
        $crate::CodeLensResolveRequest
    };
    (
        params "codeLens/resolve" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CodeLens { $($field : $expr,)* $(.. $base)? }
    };
    (params "codeLens/resolve"($($expr:expr),* $(,)?)) => {
        $crate::CodeLens::new($($expr,)*)
    };
    ("workspace/codeLens/refresh") => {
        $crate::CodeLensRefreshRequest
    };
    (
        params "workspace/codeLens/refresh" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        ()
    };
    (params "workspace/codeLens/refresh"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("textDocument/documentLink") => {
        $crate::DocumentLinkRequest
    };
    (
        params "textDocument/documentLink" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DocumentLinkParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/documentLink"($($expr:expr),* $(,)?)) => {
        $crate::DocumentLinkParams::new($($expr,)*)
    };
    ("documentLink/resolve") => {
        $crate::DocumentLinkResolveRequest
    };
    (
        params "documentLink/resolve" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DocumentLink { $($field : $expr,)* $(.. $base)? }
    };
    (params "documentLink/resolve"($($expr:expr),* $(,)?)) => {
        $crate::DocumentLink::new($($expr,)*)
    };
    ("textDocument/formatting") => {
        $crate::DocumentFormattingRequest
    };
    (
        params "textDocument/formatting" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DocumentFormattingParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/formatting"($($expr:expr),* $(,)?)) => {
        $crate::DocumentFormattingParams::new($($expr,)*)
    };
    ("textDocument/rangeFormatting") => {
        $crate::DocumentRangeFormattingRequest
    };
    (
        params "textDocument/rangeFormatting" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::DocumentRangeFormattingParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/rangeFormatting"($($expr:expr),* $(,)?)) => {
        $crate::DocumentRangeFormattingParams::new($($expr,)*)
    };
    ("textDocument/rangesFormatting") => {
        $crate::DocumentRangesFormattingRequest
    };
    (
        params "textDocument/rangesFormatting" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::DocumentRangesFormattingParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/rangesFormatting"($($expr:expr),* $(,)?)) => {
        $crate::DocumentRangesFormattingParams::new($($expr,)*)
    };
    ("textDocument/onTypeFormatting") => {
        $crate::DocumentOnTypeFormattingRequest
    };
    (
        params "textDocument/onTypeFormatting" { $($field:ident : $expr:expr),* $(,)? $(,
        .. $base:expr)? }
    ) => {
        $crate::DocumentOnTypeFormattingParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/onTypeFormatting"($($expr:expr),* $(,)?)) => {
        $crate::DocumentOnTypeFormattingParams::new($($expr,)*)
    };
    ("textDocument/rename") => {
        $crate::RenameRequest
    };
    (
        params "textDocument/rename" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::RenameParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/rename"($($expr:expr),* $(,)?)) => {
        $crate::RenameParams::new($($expr,)*)
    };
    ("textDocument/prepareRename") => {
        $crate::PrepareRenameRequest
    };
    (
        params "textDocument/prepareRename" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::PrepareRenameParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/prepareRename"($($expr:expr),* $(,)?)) => {
        $crate::PrepareRenameParams::new($($expr,)*)
    };
    ("workspace/executeCommand") => {
        $crate::ExecuteCommandRequest
    };
    (
        params "workspace/executeCommand" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::ExecuteCommandParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/executeCommand"($($expr:expr),* $(,)?)) => {
        $crate::ExecuteCommandParams::new($($expr,)*)
    };
    ("workspace/applyEdit") => {
        $crate::ApplyWorkspaceEditRequest
    };
    (
        params "workspace/applyEdit" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::ApplyWorkspaceEditParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/applyEdit"($($expr:expr),* $(,)?)) => {
        $crate::ApplyWorkspaceEditParams::new($($expr,)*)
    };
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
    (
        params "workspace/didChangeWorkspaceFolders" { $($field:ident : $expr:expr),*
        $(,)? $(, .. $base:expr)? }
    ) => {
        $crate::DidChangeWorkspaceFoldersParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/didChangeWorkspaceFolders"($($expr:expr),* $(,)?)) => {
        $crate::DidChangeWorkspaceFoldersParams::new($($expr,)*)
    };
    ("window/workDoneProgress/cancel") => {
        $crate::WorkDoneProgressCancelNotification
    };
    (
        params "window/workDoneProgress/cancel" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::WorkDoneProgressCancelParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "window/workDoneProgress/cancel"($($expr:expr),* $(,)?)) => {
        $crate::WorkDoneProgressCancelParams::new($($expr,)*)
    };
    ("workspace/didCreateFiles") => {
        $crate::DidCreateFilesNotification
    };
    (
        params "workspace/didCreateFiles" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CreateFilesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/didCreateFiles"($($expr:expr),* $(,)?)) => {
        $crate::CreateFilesParams::new($($expr,)*)
    };
    ("workspace/didRenameFiles") => {
        $crate::DidRenameFilesNotification
    };
    (
        params "workspace/didRenameFiles" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::RenameFilesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/didRenameFiles"($($expr:expr),* $(,)?)) => {
        $crate::RenameFilesParams::new($($expr,)*)
    };
    ("workspace/didDeleteFiles") => {
        $crate::DidDeleteFilesNotification
    };
    (
        params "workspace/didDeleteFiles" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DeleteFilesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/didDeleteFiles"($($expr:expr),* $(,)?)) => {
        $crate::DeleteFilesParams::new($($expr,)*)
    };
    ("notebookDocument/didOpen") => {
        $crate::DidOpenNotebookDocumentNotification
    };
    (
        params "notebookDocument/didOpen" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DidOpenNotebookDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "notebookDocument/didOpen"($($expr:expr),* $(,)?)) => {
        $crate::DidOpenNotebookDocumentParams::new($($expr,)*)
    };
    ("notebookDocument/didChange") => {
        $crate::DidChangeNotebookDocumentNotification
    };
    (
        params "notebookDocument/didChange" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DidChangeNotebookDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "notebookDocument/didChange"($($expr:expr),* $(,)?)) => {
        $crate::DidChangeNotebookDocumentParams::new($($expr,)*)
    };
    ("notebookDocument/didSave") => {
        $crate::DidSaveNotebookDocumentNotification
    };
    (
        params "notebookDocument/didSave" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DidSaveNotebookDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "notebookDocument/didSave"($($expr:expr),* $(,)?)) => {
        $crate::DidSaveNotebookDocumentParams::new($($expr,)*)
    };
    ("notebookDocument/didClose") => {
        $crate::DidCloseNotebookDocumentNotification
    };
    (
        params "notebookDocument/didClose" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DidCloseNotebookDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "notebookDocument/didClose"($($expr:expr),* $(,)?)) => {
        $crate::DidCloseNotebookDocumentParams::new($($expr,)*)
    };
    ("initialized") => {
        $crate::InitializedNotification
    };
    (
        params "initialized" { $($field:ident : $expr:expr),* $(,)? $(, .. $base:expr)? }
    ) => {
        $crate::InitializedParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "initialized"($($expr:expr),* $(,)?)) => {
        $crate::InitializedParams::new($($expr,)*)
    };
    ("exit") => {
        $crate::ExitNotification
    };
    (params "exit" { $($field:ident : $expr:expr),* $(,)? $(, .. $base:expr)? }) => {
        ()
    };
    (params "exit"($($expr:expr),* $(,)?)) => {
        ()
    };
    ("workspace/didChangeConfiguration") => {
        $crate::DidChangeConfigurationNotification
    };
    (
        params "workspace/didChangeConfiguration" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::DidChangeConfigurationParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/didChangeConfiguration"($($expr:expr),* $(,)?)) => {
        $crate::DidChangeConfigurationParams::new($($expr,)*)
    };
    ("window/showMessage") => {
        $crate::ShowMessageNotification
    };
    (
        params "window/showMessage" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::ShowMessageParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "window/showMessage"($($expr:expr),* $(,)?)) => {
        $crate::ShowMessageParams::new($($expr,)*)
    };
    ("window/logMessage") => {
        $crate::LogMessageNotification
    };
    (
        params "window/logMessage" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::LogMessageParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "window/logMessage"($($expr:expr),* $(,)?)) => {
        $crate::LogMessageParams::new($($expr,)*)
    };
    ("telemetry/event") => {
        $crate::TelemetryEventNotification
    };
    (
        params "telemetry/event" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::LspAny { $($field : $expr,)* $(.. $base)? }
    };
    (params "telemetry/event"($($expr:expr),* $(,)?)) => {
        $crate::LspAny::new($($expr,)*)
    };
    ("textDocument/didOpen") => {
        $crate::DidOpenTextDocumentNotification
    };
    (
        params "textDocument/didOpen" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DidOpenTextDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/didOpen"($($expr:expr),* $(,)?)) => {
        $crate::DidOpenTextDocumentParams::new($($expr,)*)
    };
    ("textDocument/didChange") => {
        $crate::DidChangeTextDocumentNotification
    };
    (
        params "textDocument/didChange" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DidChangeTextDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/didChange"($($expr:expr),* $(,)?)) => {
        $crate::DidChangeTextDocumentParams::new($($expr,)*)
    };
    ("textDocument/didClose") => {
        $crate::DidCloseTextDocumentNotification
    };
    (
        params "textDocument/didClose" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DidCloseTextDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/didClose"($($expr:expr),* $(,)?)) => {
        $crate::DidCloseTextDocumentParams::new($($expr,)*)
    };
    ("textDocument/didSave") => {
        $crate::DidSaveTextDocumentNotification
    };
    (
        params "textDocument/didSave" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::DidSaveTextDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/didSave"($($expr:expr),* $(,)?)) => {
        $crate::DidSaveTextDocumentParams::new($($expr,)*)
    };
    ("textDocument/willSave") => {
        $crate::WillSaveTextDocumentNotification
    };
    (
        params "textDocument/willSave" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::WillSaveTextDocumentParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/willSave"($($expr:expr),* $(,)?)) => {
        $crate::WillSaveTextDocumentParams::new($($expr,)*)
    };
    ("workspace/didChangeWatchedFiles") => {
        $crate::DidChangeWatchedFilesNotification
    };
    (
        params "workspace/didChangeWatchedFiles" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::DidChangeWatchedFilesParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "workspace/didChangeWatchedFiles"($($expr:expr),* $(,)?)) => {
        $crate::DidChangeWatchedFilesParams::new($($expr,)*)
    };
    ("textDocument/publishDiagnostics") => {
        $crate::PublishDiagnosticsNotification
    };
    (
        params "textDocument/publishDiagnostics" { $($field:ident : $expr:expr),* $(,)?
        $(, .. $base:expr)? }
    ) => {
        $crate::PublishDiagnosticsParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "textDocument/publishDiagnostics"($($expr:expr),* $(,)?)) => {
        $crate::PublishDiagnosticsParams::new($($expr,)*)
    };
    ("$/setTrace") => {
        $crate::SetTraceNotification
    };
    (
        params "$/setTrace" { $($field:ident : $expr:expr),* $(,)? $(, .. $base:expr)? }
    ) => {
        $crate::SetTraceParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "$/setTrace"($($expr:expr),* $(,)?)) => {
        $crate::SetTraceParams::new($($expr,)*)
    };
    ("$/logTrace") => {
        $crate::LogTraceNotification
    };
    (
        params "$/logTrace" { $($field:ident : $expr:expr),* $(,)? $(, .. $base:expr)? }
    ) => {
        $crate::LogTraceParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "$/logTrace"($($expr:expr),* $(,)?)) => {
        $crate::LogTraceParams::new($($expr,)*)
    };
    ("$/cancelRequest") => {
        $crate::CancelNotification
    };
    (
        params "$/cancelRequest" { $($field:ident : $expr:expr),* $(,)? $(, ..
        $base:expr)? }
    ) => {
        $crate::CancelParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "$/cancelRequest"($($expr:expr),* $(,)?)) => {
        $crate::CancelParams::new($($expr,)*)
    };
    ("$/progress") => {
        $crate::ProgressNotification
    };
    (
        params "$/progress" { $($field:ident : $expr:expr),* $(,)? $(, .. $base:expr)? }
    ) => {
        $crate::ProgressParams { $($field : $expr,)* $(.. $base)? }
    };
    (params "$/progress"($($expr:expr),* $(,)?)) => {
        $crate::ProgressParams::new($($expr,)*)
    };
}
