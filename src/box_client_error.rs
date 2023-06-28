use std::error;
use std::fmt;

use crate::auth::AuthError;
use crate::http_client::reqwest::ReqwestError;
use crate::http_client::HttpError;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
    AuthError(AuthError),
    ReqwestError(HttpError),
    // BoxAPIError(crate::http_client::box_api_error::BoxAPIError),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::AuthError(e) => ("auth", e.to_string()),
            Error::ReqwestError(e) => ("reqwest", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::AuthError(e) => e,
            Error::ReqwestError(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<AuthError> for Error<T> {
    fn from(e: AuthError) -> Self {
        Error::AuthError(e)
    }
}

impl<T> From<ReqwestError> for Error<T> {
    fn from(e: ReqwestError) -> Self {
        Error::ReqwestError(e)
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
