//! HTTP client module.
//! This module contains the `BaseHttpClient` trait, which is implemented by the `reqwest` module.
pub mod box_api_error;
pub mod common;
pub mod reqwest;

pub use self::reqwest::{ReqwestClient as HttpClient, ReqwestError as HttpError};
pub use common::{BaseHttpClient, Form, Headers, Query};
