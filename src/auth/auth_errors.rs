use std::fmt;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthErrorResponse {
    /// The type of the error returned.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The type of the error returned.
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

impl AuthErrorResponse {
    /// An OAuth 2.0 error
    pub fn new() -> AuthErrorResponse {
        AuthErrorResponse {
            error: None,
            error_description: None,
        }
    }
}

impl fmt::Display for AuthErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AuthErrorMessage: {:?} ({:?})",
            self.error, self.error_description,
        )
    }
}

/// Box API Auth errors
// #[derive(Debug)]
pub enum AuthError {
    Network(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    Generic(String),
    ResponseError(AuthErrorResponse),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            AuthError::Network(e) => ("reqwest", e.to_string()),
            AuthError::Serde(e) => ("serde", e.to_string()),
            AuthError::Io(e) => ("IO", e.to_string()),
            AuthError::Generic(e) => ("Token", e.to_string()),
            AuthError::ResponseError(e) => ("API Error", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl fmt::Debug for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            AuthError::Network(e) => ("reqwest", e.to_string()),
            AuthError::Serde(e) => ("serde", e.to_string()),
            AuthError::Io(e) => ("IO", e.to_string()),
            AuthError::Generic(e) => ("Token", e.to_string()),
            AuthError::ResponseError(e) => ("API Error", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl From<reqwest::Error> for AuthError {
    fn from(e: reqwest::Error) -> Self {
        AuthError::Network(e)
    }
}

impl From<serde_json::Error> for AuthError {
    fn from(e: serde_json::Error) -> Self {
        AuthError::Serde(e)
    }
}

impl From<std::io::Error> for AuthError {
    fn from(e: std::io::Error) -> Self {
        AuthError::Io(e)
    }
}

impl From<String> for AuthError {
    fn from(e: String) -> Self {
        AuthError::Generic(e)
    }
}

impl From<AuthErrorResponse> for AuthError {
    fn from(e: AuthErrorResponse) -> Self {
        AuthError::ResponseError(e)
    }
}
