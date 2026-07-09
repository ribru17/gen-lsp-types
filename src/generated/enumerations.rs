use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// A set of predefined token types. This set is not fixed
/// an clients can specify additional token types via the
/// corresponding client capabilities.
///
/// @since 3.16.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub enum SemanticTokenTypes {
    Namespace,
    /// Represents a generic type. Acts as a fallback for types which can't be mapped to
    /// a specific type like class or enum.
    Type,
    Class,
    Enum,
    Interface,
    Struct,
    TypeParameter,
    Parameter,
    Variable,
    Property,
    EnumMember,
    Event,
    Function,
    Method,
    Macro,
    Keyword,
    Modifier,
    Comment,
    String,
    Number,
    Regexp,
    Operator,
    /// @since 3.17.0
    Decorator,
    /// @since 3.18.0
    Label,
    /// A custom value.
    #[serde(untagged)]
    Custom(Cow<'static, str>),
}
impl From<SemanticTokenTypes> for String {
    fn from(e: SemanticTokenTypes) -> Self {
        match e {
            SemanticTokenTypes::Namespace => "namespace".to_string(),
            SemanticTokenTypes::Type => "type".to_string(),
            SemanticTokenTypes::Class => "class".to_string(),
            SemanticTokenTypes::Enum => "enum".to_string(),
            SemanticTokenTypes::Interface => "interface".to_string(),
            SemanticTokenTypes::Struct => "struct".to_string(),
            SemanticTokenTypes::TypeParameter => "typeParameter".to_string(),
            SemanticTokenTypes::Parameter => "parameter".to_string(),
            SemanticTokenTypes::Variable => "variable".to_string(),
            SemanticTokenTypes::Property => "property".to_string(),
            SemanticTokenTypes::EnumMember => "enumMember".to_string(),
            SemanticTokenTypes::Event => "event".to_string(),
            SemanticTokenTypes::Function => "function".to_string(),
            SemanticTokenTypes::Method => "method".to_string(),
            SemanticTokenTypes::Macro => "macro".to_string(),
            SemanticTokenTypes::Keyword => "keyword".to_string(),
            SemanticTokenTypes::Modifier => "modifier".to_string(),
            SemanticTokenTypes::Comment => "comment".to_string(),
            SemanticTokenTypes::String => "string".to_string(),
            SemanticTokenTypes::Number => "number".to_string(),
            SemanticTokenTypes::Regexp => "regexp".to_string(),
            SemanticTokenTypes::Operator => "operator".to_string(),
            SemanticTokenTypes::Decorator => "decorator".to_string(),
            SemanticTokenTypes::Label => "label".to_string(),
            SemanticTokenTypes::Custom(any) => any.into_owned(),
        }
    }
}
impl From<String> for SemanticTokenTypes {
    fn from(v: String) -> Self {
        match v.as_str() {
            "namespace" => Self::Namespace,
            "type" => Self::Type,
            "class" => Self::Class,
            "enum" => Self::Enum,
            "interface" => Self::Interface,
            "struct" => Self::Struct,
            "typeParameter" => Self::TypeParameter,
            "parameter" => Self::Parameter,
            "variable" => Self::Variable,
            "property" => Self::Property,
            "enumMember" => Self::EnumMember,
            "event" => Self::Event,
            "function" => Self::Function,
            "method" => Self::Method,
            "macro" => Self::Macro,
            "keyword" => Self::Keyword,
            "modifier" => Self::Modifier,
            "comment" => Self::Comment,
            "string" => Self::String,
            "number" => Self::Number,
            "regexp" => Self::Regexp,
            "operator" => Self::Operator,
            "decorator" => Self::Decorator,
            "label" => Self::Label,
            _ => Self::Custom(Cow::Owned(v)),
        }
    }
}
impl SemanticTokenTypes {
    /// Create a custom `SemanticTokenTypes` from a string literal.
    #[must_use]
    pub const fn new(s: &'static str) -> Self {
        Self::Custom(Cow::Borrowed(s))
    }
}
impl From<&'static str> for SemanticTokenTypes {
    fn from(s: &'static str) -> Self {
        match s {
            "namespace" => Self::Namespace,
            "type" => Self::Type,
            "class" => Self::Class,
            "enum" => Self::Enum,
            "interface" => Self::Interface,
            "struct" => Self::Struct,
            "typeParameter" => Self::TypeParameter,
            "parameter" => Self::Parameter,
            "variable" => Self::Variable,
            "property" => Self::Property,
            "enumMember" => Self::EnumMember,
            "event" => Self::Event,
            "function" => Self::Function,
            "method" => Self::Method,
            "macro" => Self::Macro,
            "keyword" => Self::Keyword,
            "modifier" => Self::Modifier,
            "comment" => Self::Comment,
            "string" => Self::String,
            "number" => Self::Number,
            "regexp" => Self::Regexp,
            "operator" => Self::Operator,
            "decorator" => Self::Decorator,
            "label" => Self::Label,
            _ => Self::Custom(Cow::Borrowed(s)),
        }
    }
}
impl fmt::Display for SemanticTokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.clone().into();
        write!(f, "{s}")
    }
}
impl SemanticTokenTypes {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Namespace => "namespace",
            Self::Type => "type",
            Self::Class => "class",
            Self::Enum => "enum",
            Self::Interface => "interface",
            Self::Struct => "struct",
            Self::TypeParameter => "typeParameter",
            Self::Parameter => "parameter",
            Self::Variable => "variable",
            Self::Property => "property",
            Self::EnumMember => "enumMember",
            Self::Event => "event",
            Self::Function => "function",
            Self::Method => "method",
            Self::Macro => "macro",
            Self::Keyword => "keyword",
            Self::Modifier => "modifier",
            Self::Comment => "comment",
            Self::String => "string",
            Self::Number => "number",
            Self::Regexp => "regexp",
            Self::Operator => "operator",
            Self::Decorator => "decorator",
            Self::Label => "label",
            Self::Custom(any) => any,
        }
    }
}

/// A set of predefined token modifiers. This set is not fixed
/// an clients can specify additional token types via the
/// corresponding client capabilities.
///
/// @since 3.16.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub enum SemanticTokenModifiers {
    Declaration,
    Definition,
    Readonly,
    Static,
    Deprecated,
    Abstract,
    Async,
    Modification,
    Documentation,
    DefaultLibrary,
    /// A custom value.
    #[serde(untagged)]
    Custom(Cow<'static, str>),
}
impl From<SemanticTokenModifiers> for String {
    fn from(e: SemanticTokenModifiers) -> Self {
        match e {
            SemanticTokenModifiers::Declaration => "declaration".to_string(),
            SemanticTokenModifiers::Definition => "definition".to_string(),
            SemanticTokenModifiers::Readonly => "readonly".to_string(),
            SemanticTokenModifiers::Static => "static".to_string(),
            SemanticTokenModifiers::Deprecated => "deprecated".to_string(),
            SemanticTokenModifiers::Abstract => "abstract".to_string(),
            SemanticTokenModifiers::Async => "async".to_string(),
            SemanticTokenModifiers::Modification => "modification".to_string(),
            SemanticTokenModifiers::Documentation => "documentation".to_string(),
            SemanticTokenModifiers::DefaultLibrary => "defaultLibrary".to_string(),
            SemanticTokenModifiers::Custom(any) => any.into_owned(),
        }
    }
}
impl From<String> for SemanticTokenModifiers {
    fn from(v: String) -> Self {
        match v.as_str() {
            "declaration" => Self::Declaration,
            "definition" => Self::Definition,
            "readonly" => Self::Readonly,
            "static" => Self::Static,
            "deprecated" => Self::Deprecated,
            "abstract" => Self::Abstract,
            "async" => Self::Async,
            "modification" => Self::Modification,
            "documentation" => Self::Documentation,
            "defaultLibrary" => Self::DefaultLibrary,
            _ => Self::Custom(Cow::Owned(v)),
        }
    }
}
impl SemanticTokenModifiers {
    /// Create a custom `SemanticTokenModifiers` from a string literal.
    #[must_use]
    pub const fn new(s: &'static str) -> Self {
        Self::Custom(Cow::Borrowed(s))
    }
}
impl From<&'static str> for SemanticTokenModifiers {
    fn from(s: &'static str) -> Self {
        match s {
            "declaration" => Self::Declaration,
            "definition" => Self::Definition,
            "readonly" => Self::Readonly,
            "static" => Self::Static,
            "deprecated" => Self::Deprecated,
            "abstract" => Self::Abstract,
            "async" => Self::Async,
            "modification" => Self::Modification,
            "documentation" => Self::Documentation,
            "defaultLibrary" => Self::DefaultLibrary,
            _ => Self::Custom(Cow::Borrowed(s)),
        }
    }
}
impl fmt::Display for SemanticTokenModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.clone().into();
        write!(f, "{s}")
    }
}
impl SemanticTokenModifiers {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Declaration => "declaration",
            Self::Definition => "definition",
            Self::Readonly => "readonly",
            Self::Static => "static",
            Self::Deprecated => "deprecated",
            Self::Abstract => "abstract",
            Self::Async => "async",
            Self::Modification => "modification",
            Self::Documentation => "documentation",
            Self::DefaultLibrary => "defaultLibrary",
            Self::Custom(any) => any,
        }
    }
}

/// The document diagnostic report kinds.
///
/// @since 3.17.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum DocumentDiagnosticReportKind {
    /// A diagnostic report with a full
    /// set of problems.
    Full,
    /// A report indicating that the last
    /// returned report is still accurate.
    Unchanged,
}
impl From<DocumentDiagnosticReportKind> for String {
    fn from(e: DocumentDiagnosticReportKind) -> Self {
        match e {
            DocumentDiagnosticReportKind::Full => "full".to_string(),
            DocumentDiagnosticReportKind::Unchanged => "unchanged".to_string(),
        }
    }
}
impl TryFrom<String> for DocumentDiagnosticReportKind {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "full" => Ok(Self::Full),
            "unchanged" => Ok(Self::Unchanged),
            _ => Err(format!("Invalid DocumentDiagnosticReportKind: {v}")),
        }
    }
}
impl fmt::Display for DocumentDiagnosticReportKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl DocumentDiagnosticReportKind {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Full => "full",
            Self::Unchanged => "unchanged",
        }
    }
}

/// Predefined error codes.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "i32", from = "i32")]
pub enum ErrorCodes {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    /// Error code indicating that a server received a notification or
    /// request before the server has received the `initialize` request.
    ServerNotInitialized,
    UnknownErrorCode,
    /// A custom value.
    #[serde(untagged)]
    Custom(i32),
}
impl From<ErrorCodes> for i32 {
    fn from(e: ErrorCodes) -> Self {
        match e {
            ErrorCodes::ParseError => -32700i32,
            ErrorCodes::InvalidRequest => -32600i32,
            ErrorCodes::MethodNotFound => -32601i32,
            ErrorCodes::InvalidParams => -32602i32,
            ErrorCodes::InternalError => -32603i32,
            ErrorCodes::ServerNotInitialized => -32002i32,
            ErrorCodes::UnknownErrorCode => -32001i32,
            ErrorCodes::Custom(any) => any,
        }
    }
}
impl From<i32> for ErrorCodes {
    fn from(v: i32) -> Self {
        match v {
            -32700i32 => Self::ParseError,
            -32600i32 => Self::InvalidRequest,
            -32601i32 => Self::MethodNotFound,
            -32602i32 => Self::InvalidParams,
            -32603i32 => Self::InternalError,
            -32002i32 => Self::ServerNotInitialized,
            -32001i32 => Self::UnknownErrorCode,
            _ => Self::Custom(v),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "i32", from = "i32")]
pub enum LspErrorCodes {
    /// A request failed but it was syntactically correct, e.g the
    /// method name was known and the parameters were valid. The error
    /// message should contain human readable information about why
    /// the request failed.
    ///
    /// @since 3.17.0
    RequestFailed,
    /// The server cancelled the request. This error code should
    /// only be used for requests that explicitly support being
    /// server cancellable.
    ///
    /// @since 3.17.0
    ServerCancelled,
    /// The server detected that the content of a document got
    /// modified outside normal conditions. A server should
    /// NOT send this error code if it detects a content change
    /// in it unprocessed messages. The result even computed
    /// on an older state might still be useful for the client.
    ///
    /// If a client decides that a result is not of any use anymore
    /// the client should cancel the request.
    ContentModified,
    /// The client has canceled a request and a server has detected
    /// the cancel.
    RequestCancelled,
    /// A custom value.
    #[serde(untagged)]
    Custom(i32),
}
impl From<LspErrorCodes> for i32 {
    fn from(e: LspErrorCodes) -> Self {
        match e {
            LspErrorCodes::RequestFailed => -32803i32,
            LspErrorCodes::ServerCancelled => -32802i32,
            LspErrorCodes::ContentModified => -32801i32,
            LspErrorCodes::RequestCancelled => -32800i32,
            LspErrorCodes::Custom(any) => any,
        }
    }
}
impl From<i32> for LspErrorCodes {
    fn from(v: i32) -> Self {
        match v {
            -32803i32 => Self::RequestFailed,
            -32802i32 => Self::ServerCancelled,
            -32801i32 => Self::ContentModified,
            -32800i32 => Self::RequestCancelled,
            _ => Self::Custom(v),
        }
    }
}

/// A set of predefined range kinds.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub enum FoldingRangeKind {
    /// Folding range for a comment
    Comment,
    /// Folding range for an import or include
    Imports,
    /// Folding range for a region (e.g. `#region`)
    Region,
    /// A custom value.
    #[serde(untagged)]
    Custom(Cow<'static, str>),
}
impl From<FoldingRangeKind> for String {
    fn from(e: FoldingRangeKind) -> Self {
        match e {
            FoldingRangeKind::Comment => "comment".to_string(),
            FoldingRangeKind::Imports => "imports".to_string(),
            FoldingRangeKind::Region => "region".to_string(),
            FoldingRangeKind::Custom(any) => any.into_owned(),
        }
    }
}
impl From<String> for FoldingRangeKind {
    fn from(v: String) -> Self {
        match v.as_str() {
            "comment" => Self::Comment,
            "imports" => Self::Imports,
            "region" => Self::Region,
            _ => Self::Custom(Cow::Owned(v)),
        }
    }
}
impl FoldingRangeKind {
    /// Create a custom `FoldingRangeKind` from a string literal.
    #[must_use]
    pub const fn new(s: &'static str) -> Self {
        Self::Custom(Cow::Borrowed(s))
    }
}
impl From<&'static str> for FoldingRangeKind {
    fn from(s: &'static str) -> Self {
        match s {
            "comment" => Self::Comment,
            "imports" => Self::Imports,
            "region" => Self::Region,
            _ => Self::Custom(Cow::Borrowed(s)),
        }
    }
}
impl fmt::Display for FoldingRangeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.clone().into();
        write!(f, "{s}")
    }
}
impl FoldingRangeKind {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Comment => "comment",
            Self::Imports => "imports",
            Self::Region => "region",
            Self::Custom(any) => any,
        }
    }
}

/// A symbol kind.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum SymbolKind {
    File,
    Module,
    Namespace,
    Package,
    Class,
    Method,
    Property,
    Field,
    Constructor,
    Enum,
    Interface,
    Function,
    Variable,
    Constant,
    String,
    Number,
    Boolean,
    Array,
    Object,
    Key,
    Null,
    EnumMember,
    Struct,
    Event,
    Operator,
    TypeParameter,
}
impl From<SymbolKind> for u32 {
    fn from(e: SymbolKind) -> Self {
        match e {
            SymbolKind::File => 1u32,
            SymbolKind::Module => 2u32,
            SymbolKind::Namespace => 3u32,
            SymbolKind::Package => 4u32,
            SymbolKind::Class => 5u32,
            SymbolKind::Method => 6u32,
            SymbolKind::Property => 7u32,
            SymbolKind::Field => 8u32,
            SymbolKind::Constructor => 9u32,
            SymbolKind::Enum => 10u32,
            SymbolKind::Interface => 11u32,
            SymbolKind::Function => 12u32,
            SymbolKind::Variable => 13u32,
            SymbolKind::Constant => 14u32,
            SymbolKind::String => 15u32,
            SymbolKind::Number => 16u32,
            SymbolKind::Boolean => 17u32,
            SymbolKind::Array => 18u32,
            SymbolKind::Object => 19u32,
            SymbolKind::Key => 20u32,
            SymbolKind::Null => 21u32,
            SymbolKind::EnumMember => 22u32,
            SymbolKind::Struct => 23u32,
            SymbolKind::Event => 24u32,
            SymbolKind::Operator => 25u32,
            SymbolKind::TypeParameter => 26u32,
        }
    }
}
impl TryFrom<u32> for SymbolKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::File),
            2u32 => Ok(Self::Module),
            3u32 => Ok(Self::Namespace),
            4u32 => Ok(Self::Package),
            5u32 => Ok(Self::Class),
            6u32 => Ok(Self::Method),
            7u32 => Ok(Self::Property),
            8u32 => Ok(Self::Field),
            9u32 => Ok(Self::Constructor),
            10u32 => Ok(Self::Enum),
            11u32 => Ok(Self::Interface),
            12u32 => Ok(Self::Function),
            13u32 => Ok(Self::Variable),
            14u32 => Ok(Self::Constant),
            15u32 => Ok(Self::String),
            16u32 => Ok(Self::Number),
            17u32 => Ok(Self::Boolean),
            18u32 => Ok(Self::Array),
            19u32 => Ok(Self::Object),
            20u32 => Ok(Self::Key),
            21u32 => Ok(Self::Null),
            22u32 => Ok(Self::EnumMember),
            23u32 => Ok(Self::Struct),
            24u32 => Ok(Self::Event),
            25u32 => Ok(Self::Operator),
            26u32 => Ok(Self::TypeParameter),
            _ => Err(format!("Invalid SymbolKind: {v}")),
        }
    }
}

/// Symbol tags are extra annotations that tweak the rendering of a symbol.
///
/// @since 3.16
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum SymbolTag {
    /// Render a symbol as obsolete, usually using a strike-out.
    Deprecated,
}
impl From<SymbolTag> for u32 {
    fn from(e: SymbolTag) -> Self {
        match e {
            SymbolTag::Deprecated => 1u32,
        }
    }
}
impl TryFrom<u32> for SymbolTag {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Deprecated),
            _ => Err(format!("Invalid SymbolTag: {v}")),
        }
    }
}

/// Moniker uniqueness level to define scope of the moniker.
///
/// @since 3.16.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum UniquenessLevel {
    /// The moniker is only unique inside a document
    Document,
    /// The moniker is unique inside a project for which a dump got created
    Project,
    /// The moniker is unique inside the group to which a project belongs
    Group,
    /// The moniker is unique inside the moniker scheme.
    Scheme,
    /// The moniker is globally unique
    Global,
}
impl From<UniquenessLevel> for String {
    fn from(e: UniquenessLevel) -> Self {
        match e {
            UniquenessLevel::Document => "document".to_string(),
            UniquenessLevel::Project => "project".to_string(),
            UniquenessLevel::Group => "group".to_string(),
            UniquenessLevel::Scheme => "scheme".to_string(),
            UniquenessLevel::Global => "global".to_string(),
        }
    }
}
impl TryFrom<String> for UniquenessLevel {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "document" => Ok(Self::Document),
            "project" => Ok(Self::Project),
            "group" => Ok(Self::Group),
            "scheme" => Ok(Self::Scheme),
            "global" => Ok(Self::Global),
            _ => Err(format!("Invalid UniquenessLevel: {v}")),
        }
    }
}
impl fmt::Display for UniquenessLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl UniquenessLevel {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Document => "document",
            Self::Project => "project",
            Self::Group => "group",
            Self::Scheme => "scheme",
            Self::Global => "global",
        }
    }
}

/// The moniker kind.
///
/// @since 3.16.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum MonikerKind {
    /// The moniker represent a symbol that is imported into a project
    Import,
    /// The moniker represents a symbol that is exported from a project
    Export,
    /// The moniker represents a symbol that is local to a project (e.g. a local
    /// variable of a function, a class not visible outside the project, ...)
    Local,
}
impl From<MonikerKind> for String {
    fn from(e: MonikerKind) -> Self {
        match e {
            MonikerKind::Import => "import".to_string(),
            MonikerKind::Export => "export".to_string(),
            MonikerKind::Local => "local".to_string(),
        }
    }
}
impl TryFrom<String> for MonikerKind {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "import" => Ok(Self::Import),
            "export" => Ok(Self::Export),
            "local" => Ok(Self::Local),
            _ => Err(format!("Invalid MonikerKind: {v}")),
        }
    }
}
impl fmt::Display for MonikerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl MonikerKind {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Import => "import",
            Self::Export => "export",
            Self::Local => "local",
        }
    }
}

/// Inlay hint kinds.
///
/// @since 3.17.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum InlayHintKind {
    /// An inlay hint that for a type annotation.
    Type,
    /// An inlay hint that is for a parameter.
    Parameter,
}
impl From<InlayHintKind> for u32 {
    fn from(e: InlayHintKind) -> Self {
        match e {
            InlayHintKind::Type => 1u32,
            InlayHintKind::Parameter => 2u32,
        }
    }
}
impl TryFrom<u32> for InlayHintKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Type),
            2u32 => Ok(Self::Parameter),
            _ => Err(format!("Invalid InlayHintKind: {v}")),
        }
    }
}

/// The message type
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum MessageType {
    /// An error message.
    Error,
    /// A warning message.
    Warning,
    /// An information message.
    Info,
    /// A log message.
    Log,
    /// A debug message.
    ///
    /// @since 3.18.0
    Debug,
}
impl From<MessageType> for u32 {
    fn from(e: MessageType) -> Self {
        match e {
            MessageType::Error => 1u32,
            MessageType::Warning => 2u32,
            MessageType::Info => 3u32,
            MessageType::Log => 4u32,
            MessageType::Debug => 5u32,
        }
    }
}
impl TryFrom<u32> for MessageType {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Error),
            2u32 => Ok(Self::Warning),
            3u32 => Ok(Self::Info),
            4u32 => Ok(Self::Log),
            5u32 => Ok(Self::Debug),
            _ => Err(format!("Invalid MessageType: {v}")),
        }
    }
}

/// Defines how the host (editor) should sync
/// document changes to the language server.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum TextDocumentSyncKind {
    /// Documents should not be synced at all.
    None,
    /// Documents are synced by always sending the full content
    /// of the document.
    Full,
    /// Documents are synced by sending the full content on open.
    /// After that only incremental updates to the document are
    /// send.
    Incremental,
}
impl From<TextDocumentSyncKind> for u32 {
    fn from(e: TextDocumentSyncKind) -> Self {
        match e {
            TextDocumentSyncKind::None => 0u32,
            TextDocumentSyncKind::Full => 1u32,
            TextDocumentSyncKind::Incremental => 2u32,
        }
    }
}
impl TryFrom<u32> for TextDocumentSyncKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            0u32 => Ok(Self::None),
            1u32 => Ok(Self::Full),
            2u32 => Ok(Self::Incremental),
            _ => Err(format!("Invalid TextDocumentSyncKind: {v}")),
        }
    }
}

/// Represents reasons why a text document is saved.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum TextDocumentSaveReason {
    /// Manually triggered, e.g. by the user pressing save, by starting debugging,
    /// or by an API call.
    Manual,
    /// Automatic after a delay.
    AfterDelay,
    /// When the editor lost focus.
    FocusOut,
}
impl From<TextDocumentSaveReason> for u32 {
    fn from(e: TextDocumentSaveReason) -> Self {
        match e {
            TextDocumentSaveReason::Manual => 1u32,
            TextDocumentSaveReason::AfterDelay => 2u32,
            TextDocumentSaveReason::FocusOut => 3u32,
        }
    }
}
impl TryFrom<u32> for TextDocumentSaveReason {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Manual),
            2u32 => Ok(Self::AfterDelay),
            3u32 => Ok(Self::FocusOut),
            _ => Err(format!("Invalid TextDocumentSaveReason: {v}")),
        }
    }
}

/// The kind of a completion entry.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum CompletionItemKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
}
impl From<CompletionItemKind> for u32 {
    fn from(e: CompletionItemKind) -> Self {
        match e {
            CompletionItemKind::Text => 1u32,
            CompletionItemKind::Method => 2u32,
            CompletionItemKind::Function => 3u32,
            CompletionItemKind::Constructor => 4u32,
            CompletionItemKind::Field => 5u32,
            CompletionItemKind::Variable => 6u32,
            CompletionItemKind::Class => 7u32,
            CompletionItemKind::Interface => 8u32,
            CompletionItemKind::Module => 9u32,
            CompletionItemKind::Property => 10u32,
            CompletionItemKind::Unit => 11u32,
            CompletionItemKind::Value => 12u32,
            CompletionItemKind::Enum => 13u32,
            CompletionItemKind::Keyword => 14u32,
            CompletionItemKind::Snippet => 15u32,
            CompletionItemKind::Color => 16u32,
            CompletionItemKind::File => 17u32,
            CompletionItemKind::Reference => 18u32,
            CompletionItemKind::Folder => 19u32,
            CompletionItemKind::EnumMember => 20u32,
            CompletionItemKind::Constant => 21u32,
            CompletionItemKind::Struct => 22u32,
            CompletionItemKind::Event => 23u32,
            CompletionItemKind::Operator => 24u32,
            CompletionItemKind::TypeParameter => 25u32,
        }
    }
}
impl TryFrom<u32> for CompletionItemKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Text),
            2u32 => Ok(Self::Method),
            3u32 => Ok(Self::Function),
            4u32 => Ok(Self::Constructor),
            5u32 => Ok(Self::Field),
            6u32 => Ok(Self::Variable),
            7u32 => Ok(Self::Class),
            8u32 => Ok(Self::Interface),
            9u32 => Ok(Self::Module),
            10u32 => Ok(Self::Property),
            11u32 => Ok(Self::Unit),
            12u32 => Ok(Self::Value),
            13u32 => Ok(Self::Enum),
            14u32 => Ok(Self::Keyword),
            15u32 => Ok(Self::Snippet),
            16u32 => Ok(Self::Color),
            17u32 => Ok(Self::File),
            18u32 => Ok(Self::Reference),
            19u32 => Ok(Self::Folder),
            20u32 => Ok(Self::EnumMember),
            21u32 => Ok(Self::Constant),
            22u32 => Ok(Self::Struct),
            23u32 => Ok(Self::Event),
            24u32 => Ok(Self::Operator),
            25u32 => Ok(Self::TypeParameter),
            _ => Err(format!("Invalid CompletionItemKind: {v}")),
        }
    }
}

/// Completion item tags are extra annotations that tweak the rendering of a completion
/// item.
///
/// @since 3.15.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum CompletionItemTag {
    /// Render a completion as obsolete, usually using a strike-out.
    Deprecated,
}
impl From<CompletionItemTag> for u32 {
    fn from(e: CompletionItemTag) -> Self {
        match e {
            CompletionItemTag::Deprecated => 1u32,
        }
    }
}
impl TryFrom<u32> for CompletionItemTag {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Deprecated),
            _ => Err(format!("Invalid CompletionItemTag: {v}")),
        }
    }
}

/// Defines whether the insert text in a completion item should be interpreted as
/// plain text or a snippet.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum InsertTextFormat {
    /// The primary text to be inserted is treated as a plain string.
    PlainText,
    /// The primary text to be inserted is treated as a snippet.
    ///
    /// A snippet can define tab stops and placeholders with `$1`, `$2`
    /// and `${3:foo}`. `$0` defines the final tab stop, it defaults to
    /// the end of the snippet. Placeholders with equal identifiers are linked,
    /// that is typing in one will update others too.
    ///
    /// See also: https://microsoft.github.io/language-server-protocol/specifications/specification-current/#snippet_syntax
    Snippet,
}
impl From<InsertTextFormat> for u32 {
    fn from(e: InsertTextFormat) -> Self {
        match e {
            InsertTextFormat::PlainText => 1u32,
            InsertTextFormat::Snippet => 2u32,
        }
    }
}
impl TryFrom<u32> for InsertTextFormat {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::PlainText),
            2u32 => Ok(Self::Snippet),
            _ => Err(format!("Invalid InsertTextFormat: {v}")),
        }
    }
}

/// How whitespace and indentation is handled during completion
/// item insertion.
///
/// @since 3.16.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum InsertTextMode {
    /// The insertion or replace strings is taken as it is. If the
    /// value is multi line the lines below the cursor will be
    /// inserted using the indentation defined in the string value.
    /// The client will not apply any kind of adjustments to the
    /// string.
    AsIs,
    /// The editor adjusts leading whitespace of new lines so that
    /// they match the indentation up to the cursor of the line for
    /// which the item is accepted.
    ///
    /// Consider a line like this: <2tabs><cursor><3tabs>foo. Accepting a
    /// multi line completion item is indented using 2 tabs and all
    /// following lines inserted will be indented using 2 tabs as well.
    AdjustIndentation,
}
impl From<InsertTextMode> for u32 {
    fn from(e: InsertTextMode) -> Self {
        match e {
            InsertTextMode::AsIs => 1u32,
            InsertTextMode::AdjustIndentation => 2u32,
        }
    }
}
impl TryFrom<u32> for InsertTextMode {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::AsIs),
            2u32 => Ok(Self::AdjustIndentation),
            _ => Err(format!("Invalid InsertTextMode: {v}")),
        }
    }
}

/// A document highlight kind.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum DocumentHighlightKind {
    /// A textual occurrence.
    Text,
    /// Read-access of a symbol, like reading a variable.
    Read,
    /// Write-access of a symbol, like writing to a variable.
    Write,
}
impl From<DocumentHighlightKind> for u32 {
    fn from(e: DocumentHighlightKind) -> Self {
        match e {
            DocumentHighlightKind::Text => 1u32,
            DocumentHighlightKind::Read => 2u32,
            DocumentHighlightKind::Write => 3u32,
        }
    }
}
impl TryFrom<u32> for DocumentHighlightKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Text),
            2u32 => Ok(Self::Read),
            3u32 => Ok(Self::Write),
            _ => Err(format!("Invalid DocumentHighlightKind: {v}")),
        }
    }
}

/// A set of predefined code action kinds
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub enum CodeActionKind {
    /// Empty kind.
    Empty,
    /// Base kind for quickfix actions: 'quickfix'
    QuickFix,
    /// Base kind for refactoring actions: 'refactor'
    Refactor,
    /// Base kind for refactoring extraction actions: 'refactor.extract'
    ///
    /// Example extract actions:
    ///
    /// - Extract method
    /// - Extract function
    /// - Extract variable
    /// - Extract interface from class
    /// - ...
    RefactorExtract,
    /// Base kind for refactoring inline actions: 'refactor.inline'
    ///
    /// Example inline actions:
    ///
    /// - Inline function
    /// - Inline variable
    /// - Inline constant
    /// - ...
    RefactorInline,
    /// Base kind for refactoring move actions: `refactor.move`
    ///
    /// Example move actions:
    ///
    /// - Move a function to a new file
    /// - Move a property between classes
    /// - Move method to base class
    /// - ...
    ///
    /// @since 3.18.0
    RefactorMove,
    /// Base kind for refactoring rewrite actions: 'refactor.rewrite'
    ///
    /// Example rewrite actions:
    ///
    /// - Convert JavaScript function to class
    /// - Add or remove parameter
    /// - Encapsulate field
    /// - Make method static
    /// - Move method to base class
    /// - ...
    RefactorRewrite,
    /// Base kind for source actions: `source`
    ///
    /// Source code actions apply to the entire file.
    Source,
    /// Base kind for an organize imports source action: `source.organizeImports`
    SourceOrganizeImports,
    /// Base kind for auto-fix source actions: `source.fixAll`.
    ///
    /// Fix all actions automatically fix errors that have a clear fix that do not require user input.
    /// They should not suppress errors or perform unsafe fixes such as generating new types or classes.
    ///
    /// @since 3.15.0
    SourceFixAll,
    /// Base kind for all code actions applying to the entire notebook's scope. CodeActionKinds using
    /// this should always begin with `notebook.`
    ///
    /// @since 3.18.0
    Notebook,
    /// A custom value.
    #[serde(untagged)]
    Custom(Cow<'static, str>),
}
impl From<CodeActionKind> for String {
    fn from(e: CodeActionKind) -> Self {
        match e {
            CodeActionKind::Empty => "".to_string(),
            CodeActionKind::QuickFix => "quickfix".to_string(),
            CodeActionKind::Refactor => "refactor".to_string(),
            CodeActionKind::RefactorExtract => "refactor.extract".to_string(),
            CodeActionKind::RefactorInline => "refactor.inline".to_string(),
            CodeActionKind::RefactorMove => "refactor.move".to_string(),
            CodeActionKind::RefactorRewrite => "refactor.rewrite".to_string(),
            CodeActionKind::Source => "source".to_string(),
            CodeActionKind::SourceOrganizeImports => "source.organizeImports".to_string(),
            CodeActionKind::SourceFixAll => "source.fixAll".to_string(),
            CodeActionKind::Notebook => "notebook".to_string(),
            CodeActionKind::Custom(any) => any.into_owned(),
        }
    }
}
impl From<String> for CodeActionKind {
    fn from(v: String) -> Self {
        match v.as_str() {
            "" => Self::Empty,
            "quickfix" => Self::QuickFix,
            "refactor" => Self::Refactor,
            "refactor.extract" => Self::RefactorExtract,
            "refactor.inline" => Self::RefactorInline,
            "refactor.move" => Self::RefactorMove,
            "refactor.rewrite" => Self::RefactorRewrite,
            "source" => Self::Source,
            "source.organizeImports" => Self::SourceOrganizeImports,
            "source.fixAll" => Self::SourceFixAll,
            "notebook" => Self::Notebook,
            _ => Self::Custom(Cow::Owned(v)),
        }
    }
}
impl CodeActionKind {
    /// Create a custom `CodeActionKind` from a string literal.
    #[must_use]
    pub const fn new(s: &'static str) -> Self {
        Self::Custom(Cow::Borrowed(s))
    }
}
impl From<&'static str> for CodeActionKind {
    fn from(s: &'static str) -> Self {
        match s {
            "" => Self::Empty,
            "quickfix" => Self::QuickFix,
            "refactor" => Self::Refactor,
            "refactor.extract" => Self::RefactorExtract,
            "refactor.inline" => Self::RefactorInline,
            "refactor.move" => Self::RefactorMove,
            "refactor.rewrite" => Self::RefactorRewrite,
            "source" => Self::Source,
            "source.organizeImports" => Self::SourceOrganizeImports,
            "source.fixAll" => Self::SourceFixAll,
            "notebook" => Self::Notebook,
            _ => Self::Custom(Cow::Borrowed(s)),
        }
    }
}
impl fmt::Display for CodeActionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.clone().into();
        write!(f, "{s}")
    }
}
impl CodeActionKind {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Empty => "",
            Self::QuickFix => "quickfix",
            Self::Refactor => "refactor",
            Self::RefactorExtract => "refactor.extract",
            Self::RefactorInline => "refactor.inline",
            Self::RefactorMove => "refactor.move",
            Self::RefactorRewrite => "refactor.rewrite",
            Self::Source => "source",
            Self::SourceOrganizeImports => "source.organizeImports",
            Self::SourceFixAll => "source.fixAll",
            Self::Notebook => "notebook",
            Self::Custom(any) => any,
        }
    }
}

/// Code action tags are extra annotations that tweak the behavior of a code action.
///
/// @since 3.18.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum CodeActionTag {
    /// Marks the code action as LLM-generated.
    LLMGenerated,
}
impl From<CodeActionTag> for u32 {
    fn from(e: CodeActionTag) -> Self {
        match e {
            CodeActionTag::LLMGenerated => 1u32,
        }
    }
}
impl TryFrom<u32> for CodeActionTag {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::LLMGenerated),
            _ => Err(format!("Invalid CodeActionTag: {v}")),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum TraceValue {
    /// Turn tracing off.
    Off,
    /// Trace messages only.
    Messages,
    /// Verbose message tracing.
    Verbose,
}
impl From<TraceValue> for String {
    fn from(e: TraceValue) -> Self {
        match e {
            TraceValue::Off => "off".to_string(),
            TraceValue::Messages => "messages".to_string(),
            TraceValue::Verbose => "verbose".to_string(),
        }
    }
}
impl TryFrom<String> for TraceValue {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "off" => Ok(Self::Off),
            "messages" => Ok(Self::Messages),
            "verbose" => Ok(Self::Verbose),
            _ => Err(format!("Invalid TraceValue: {v}")),
        }
    }
}
impl fmt::Display for TraceValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl TraceValue {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Off => "off",
            Self::Messages => "messages",
            Self::Verbose => "verbose",
        }
    }
}

/// Describes the content type that a client supports in various
/// result literals like `Hover`, `ParameterInfo` or `CompletionItem`.
///
/// Please note that `MarkupKinds` must not start with a `$`. This kinds
/// are reserved for internal usage.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum MarkupKind {
    /// Plain text is supported as a content format
    PlainText,
    /// Markdown is supported as a content format
    Markdown,
}
impl From<MarkupKind> for String {
    fn from(e: MarkupKind) -> Self {
        match e {
            MarkupKind::PlainText => "plaintext".to_string(),
            MarkupKind::Markdown => "markdown".to_string(),
        }
    }
}
impl TryFrom<String> for MarkupKind {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "plaintext" => Ok(Self::PlainText),
            "markdown" => Ok(Self::Markdown),
            _ => Err(format!("Invalid MarkupKind: {v}")),
        }
    }
}
impl fmt::Display for MarkupKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl MarkupKind {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::PlainText => "plaintext",
            Self::Markdown => "markdown",
        }
    }
}

/// Predefined Language kinds
/// @since 3.18.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub enum LanguageKind {
    ABAP,
    WindowsBat,
    BibTeX,
    Clojure,
    Coffeescript,
    C,
    CPP,
    CSharp,
    CSS,
    /// @since 3.18.0
    D,
    /// @since 3.18.0
    Delphi,
    Diff,
    Dart,
    Dockerfile,
    Elixir,
    Erlang,
    FSharp,
    GitCommit,
    GitRebase,
    Go,
    Groovy,
    Handlebars,
    Haskell,
    HTML,
    Ini,
    Java,
    JavaScript,
    JavaScriptReact,
    JSON,
    LaTeX,
    Less,
    Lua,
    Makefile,
    Markdown,
    ObjectiveC,
    ObjectiveCPP,
    /// @since 3.18.0
    Pascal,
    Perl,
    Perl6,
    PHP,
    Plaintext,
    Powershell,
    Pug,
    Python,
    R,
    Razor,
    Ruby,
    Rust,
    SCSS,
    SASS,
    Scala,
    ShaderLab,
    ShellScript,
    SQL,
    Swift,
    TypeScript,
    TypeScriptReact,
    TeX,
    VisualBasic,
    XML,
    XSL,
    YAML,
    /// A custom value.
    #[serde(untagged)]
    Custom(Cow<'static, str>),
}
impl From<LanguageKind> for String {
    fn from(e: LanguageKind) -> Self {
        match e {
            LanguageKind::ABAP => "abap".to_string(),
            LanguageKind::WindowsBat => "bat".to_string(),
            LanguageKind::BibTeX => "bibtex".to_string(),
            LanguageKind::Clojure => "clojure".to_string(),
            LanguageKind::Coffeescript => "coffeescript".to_string(),
            LanguageKind::C => "c".to_string(),
            LanguageKind::CPP => "cpp".to_string(),
            LanguageKind::CSharp => "csharp".to_string(),
            LanguageKind::CSS => "css".to_string(),
            LanguageKind::D => "d".to_string(),
            LanguageKind::Delphi => "pascal".to_string(),
            LanguageKind::Diff => "diff".to_string(),
            LanguageKind::Dart => "dart".to_string(),
            LanguageKind::Dockerfile => "dockerfile".to_string(),
            LanguageKind::Elixir => "elixir".to_string(),
            LanguageKind::Erlang => "erlang".to_string(),
            LanguageKind::FSharp => "fsharp".to_string(),
            LanguageKind::GitCommit => "git-commit".to_string(),
            LanguageKind::GitRebase => "git-rebase".to_string(),
            LanguageKind::Go => "go".to_string(),
            LanguageKind::Groovy => "groovy".to_string(),
            LanguageKind::Handlebars => "handlebars".to_string(),
            LanguageKind::Haskell => "haskell".to_string(),
            LanguageKind::HTML => "html".to_string(),
            LanguageKind::Ini => "ini".to_string(),
            LanguageKind::Java => "java".to_string(),
            LanguageKind::JavaScript => "javascript".to_string(),
            LanguageKind::JavaScriptReact => "javascriptreact".to_string(),
            LanguageKind::JSON => "json".to_string(),
            LanguageKind::LaTeX => "latex".to_string(),
            LanguageKind::Less => "less".to_string(),
            LanguageKind::Lua => "lua".to_string(),
            LanguageKind::Makefile => "makefile".to_string(),
            LanguageKind::Markdown => "markdown".to_string(),
            LanguageKind::ObjectiveC => "objective-c".to_string(),
            LanguageKind::ObjectiveCPP => "objective-cpp".to_string(),
            LanguageKind::Pascal => "pascal".to_string(),
            LanguageKind::Perl => "perl".to_string(),
            LanguageKind::Perl6 => "perl6".to_string(),
            LanguageKind::PHP => "php".to_string(),
            LanguageKind::Plaintext => "plaintext".to_string(),
            LanguageKind::Powershell => "powershell".to_string(),
            LanguageKind::Pug => "jade".to_string(),
            LanguageKind::Python => "python".to_string(),
            LanguageKind::R => "r".to_string(),
            LanguageKind::Razor => "razor".to_string(),
            LanguageKind::Ruby => "ruby".to_string(),
            LanguageKind::Rust => "rust".to_string(),
            LanguageKind::SCSS => "scss".to_string(),
            LanguageKind::SASS => "sass".to_string(),
            LanguageKind::Scala => "scala".to_string(),
            LanguageKind::ShaderLab => "shaderlab".to_string(),
            LanguageKind::ShellScript => "shellscript".to_string(),
            LanguageKind::SQL => "sql".to_string(),
            LanguageKind::Swift => "swift".to_string(),
            LanguageKind::TypeScript => "typescript".to_string(),
            LanguageKind::TypeScriptReact => "typescriptreact".to_string(),
            LanguageKind::TeX => "tex".to_string(),
            LanguageKind::VisualBasic => "vb".to_string(),
            LanguageKind::XML => "xml".to_string(),
            LanguageKind::XSL => "xsl".to_string(),
            LanguageKind::YAML => "yaml".to_string(),
            LanguageKind::Custom(any) => any.into_owned(),
        }
    }
}
impl From<String> for LanguageKind {
    fn from(v: String) -> Self {
        match v.as_str() {
            "abap" => Self::ABAP,
            "bat" => Self::WindowsBat,
            "bibtex" => Self::BibTeX,
            "clojure" => Self::Clojure,
            "coffeescript" => Self::Coffeescript,
            "c" => Self::C,
            "cpp" => Self::CPP,
            "csharp" => Self::CSharp,
            "css" => Self::CSS,
            "d" => Self::D,
            "pascal" => Self::Delphi,
            "diff" => Self::Diff,
            "dart" => Self::Dart,
            "dockerfile" => Self::Dockerfile,
            "elixir" => Self::Elixir,
            "erlang" => Self::Erlang,
            "fsharp" => Self::FSharp,
            "git-commit" => Self::GitCommit,
            "git-rebase" => Self::GitRebase,
            "go" => Self::Go,
            "groovy" => Self::Groovy,
            "handlebars" => Self::Handlebars,
            "haskell" => Self::Haskell,
            "html" => Self::HTML,
            "ini" => Self::Ini,
            "java" => Self::Java,
            "javascript" => Self::JavaScript,
            "javascriptreact" => Self::JavaScriptReact,
            "json" => Self::JSON,
            "latex" => Self::LaTeX,
            "less" => Self::Less,
            "lua" => Self::Lua,
            "makefile" => Self::Makefile,
            "markdown" => Self::Markdown,
            "objective-c" => Self::ObjectiveC,
            "objective-cpp" => Self::ObjectiveCPP,
            "pascal" => Self::Pascal,
            "perl" => Self::Perl,
            "perl6" => Self::Perl6,
            "php" => Self::PHP,
            "plaintext" => Self::Plaintext,
            "powershell" => Self::Powershell,
            "jade" => Self::Pug,
            "python" => Self::Python,
            "r" => Self::R,
            "razor" => Self::Razor,
            "ruby" => Self::Ruby,
            "rust" => Self::Rust,
            "scss" => Self::SCSS,
            "sass" => Self::SASS,
            "scala" => Self::Scala,
            "shaderlab" => Self::ShaderLab,
            "shellscript" => Self::ShellScript,
            "sql" => Self::SQL,
            "swift" => Self::Swift,
            "typescript" => Self::TypeScript,
            "typescriptreact" => Self::TypeScriptReact,
            "tex" => Self::TeX,
            "vb" => Self::VisualBasic,
            "xml" => Self::XML,
            "xsl" => Self::XSL,
            "yaml" => Self::YAML,
            _ => Self::Custom(Cow::Owned(v)),
        }
    }
}
impl LanguageKind {
    /// Create a custom `LanguageKind` from a string literal.
    #[must_use]
    pub const fn new(s: &'static str) -> Self {
        Self::Custom(Cow::Borrowed(s))
    }
}
impl From<&'static str> for LanguageKind {
    fn from(s: &'static str) -> Self {
        match s {
            "abap" => Self::ABAP,
            "bat" => Self::WindowsBat,
            "bibtex" => Self::BibTeX,
            "clojure" => Self::Clojure,
            "coffeescript" => Self::Coffeescript,
            "c" => Self::C,
            "cpp" => Self::CPP,
            "csharp" => Self::CSharp,
            "css" => Self::CSS,
            "d" => Self::D,
            "pascal" => Self::Delphi,
            "diff" => Self::Diff,
            "dart" => Self::Dart,
            "dockerfile" => Self::Dockerfile,
            "elixir" => Self::Elixir,
            "erlang" => Self::Erlang,
            "fsharp" => Self::FSharp,
            "git-commit" => Self::GitCommit,
            "git-rebase" => Self::GitRebase,
            "go" => Self::Go,
            "groovy" => Self::Groovy,
            "handlebars" => Self::Handlebars,
            "haskell" => Self::Haskell,
            "html" => Self::HTML,
            "ini" => Self::Ini,
            "java" => Self::Java,
            "javascript" => Self::JavaScript,
            "javascriptreact" => Self::JavaScriptReact,
            "json" => Self::JSON,
            "latex" => Self::LaTeX,
            "less" => Self::Less,
            "lua" => Self::Lua,
            "makefile" => Self::Makefile,
            "markdown" => Self::Markdown,
            "objective-c" => Self::ObjectiveC,
            "objective-cpp" => Self::ObjectiveCPP,
            "pascal" => Self::Pascal,
            "perl" => Self::Perl,
            "perl6" => Self::Perl6,
            "php" => Self::PHP,
            "plaintext" => Self::Plaintext,
            "powershell" => Self::Powershell,
            "jade" => Self::Pug,
            "python" => Self::Python,
            "r" => Self::R,
            "razor" => Self::Razor,
            "ruby" => Self::Ruby,
            "rust" => Self::Rust,
            "scss" => Self::SCSS,
            "sass" => Self::SASS,
            "scala" => Self::Scala,
            "shaderlab" => Self::ShaderLab,
            "shellscript" => Self::ShellScript,
            "sql" => Self::SQL,
            "swift" => Self::Swift,
            "typescript" => Self::TypeScript,
            "typescriptreact" => Self::TypeScriptReact,
            "tex" => Self::TeX,
            "vb" => Self::VisualBasic,
            "xml" => Self::XML,
            "xsl" => Self::XSL,
            "yaml" => Self::YAML,
            _ => Self::Custom(Cow::Borrowed(s)),
        }
    }
}
impl fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.clone().into();
        write!(f, "{s}")
    }
}
impl LanguageKind {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ABAP => "abap",
            Self::WindowsBat => "bat",
            Self::BibTeX => "bibtex",
            Self::Clojure => "clojure",
            Self::Coffeescript => "coffeescript",
            Self::C => "c",
            Self::CPP => "cpp",
            Self::CSharp => "csharp",
            Self::CSS => "css",
            Self::D => "d",
            Self::Delphi => "pascal",
            Self::Diff => "diff",
            Self::Dart => "dart",
            Self::Dockerfile => "dockerfile",
            Self::Elixir => "elixir",
            Self::Erlang => "erlang",
            Self::FSharp => "fsharp",
            Self::GitCommit => "git-commit",
            Self::GitRebase => "git-rebase",
            Self::Go => "go",
            Self::Groovy => "groovy",
            Self::Handlebars => "handlebars",
            Self::Haskell => "haskell",
            Self::HTML => "html",
            Self::Ini => "ini",
            Self::Java => "java",
            Self::JavaScript => "javascript",
            Self::JavaScriptReact => "javascriptreact",
            Self::JSON => "json",
            Self::LaTeX => "latex",
            Self::Less => "less",
            Self::Lua => "lua",
            Self::Makefile => "makefile",
            Self::Markdown => "markdown",
            Self::ObjectiveC => "objective-c",
            Self::ObjectiveCPP => "objective-cpp",
            Self::Pascal => "pascal",
            Self::Perl => "perl",
            Self::Perl6 => "perl6",
            Self::PHP => "php",
            Self::Plaintext => "plaintext",
            Self::Powershell => "powershell",
            Self::Pug => "jade",
            Self::Python => "python",
            Self::R => "r",
            Self::Razor => "razor",
            Self::Ruby => "ruby",
            Self::Rust => "rust",
            Self::SCSS => "scss",
            Self::SASS => "sass",
            Self::Scala => "scala",
            Self::ShaderLab => "shaderlab",
            Self::ShellScript => "shellscript",
            Self::SQL => "sql",
            Self::Swift => "swift",
            Self::TypeScript => "typescript",
            Self::TypeScriptReact => "typescriptreact",
            Self::TeX => "tex",
            Self::VisualBasic => "vb",
            Self::XML => "xml",
            Self::XSL => "xsl",
            Self::YAML => "yaml",
            Self::Custom(any) => any,
        }
    }
}

/// Describes how an [inline completion provider][InlineCompletionItemProvider] was triggered.
///
/// @since 3.18.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum InlineCompletionTriggerKind {
    /// Completion was triggered explicitly by a user gesture.
    Invoked,
    /// Completion was triggered automatically while editing.
    Automatic,
}
impl From<InlineCompletionTriggerKind> for u32 {
    fn from(e: InlineCompletionTriggerKind) -> Self {
        match e {
            InlineCompletionTriggerKind::Invoked => 1u32,
            InlineCompletionTriggerKind::Automatic => 2u32,
        }
    }
}
impl TryFrom<u32> for InlineCompletionTriggerKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Invoked),
            2u32 => Ok(Self::Automatic),
            _ => Err(format!("Invalid InlineCompletionTriggerKind: {v}")),
        }
    }
}

/// A set of predefined position encoding kinds.
///
/// @since 3.17.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub enum PositionEncodingKind {
    /// Character offsets count UTF-8 code units (e.g. bytes).
    UTF8,
    /// Character offsets count UTF-16 code units.
    ///
    /// This is the default and must always be supported
    /// by servers
    UTF16,
    /// Character offsets count UTF-32 code units.
    ///
    /// Implementation note: these are the same as Unicode codepoints,
    /// so this `PositionEncodingKind` may also be used for an
    /// encoding-agnostic representation of character offsets.
    UTF32,
    /// A custom value.
    #[serde(untagged)]
    Custom(Cow<'static, str>),
}
impl From<PositionEncodingKind> for String {
    fn from(e: PositionEncodingKind) -> Self {
        match e {
            PositionEncodingKind::UTF8 => "utf-8".to_string(),
            PositionEncodingKind::UTF16 => "utf-16".to_string(),
            PositionEncodingKind::UTF32 => "utf-32".to_string(),
            PositionEncodingKind::Custom(any) => any.into_owned(),
        }
    }
}
impl From<String> for PositionEncodingKind {
    fn from(v: String) -> Self {
        match v.as_str() {
            "utf-8" => Self::UTF8,
            "utf-16" => Self::UTF16,
            "utf-32" => Self::UTF32,
            _ => Self::Custom(Cow::Owned(v)),
        }
    }
}
impl PositionEncodingKind {
    /// Create a custom `PositionEncodingKind` from a string literal.
    #[must_use]
    pub const fn new(s: &'static str) -> Self {
        Self::Custom(Cow::Borrowed(s))
    }
}
impl From<&'static str> for PositionEncodingKind {
    fn from(s: &'static str) -> Self {
        match s {
            "utf-8" => Self::UTF8,
            "utf-16" => Self::UTF16,
            "utf-32" => Self::UTF32,
            _ => Self::Custom(Cow::Borrowed(s)),
        }
    }
}
impl fmt::Display for PositionEncodingKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.clone().into();
        write!(f, "{s}")
    }
}
impl PositionEncodingKind {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::UTF8 => "utf-8",
            Self::UTF16 => "utf-16",
            Self::UTF32 => "utf-32",
            Self::Custom(any) => any,
        }
    }
}

/// The file event type
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum FileChangeType {
    /// The file got created.
    Created,
    /// The file got changed.
    Changed,
    /// The file got deleted.
    Deleted,
}
impl From<FileChangeType> for u32 {
    fn from(e: FileChangeType) -> Self {
        match e {
            FileChangeType::Created => 1u32,
            FileChangeType::Changed => 2u32,
            FileChangeType::Deleted => 3u32,
        }
    }
}
impl TryFrom<u32> for FileChangeType {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Created),
            2u32 => Ok(Self::Changed),
            3u32 => Ok(Self::Deleted),
            _ => Err(format!("Invalid FileChangeType: {v}")),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", from = "u32")]
pub enum WatchKind {
    /// Interested in create events.
    Create,
    /// Interested in change events
    Change,
    /// Interested in delete events
    Delete,
    /// A custom value.
    #[serde(untagged)]
    Custom(u32),
}
impl From<WatchKind> for u32 {
    fn from(e: WatchKind) -> Self {
        match e {
            WatchKind::Create => 1u32,
            WatchKind::Change => 2u32,
            WatchKind::Delete => 4u32,
            WatchKind::Custom(any) => any,
        }
    }
}
impl From<u32> for WatchKind {
    fn from(v: u32) -> Self {
        match v {
            1u32 => Self::Create,
            2u32 => Self::Change,
            4u32 => Self::Delete,
            _ => Self::Custom(v),
        }
    }
}

/// The diagnostic's severity.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum DiagnosticSeverity {
    /// Reports an error.
    Error,
    /// Reports a warning.
    Warning,
    /// Reports an information.
    Information,
    /// Reports a hint.
    Hint,
}
impl From<DiagnosticSeverity> for u32 {
    fn from(e: DiagnosticSeverity) -> Self {
        match e {
            DiagnosticSeverity::Error => 1u32,
            DiagnosticSeverity::Warning => 2u32,
            DiagnosticSeverity::Information => 3u32,
            DiagnosticSeverity::Hint => 4u32,
        }
    }
}
impl TryFrom<u32> for DiagnosticSeverity {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Error),
            2u32 => Ok(Self::Warning),
            3u32 => Ok(Self::Information),
            4u32 => Ok(Self::Hint),
            _ => Err(format!("Invalid DiagnosticSeverity: {v}")),
        }
    }
}

/// The diagnostic tags.
///
/// @since 3.15.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum DiagnosticTag {
    /// Unused or unnecessary code.
    ///
    /// Clients are allowed to render diagnostics with this tag faded out instead of having
    /// an error squiggle.
    Unnecessary,
    /// Deprecated or obsolete code.
    ///
    /// Clients are allowed to rendered diagnostics with this tag strike through.
    Deprecated,
}
impl From<DiagnosticTag> for u32 {
    fn from(e: DiagnosticTag) -> Self {
        match e {
            DiagnosticTag::Unnecessary => 1u32,
            DiagnosticTag::Deprecated => 2u32,
        }
    }
}
impl TryFrom<u32> for DiagnosticTag {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Unnecessary),
            2u32 => Ok(Self::Deprecated),
            _ => Err(format!("Invalid DiagnosticTag: {v}")),
        }
    }
}

/// How a completion was triggered
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum CompletionTriggerKind {
    /// Completion was triggered by typing an identifier (24x7 code
    /// complete), manual invocation (e.g Ctrl+Space) or via API.
    Invoked,
    /// Completion was triggered by a trigger character specified by
    /// the `triggerCharacters` properties of the `CompletionRegistrationOptions`.
    TriggerCharacter,
    /// Completion was re-triggered as current completion list is incomplete
    TriggerForIncompleteCompletions,
}
impl From<CompletionTriggerKind> for u32 {
    fn from(e: CompletionTriggerKind) -> Self {
        match e {
            CompletionTriggerKind::Invoked => 1u32,
            CompletionTriggerKind::TriggerCharacter => 2u32,
            CompletionTriggerKind::TriggerForIncompleteCompletions => 3u32,
        }
    }
}
impl TryFrom<u32> for CompletionTriggerKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Invoked),
            2u32 => Ok(Self::TriggerCharacter),
            3u32 => Ok(Self::TriggerForIncompleteCompletions),
            _ => Err(format!("Invalid CompletionTriggerKind: {v}")),
        }
    }
}

/// Defines how values from a set of defaults and an individual item will be
/// merged.
///
/// @since 3.18.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum ApplyKind {
    /// The value from the individual item (if provided and not `null`) will be
    /// used instead of the default.
    Replace,
    /// The value from the item will be merged with the default.
    ///
    /// The specific rules for mergeing values are defined against each field
    /// that supports merging.
    Merge,
}
impl From<ApplyKind> for u32 {
    fn from(e: ApplyKind) -> Self {
        match e {
            ApplyKind::Replace => 1u32,
            ApplyKind::Merge => 2u32,
        }
    }
}
impl TryFrom<u32> for ApplyKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Replace),
            2u32 => Ok(Self::Merge),
            _ => Err(format!("Invalid ApplyKind: {v}")),
        }
    }
}

/// How a signature help was triggered.
///
/// @since 3.15.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum SignatureHelpTriggerKind {
    /// Signature help was invoked manually by the user or by a command.
    Invoked,
    /// Signature help was triggered by a trigger character.
    TriggerCharacter,
    /// Signature help was triggered by the cursor moving or by the document content changing.
    ContentChange,
}
impl From<SignatureHelpTriggerKind> for u32 {
    fn from(e: SignatureHelpTriggerKind) -> Self {
        match e {
            SignatureHelpTriggerKind::Invoked => 1u32,
            SignatureHelpTriggerKind::TriggerCharacter => 2u32,
            SignatureHelpTriggerKind::ContentChange => 3u32,
        }
    }
}
impl TryFrom<u32> for SignatureHelpTriggerKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Invoked),
            2u32 => Ok(Self::TriggerCharacter),
            3u32 => Ok(Self::ContentChange),
            _ => Err(format!("Invalid SignatureHelpTriggerKind: {v}")),
        }
    }
}

/// The reason why code actions were requested.
///
/// @since 3.17.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum CodeActionTriggerKind {
    /// Code actions were explicitly requested by the user or by an extension.
    Invoked,
    /// Code actions were requested automatically.
    ///
    /// This typically happens when current selection in a file changes, but can
    /// also be triggered when file content changes.
    Automatic,
}
impl From<CodeActionTriggerKind> for u32 {
    fn from(e: CodeActionTriggerKind) -> Self {
        match e {
            CodeActionTriggerKind::Invoked => 1u32,
            CodeActionTriggerKind::Automatic => 2u32,
        }
    }
}
impl TryFrom<u32> for CodeActionTriggerKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Invoked),
            2u32 => Ok(Self::Automatic),
            _ => Err(format!("Invalid CodeActionTriggerKind: {v}")),
        }
    }
}

/// A pattern kind describing if a glob pattern matches a file a folder or
/// both.
///
/// @since 3.16.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum FileOperationPatternKind {
    /// The pattern matches a file only.
    File,
    /// The pattern matches a folder only.
    Folder,
}
impl From<FileOperationPatternKind> for String {
    fn from(e: FileOperationPatternKind) -> Self {
        match e {
            FileOperationPatternKind::File => "file".to_string(),
            FileOperationPatternKind::Folder => "folder".to_string(),
        }
    }
}
impl TryFrom<String> for FileOperationPatternKind {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "file" => Ok(Self::File),
            "folder" => Ok(Self::Folder),
            _ => Err(format!("Invalid FileOperationPatternKind: {v}")),
        }
    }
}
impl fmt::Display for FileOperationPatternKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl FileOperationPatternKind {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::File => "file",
            Self::Folder => "folder",
        }
    }
}

/// A notebook cell kind.
///
/// @since 3.17.0
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum NotebookCellKind {
    /// A markup-cell is formatted source that is used for display.
    Markup,
    /// A code-cell is source code.
    Code,
}
impl From<NotebookCellKind> for u32 {
    fn from(e: NotebookCellKind) -> Self {
        match e {
            NotebookCellKind::Markup => 1u32,
            NotebookCellKind::Code => 2u32,
        }
    }
}
impl TryFrom<u32> for NotebookCellKind {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Markup),
            2u32 => Ok(Self::Code),
            _ => Err(format!("Invalid NotebookCellKind: {v}")),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum ResourceOperationKind {
    /// Supports creating new files and folders.
    Create,
    /// Supports renaming existing files and folders.
    Rename,
    /// Supports deleting existing files and folders.
    Delete,
}
impl From<ResourceOperationKind> for String {
    fn from(e: ResourceOperationKind) -> Self {
        match e {
            ResourceOperationKind::Create => "create".to_string(),
            ResourceOperationKind::Rename => "rename".to_string(),
            ResourceOperationKind::Delete => "delete".to_string(),
        }
    }
}
impl TryFrom<String> for ResourceOperationKind {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "create" => Ok(Self::Create),
            "rename" => Ok(Self::Rename),
            "delete" => Ok(Self::Delete),
            _ => Err(format!("Invalid ResourceOperationKind: {v}")),
        }
    }
}
impl fmt::Display for ResourceOperationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl ResourceOperationKind {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Create => "create",
            Self::Rename => "rename",
            Self::Delete => "delete",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum FailureHandlingKind {
    /// Applying the workspace change is simply aborted if one of the changes provided
    /// fails. All operations executed before the failing operation stay executed.
    Abort,
    /// All operations are executed transactional. That means they either all
    /// succeed or no changes at all are applied to the workspace.
    Transactional,
    /// If the workspace edit contains only textual file changes they are executed transactional.
    /// If resource changes (create, rename or delete file) are part of the change the failure
    /// handling strategy is abort.
    TextOnlyTransactional,
    /// The client tries to undo the operations already executed. But there is no
    /// guarantee that this is succeeding.
    Undo,
}
impl From<FailureHandlingKind> for String {
    fn from(e: FailureHandlingKind) -> Self {
        match e {
            FailureHandlingKind::Abort => "abort".to_string(),
            FailureHandlingKind::Transactional => "transactional".to_string(),
            FailureHandlingKind::TextOnlyTransactional => {
                "textOnlyTransactional".to_string()
            }
            FailureHandlingKind::Undo => "undo".to_string(),
        }
    }
}
impl TryFrom<String> for FailureHandlingKind {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "abort" => Ok(Self::Abort),
            "transactional" => Ok(Self::Transactional),
            "textOnlyTransactional" => Ok(Self::TextOnlyTransactional),
            "undo" => Ok(Self::Undo),
            _ => Err(format!("Invalid FailureHandlingKind: {v}")),
        }
    }
}
impl fmt::Display for FailureHandlingKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl FailureHandlingKind {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Abort => "abort",
            Self::Transactional => "transactional",
            Self::TextOnlyTransactional => "textOnlyTransactional",
            Self::Undo => "undo",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "u32", try_from = "u32")]
pub enum PrepareSupportDefaultBehavior {
    /// The client's default behavior is to select the identifier
    /// according the to language's syntax rule.
    Identifier,
}
impl From<PrepareSupportDefaultBehavior> for u32 {
    fn from(e: PrepareSupportDefaultBehavior) -> Self {
        match e {
            PrepareSupportDefaultBehavior::Identifier => 1u32,
        }
    }
}
impl TryFrom<u32> for PrepareSupportDefaultBehavior {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, <Self as TryFrom<u32>>::Error> {
        match v {
            1u32 => Ok(Self::Identifier),
            _ => Err(format!("Invalid PrepareSupportDefaultBehavior: {v}")),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(into = "String", try_from = "String")]
pub enum TokenFormat {
    Relative,
}
impl From<TokenFormat> for String {
    fn from(e: TokenFormat) -> Self {
        match e {
            TokenFormat::Relative => "relative".to_string(),
        }
    }
}
impl TryFrom<String> for TokenFormat {
    type Error = String;
    fn try_from(v: String) -> Result<Self, <Self as TryFrom<String>>::Error> {
        match v.as_str() {
            "relative" => Ok(Self::Relative),
            _ => Err(format!("Invalid TokenFormat: {v}")),
        }
    }
}
impl fmt::Display for TokenFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}
impl TokenFormat {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Relative => "relative",
        }
    }
}

impl std::ops::BitOr for WatchKind {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        (Into::<u32>::into(self) | Into::<u32>::into(rhs)).into()
    }
}
impl std::ops::BitOrAssign for WatchKind {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = (Into::<u32>::into(*self) | Into::<u32>::into(rhs)).into();
    }
}
impl std::ops::BitAnd for WatchKind {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        (Into::<u32>::into(self) & Into::<u32>::into(rhs)).into()
    }
}
impl std::ops::BitAndAssign for WatchKind {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = (Into::<u32>::into(*self) & Into::<u32>::into(rhs)).into();
    }
}
impl std::ops::BitXor for WatchKind {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        (Into::<u32>::into(self) ^ Into::<u32>::into(rhs)).into()
    }
}
impl std::ops::BitXorAssign for WatchKind {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = (Into::<u32>::into(*self) ^ Into::<u32>::into(rhs)).into();
    }
}
