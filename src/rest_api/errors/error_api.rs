use std::fmt;

use crate::auth::auth_errors::AuthError;

use super::model::client_error::BoxAPIErrorResponse;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

// #[derive(Debug)]
pub enum BoxAPIError {
    Network(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(BoxAPIErrorResponse),
    AuthError(AuthError),
}

impl fmt::Display for BoxAPIErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "APIError {{ object_type: {:?}, status: {:?}, code: {:?}, message: {:?}, context_info: {:?}, help_url: {:?}, request_id: {:?} }}", self.object_type, self.status, self.code, self.message, self.context_info, self.help_url, self.request_id)
    }
}

impl fmt::Display for BoxAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            BoxAPIError::Network(e) => ("reqwest", e.to_string()),
            BoxAPIError::Serde(e) => ("serde", e.to_string()),
            BoxAPIError::Io(e) => ("IO", e.to_string()),
            BoxAPIError::ResponseError(e) => ("Box API Error", e.to_string()),
            BoxAPIError::AuthError(e) => ("Box Auth Error", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}
impl fmt::Debug for BoxAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            BoxAPIError::Network(e) => ("reqwest", e.to_string()),
            BoxAPIError::Serde(e) => ("serde", e.to_string()),
            BoxAPIError::Io(e) => ("IO", e.to_string()),
            BoxAPIError::ResponseError(e) => ("Box API Error", e.to_string()),
            BoxAPIError::AuthError(e) => ("Box Auth Error", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl From<reqwest::Error> for BoxAPIError {
    fn from(e: reqwest::Error) -> Self {
        BoxAPIError::Network(e)
    }
}

impl From<serde_json::Error> for BoxAPIError {
    fn from(e: serde_json::Error) -> Self {
        BoxAPIError::Serde(e)
    }
}

impl From<std::io::Error> for BoxAPIError {
    fn from(e: std::io::Error) -> Self {
        BoxAPIError::Io(e)
    }
}
impl From<BoxAPIErrorResponse> for BoxAPIError {
    fn from(e: BoxAPIErrorResponse) -> Self {
        BoxAPIError::ResponseError(e)
    }
}

impl From<AuthError> for BoxAPIError {
    fn from(e: AuthError) -> Self {
        BoxAPIError::AuthError(e)
    }
}

// pub fn urlencode<T: AsRef<str>>(s: T) -> String {
//     ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
// }

// pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
//     if let serde_json::Value::Object(object) = value {
//         let mut params = vec![];

//         for (key, value) in object {
//             match value {
//                 serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
//                     &format!("{}[{}]", prefix, key),
//                     value,
//                 )),
//                 serde_json::Value::Array(array) => {
//                     for (i, value) in array.iter().enumerate() {
//                         params.append(&mut parse_deep_object(
//                             &format!("{}[{}][{}]", prefix, key, i),
//                             value,
//                         ));
//                     }
//                 }
//                 serde_json::Value::String(s) => {
//                     params.push((format!("{}[{}]", prefix, key), s.clone()))
//                 }
//                 _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
//             }
//         }

//         return params;
//     }

//     unimplemented!("Only objects are supported with style=deepObject")
// }
