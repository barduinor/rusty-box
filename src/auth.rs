//! Box API authentication
pub mod access_token;
pub mod auth_ccg;
pub mod auth_client;
pub mod auth_developer;
pub mod auth_errors;

use async_trait::async_trait;

use crate::http_client::Headers;

use self::auth_errors::AuthError;

/// Trait for authentication methods
#[async_trait]
pub trait Auth<'a> {
    async fn access_token(&mut self) -> Result<String, AuthError>;
    async fn to_json(&mut self) -> Result<String, AuthError>;
    fn base_api_url(&self) -> String;
    fn user_agent(&self) -> String;
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
