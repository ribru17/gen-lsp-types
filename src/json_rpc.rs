use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{ErrorCodes, Notification, Request};

fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum Version {
    #[serde(rename = "2.0")]
    TwoPointZero,
}

/// A unique ID used to correlate requests and responses together.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Id {
    /// Numeric ID.
    Number(i64),
    /// String ID.
    String(String),
    /// Null ID.
    ///
    /// The use of Null as a value for the id member in a Request object is discouraged, because
    /// this specification uses a value of Null for Responses with an unknown id. Also, because
    /// JSON-RPC 1.0 uses an id value of Null for Notifications this could cause confusion in
    /// handling.
    Null,
}

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for Id {
    fn from(value: i32) -> Self {
        Self::Number(value as i64)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<()> for Id {
    fn from((): ()) -> Self {
        Self::Null
    }
}

impl From<crate::Id> for Id {
    fn from(value: crate::Id) -> Self {
        match value {
            crate::Id::Int(int) => Self::Number(int as i64),
            crate::Id::String(string) => Self::String(string),
        }
    }
}

impl TryFrom<Id> for crate::Id {
    type Error = String;

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        match value {
            Id::String(string) => Ok(Self::String(string)),
            Id::Number(number) => Ok(Self::Int(
                i32::try_from(number).map_err(|e| format!("Request ID int too big: {e}"))?,
            )),
            Id::Null => Err(Self::Error::from("Id cannot be null")),
        }
    }
}

/// A JSON-RPC Request (or Notification) object.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct RequestObject {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    jsonrpc: Version,
    /// An identifier established by the Client that MUST contain a String, Number, or NULL value if
    /// included. If it is not included it is assumed to be a notification. The value SHOULD
    /// normally not be Null and Numbers SHOULD NOT contain fractional parts.
    #[serde(default, deserialize_with = "deserialize_some")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Id>,
    /// A String containing the name of the method to be invoked. Method names that begin with the
    /// word rpc followed by a period character (U+002E or ASCII 46) are reserved for rpc-internal
    /// methods and extensions and MUST NOT be used for anything else.
    #[serde(default)]
    method: String,
    /// A Structured value that holds the parameter values to be used during the invocation of the
    /// method. This member MAY be omitted.
    ///
    /// If present, parameters for the rpc call MUST be provided as a Structured value. Either
    /// by-position through an Array or by-name through an Object.
    #[serde(default, deserialize_with = "deserialize_some")]
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
}

impl RequestObject {
    /// Creates a JSON-RPC Request object from a server request.
    ///
    /// # Panics
    ///
    /// Will panic if `result` cannot be serialized (impossible unless the trait was implemented
    /// incorrectly).
    pub fn from_request<R>(id: Id, params: R::Params) -> Self
    where
        R: Request,
    {
        let params = serde_json::to_value(params).expect("Invalid request params");
        let params = match params {
            Value::Null => None,
            Value::Array(_) | Value::Object(_) => Some(params),
            _ => panic!("Parameters must be an object or array, if not omitted."),
        };
        Self {
            jsonrpc: Version::TwoPointZero,
            id: Some(id),
            params,
            method: R::METHOD.into(),
        }
    }

    /// Creates a JSON-RPC Request object from a server notification.
    ///
    /// # Panics
    ///
    /// Will panic if `result` cannot be serialized (impossible unless the trait was implemented
    /// incorrectly).
    pub fn from_notification<N>(params: N::Params) -> Self
    where
        N: Notification,
    {
        let params = serde_json::to_value(params).expect("Invalid request params");
        let params = match params {
            Value::Null => None,
            Value::Array(_) | Value::Object(_) => Some(params),
            _ => panic!("Parameters must be an object or array, if not omitted."),
        };
        Self {
            jsonrpc: Version::TwoPointZero,
            method: N::METHOD.to_string(),
            params,
            id: None,
        }
    }

    /// Returns the method to be invoked.
    #[must_use]
    pub fn method(&self) -> &str {
        self.method.as_ref()
    }

    /// Returns the id.
    #[must_use]
    pub const fn id(&self) -> Option<&Id> {
        self.id.as_ref()
    }

    /// Returns the params.
    #[must_use]
    pub const fn params(&self) -> Option<&Value> {
        self.params.as_ref()
    }

    /// Splits the request into the method name, request ID, and the parameters.
    #[must_use]
    pub fn into_parts(self) -> (String, Option<Id>, Option<Value>) {
        (self.method, self.id, self.params)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
enum Kind {
    /// This member is REQUIRED on success. This member MUST NOT exist if there was an error
    /// invoking the method. The value of this member is determined by the method invoked on the
    /// Server.
    Ok { result: Value },
    /// This member is REQUIRED on error. This member MUST NOT exist if there was no error triggered
    /// during invocation. The value for this member MUST be an Object as defined in section 5.1.
    Err { error: Error },
}

/// A JSON-RPC Error object.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Error {
    /// A Number that indicates the error type that occurred.
    pub code: ErrorCodes,
    /// A String providing a short description of the error. The message SHOULD be limited to a
    /// concise single sentence.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error. This
    /// may be omitted. The value of this member is defined by the Server (e.g. detailed error
    /// information, nested errors etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// A JSON-RPC Response object.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResponseObject {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    jsonrpc: Version,
    #[serde(flatten)]
    kind: Kind,
    /// This member is REQUIRED. It MUST be the same as the value of the id member in the
    /// Request Object. If there was an error in detecting the id in the Request object
    /// (e.g. Parse error/Invalid Request), it MUST be Null.
    id: Id,
}

impl ResponseObject {
    /// Creates a successful Response object from a result value.
    ///
    /// # Panics
    ///
    /// Will panic if `result` cannot be serialized (impossible unless the trait was implemented
    /// incorrectly).
    pub fn from_success<R>(id: Id, result: R::Result) -> Self
    where
        R: Request,
    {
        let result = serde_json::to_value(result).unwrap();
        Self {
            jsonrpc: Version::TwoPointZero,
            kind: Kind::Ok { result },
            id,
        }
    }

    /// Creates an error Response object from an error value.
    #[must_use]
    pub const fn from_error(id: Id, error: Error) -> Self {
        Self {
            jsonrpc: Version::TwoPointZero,
            kind: Kind::Err { error },
            id,
        }
    }

    /// Returns `true` if the Response object indicates success.
    #[must_use]
    pub const fn is_ok(&self) -> bool {
        matches!(self.kind, Kind::Ok { .. })
    }

    /// Returns `true` if the Response object indicates failure.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        !self.is_ok()
    }

    /// Returns the corresponding Response object ID.
    #[must_use]
    pub const fn id(&self) -> &Id {
        &self.id
    }

    /// Returns the `result` value, if present.
    #[must_use]
    pub const fn result(&self) -> Option<&Value> {
        match &self.kind {
            Kind::Ok { result } => Some(result),
            Kind::Err { .. } => None,
        }
    }

    /// Returns the `error` object, if present.
    #[must_use]
    pub const fn error(&self) -> Option<&Error> {
        match &self.kind {
            Kind::Err { error } => Some(error),
            Kind::Ok { .. } => None,
        }
    }
}
