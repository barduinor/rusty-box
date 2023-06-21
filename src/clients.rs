pub mod base;

use crate::http_client::HttpError;
use thiserror::Error;
/// Possible errors returned from the http client.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),

    // Note that this type is boxed because its size might be very large in
    // comparison to the rest. For more information visit:
    // https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    #[error("http error: {0}")]
    Http(Box<HttpError>),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),

    #[error("cache file error: {0}")]
    CacheFile(String),
    //
    // #[error("model error: {0}")]
    // Model(#[from] model::ModelError),
}
pub type ClientResult<T> = Result<T, ClientError>;
