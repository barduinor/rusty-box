use async_trait::async_trait;

use crate::http_client::{self, Headers};

pub mod auth_ccg;
pub mod auth_developer;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    /// The request couldn't be completed because there was an error when trying
    /// to do so
    #[error("request: {0}")]
    Client(#[from] reqwest::Error),

    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503. In some cases, the response may contain a
    /// custom message from Spotify with more information, which can be
    /// serialized into `rspotify_model::ApiError`.
    #[error("status code {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),

    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("Generic: {message}")]
    Generic { message: String },

    #[error("request: {0}")]
    RequestError(#[from] http_client::reqwest::ReqwestError),
}

#[async_trait]
pub trait Auth<'a> {
    async fn access_token(&mut self) -> Result<String, AuthError>;
    fn to_json(&self) -> Result<String, AuthError>;
    fn base_api_url(&self) -> String;
    fn user_agent(&self) -> String;
    // async fn auth_header(&mut self) -> Result<Headers, AuthError>;
}

impl dyn Auth<'_> {
    pub async fn auth_header(&mut self) -> Result<Headers, AuthError> {
        let mut header = Headers::new();

        let header_name = "Authorization".to_string();
        let header_value = format!("Bearer {}", self.access_token().await?);

        header.insert(header_name, header_value);

        Ok(header)
    }
}

// implement debug
impl std::fmt::Debug for dyn Auth<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Auth").finish()
    }
}
