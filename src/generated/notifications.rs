#[allow(clippy::wildcard_imports)]
use super::*;

/// The `workspace/didChangeWorkspaceFolders` notification is sent from the client to the server when the workspace
/// folder configuration changes.
#[derive(Debug)]
pub enum DidChangeWorkspaceFoldersNotification {}
impl Notification for DidChangeWorkspaceFoldersNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WorkspaceDidChangeWorkspaceFolders;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidChangeWorkspaceFoldersParams;
}

/// The `window/workDoneProgress/cancel` notification is sent from  the client to the server to cancel a progress
/// initiated on the server side.
#[derive(Debug)]
pub enum WorkDoneProgressCancelNotification {}
impl Notification for WorkDoneProgressCancelNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WindowWorkDoneProgressCancel;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = WorkDoneProgressCancelParams;
}

/// The did create files notification is sent from the client to the server when
/// files were created from within the client.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum DidCreateFilesNotification {}
impl Notification for DidCreateFilesNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WorkspaceDidCreateFiles;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = CreateFilesParams;
}

/// The did rename files notification is sent from the client to the server when
/// files were renamed from within the client.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum DidRenameFilesNotification {}
impl Notification for DidRenameFilesNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WorkspaceDidRenameFiles;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = RenameFilesParams;
}

/// The will delete files request is sent from the client to the server before files are actually
/// deleted as long as the deletion is triggered from within the client.
///
/// @since 3.16.0
#[derive(Debug)]
pub enum DidDeleteFilesNotification {}
impl Notification for DidDeleteFilesNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WorkspaceDidDeleteFiles;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DeleteFilesParams;
}

/// A notification sent when a notebook opens.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum DidOpenNotebookDocumentNotification {}
impl Notification for DidOpenNotebookDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::NotebookDocumentDidOpen;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidOpenNotebookDocumentParams;
}

#[derive(Debug)]
pub enum DidChangeNotebookDocumentNotification {}
impl Notification for DidChangeNotebookDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::NotebookDocumentDidChange;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidChangeNotebookDocumentParams;
}

/// A notification sent when a notebook document is saved.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum DidSaveNotebookDocumentNotification {}
impl Notification for DidSaveNotebookDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::NotebookDocumentDidSave;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidSaveNotebookDocumentParams;
}

/// A notification sent when a notebook closes.
///
/// @since 3.17.0
#[derive(Debug)]
pub enum DidCloseNotebookDocumentNotification {}
impl Notification for DidCloseNotebookDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::NotebookDocumentDidClose;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidCloseNotebookDocumentParams;
}

/// The initialized notification is sent from the client to the
/// server after the client is fully initialized and the server
/// is allowed to send requests from the server to the client.
#[derive(Debug)]
pub enum InitializedNotification {}
impl Notification for InitializedNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::Initialized;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = InitializedParams;
}

/// The exit event is sent from the client to the server to
/// ask the server to exit its process.
#[derive(Debug)]
pub enum ExitNotification {}
impl Notification for ExitNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::Exit;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = ();
}

/// The configuration change notification is sent from the client to the server
/// when the client's configuration has changed. The notification contains
/// the changed configuration as defined by the language client.
#[derive(Debug)]
pub enum DidChangeConfigurationNotification {}
impl Notification for DidChangeConfigurationNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WorkspaceDidChangeConfiguration;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidChangeConfigurationParams;
}

/// The show message notification is sent from a server to a client to ask
/// the client to display a particular message in the user interface.
#[derive(Debug)]
pub enum ShowMessageNotification {}
impl Notification for ShowMessageNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WindowShowMessage;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = ShowMessageParams;
}

/// The log message notification is sent from the server to the client to ask
/// the client to log a particular message.
#[derive(Debug)]
pub enum LogMessageNotification {}
impl Notification for LogMessageNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WindowLogMessage;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = LogMessageParams;
}

/// The telemetry event notification is sent from the server to the client to ask
/// the client to log telemetry data.
#[derive(Debug)]
pub enum TelemetryEventNotification {}
impl Notification for TelemetryEventNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::TelemetryEvent;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = LspAny;
}

/// The document open notification is sent from the client to the server to signal
/// newly opened text documents. The document's truth is now managed by the client
/// and the server must not try to read the document's truth using the document's
/// uri. Open in this sense means it is managed by the client. It doesn't necessarily
/// mean that its content is presented in an editor. An open notification must not
/// be sent more than once without a corresponding close notification send before.
/// This means open and close notification must be balanced and the max open count
/// is one.
#[derive(Debug)]
pub enum DidOpenTextDocumentNotification {}
impl Notification for DidOpenTextDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::TextDocumentDidOpen;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidOpenTextDocumentParams;
}

/// The document change notification is sent from the client to the server to signal
/// changes to a text document.
#[derive(Debug)]
pub enum DidChangeTextDocumentNotification {}
impl Notification for DidChangeTextDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::TextDocumentDidChange;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidChangeTextDocumentParams;
}

/// The document close notification is sent from the client to the server when
/// the document got closed in the client. The document's truth now exists where
/// the document's uri points to (e.g. if the document's uri is a file uri the
/// truth now exists on disk). As with the open notification the close notification
/// is about managing the document's content. Receiving a close notification
/// doesn't mean that the document was open in an editor before. A close
/// notification requires a previous open notification to be sent.
#[derive(Debug)]
pub enum DidCloseTextDocumentNotification {}
impl Notification for DidCloseTextDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::TextDocumentDidClose;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidCloseTextDocumentParams;
}

/// The document save notification is sent from the client to the server when
/// the document got saved in the client.
#[derive(Debug)]
pub enum DidSaveTextDocumentNotification {}
impl Notification for DidSaveTextDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::TextDocumentDidSave;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidSaveTextDocumentParams;
}

/// A document will save notification is sent from the client to the server before
/// the document is actually saved.
#[derive(Debug)]
pub enum WillSaveTextDocumentNotification {}
impl Notification for WillSaveTextDocumentNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::TextDocumentWillSave;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = WillSaveTextDocumentParams;
}

/// The watched files notification is sent from the client to the server when
/// the client detects changes to file watched by the language client.
#[derive(Debug)]
pub enum DidChangeWatchedFilesNotification {}
impl Notification for DidChangeWatchedFilesNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::WorkspaceDidChangeWatchedFiles;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = DidChangeWatchedFilesParams;
}

/// Diagnostics notification are sent from the server to the client to signal
/// results of validation runs.
#[derive(Debug)]
pub enum PublishDiagnosticsNotification {}
impl Notification for PublishDiagnosticsNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::TextDocumentPublishDiagnostics;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = PublishDiagnosticsParams;
}

#[derive(Debug)]
pub enum SetTraceNotification {}
impl Notification for SetTraceNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::SetTrace;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    type Params = SetTraceParams;
}

#[derive(Debug)]
pub enum LogTraceNotification {}
impl Notification for LogTraceNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::LogTrace;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ServerToClient;
    type Params = LogTraceParams;
}

#[derive(Debug)]
pub enum CancelNotification {}
impl Notification for CancelNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::CancelRequest;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::Both;
    type Params = CancelParams;
}

#[derive(Debug)]
pub enum ProgressNotification {}
impl Notification for ProgressNotification {
    const METHOD: LspNotificationMethod<'static> = LspNotificationMethod::Progress;
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::Both;
    type Params = ProgressParams;
}
