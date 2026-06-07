use std::collections::HashMap;
#[allow(clippy::wildcard_imports)]
use super::*;

/// Information about where a symbol is defined.
///
/// Provides additional metadata over normal [location][Location] definitions, including the range of
/// the defining symbol
pub type DefinitionLink = LocationLink;

/// LSP arrays.
/// @since 3.17.0
pub type LspArray = Vec<LspAny>;

/// The LSP any type.
/// Please note that strictly speaking a property with the value `undefined`
/// can't be converted into JSON preserving the property name. However for
/// convenience it is allowed and assumed that all these properties are
/// optional as well.
/// @since 3.17.0
pub type LspAny = serde_json::Value;

/// Information about where a symbol is declared.
///
/// Provides additional metadata over normal [location][Location] declarations, including the range of
/// the declaring symbol.
///
/// Servers should prefer returning `DeclarationLink` over `Declaration` if supported
/// by the client.
pub type DeclarationLink = LocationLink;

/// A document selector is the combination of one or many document filters.
///
/// @sample `let sel:DocumentSelector = [{ language: 'typescript' }, { language: 'json', pattern: '**∕tsconfig.json' }]`;
///
/// The use of a string as a document filter is deprecated @since 3.16.0.
pub type DocumentSelector = Vec<DocumentFilter>;

/// An identifier to refer to a change annotation stored with a workspace edit.
pub type ChangeAnnotationIdentifier = String;

/// LSP object definition.
/// @since 3.17.0
pub type LspObject = HashMap<String, LspAny>;

/// The glob pattern to watch relative to the base path. Glob patterns can have the following syntax:
/// - `*` to match zero or more characters in a path segment
/// - `?` to match on one character in a path segment
/// - `**` to match any number of path segments, including none
/// - `{}` to group conditions (e.g. `**/*.{ts,js}` matches all TypeScript and JavaScript files)
/// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
/// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
///
/// @since 3.17.0
pub type Pattern = String;

pub type RegularExpressionEngineKind = String;
