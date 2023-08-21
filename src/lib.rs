#[macro_use]
extern crate serde_derive;

pub mod auth;
pub mod client;
pub mod config;
pub mod rest_api;

// Top-Lebel re-exports
pub use crate::client::box_client::BoxClient;
pub use crate::client::client_error::BoxAPIError;

pub use crate::auth::auth_errors::AuthError;

pub use crate::auth::auth_ccg::CCGAuth;
pub use crate::auth::auth_developer::DevAuth;
pub use crate::auth::auth_oauth::OAuth;

pub use crate::auth::auth_ccg::SubjectType;
pub use crate::config::Config;

pub use crate::rest_api::users;
pub use crate::rest_api::users::users_api;
