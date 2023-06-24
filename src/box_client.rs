use crate::auth::{Auth, AuthError};
use crate::http_client::{Headers, HttpClient};
use thiserror::Error;

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
    Http(Box<AuthError>),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),

    #[error("cache file error: {0}")]
    CacheFile(String),
}

// The conversion has to be done manually because it's in a `Box<T>`
impl From<AuthError> for ClientError {
    fn from(err: AuthError) -> Self {
        Self::Http(Box::new(err))
    }
}

#[derive(Debug)]
pub struct BoxClient<'a> {
    pub auth: Box<dyn Auth<'a> + 'static>,
    pub http: HttpClient,
}

impl<'a> BoxClient<'a> {
    pub fn new(auth: Box<dyn Auth<'a> + 'static>) -> Self {
        Self {
            auth,
            http: HttpClient::default(),
        }
    }

    pub async fn headers(&mut self) -> Result<Headers, AuthError> {
        let mut headers = Headers::new();

        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), self.auth.user_agent());

        let auth_headers = self.auth.auth_header().await?;

        headers.extend(auth_headers);
        Ok(headers)
    }
}

// impl BaseClient for BoxClient<'a> {
//     fn base_api_url(&self) -> String {
//         self.auth.base_api_url()
//     }
// }

#[cfg(test)]
use crate::auth::auth_ccg::{CCGAuth, SubjectType};

#[tokio::test]
async fn test_create_client_dev() {
    dotenv::from_filename(".dev.env").ok();
    let config = crate::config::Config::new();
    let auth = crate::auth::auth_developer::DeveloperToken::new(
        config,
        std::env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN must be set"),
    );
    let mut client = BoxClient::new(Box::new(auth));
    let access_token = client.auth.access_token().await;
    println!("{:#?}", client);
    assert!(access_token.is_ok());
}

#[tokio::test]
async fn test_create_client_ccg() {
    dotenv::from_filename(".ccg.env").ok();
    let config = crate::config::Config::new();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let box_subject_type = SubjectType::Enterprise;
    let box_subject_id = std::env::var("BOX_ENTERPRISE_ID").expect("BOX_ENTERPRISE_ID must be set");

    let auth = CCGAuth::new(
        config,
        client_id,
        client_secret,
        box_subject_type,
        box_subject_id,
    );

    let mut client = BoxClient::new(Box::new(auth));
    let access_token = client.auth.access_token().await;
    // println!("{:#?}", client.auth);
    assert!(access_token.is_ok());
}
