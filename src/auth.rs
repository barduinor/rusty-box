use std::fmt;

use async_trait::async_trait;

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

    #[error("Generic: {message}")]
    Generic { message: String },
}

#[async_trait]
pub trait Auth
where
    Self: Default + Clone + fmt::Debug,
{
    async fn access_token(&mut self) -> Result<String, AuthError>;
    fn to_json(&self) -> Result<String, AuthError>;
    fn base_api_url(&self) -> String;
}

// implement debug
// impl std::fmt::Debug for dyn Auth {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Auth").finish()
//     }
// }
