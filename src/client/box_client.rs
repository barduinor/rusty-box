//! Box client implementation

/// Box client implementation
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

    // TODO: ERROR HANDLING
    pub async fn headers(&mut self) -> Result<Headers, BoxAPIError> {
        let mut headers = Headers::new();

        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), self.auth.user_agent());

        let auth_headers = self.auth.auth_header().await?;

        headers.extend(auth_headers);
        Ok(headers)
    }
}

#[cfg(test)]
use crate::auth::auth_ccg::{CCGAuth, SubjectType};
use crate::auth::Auth;

use super::{
    client_error::BoxAPIError,
    http_client::{Headers, HttpClient},
};

#[tokio::test]
async fn test_create_client_ccg() {
    dotenv::from_filename(".ccg.env").ok();
    let config = crate::config::Config::new();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let env_subject_type = std::env::var("BOX_SUBJECT_TYPE").expect("BOX_SUBJECT_TYPE must be set");
    let box_subject_type = match env_subject_type.as_str() {
        "user" => SubjectType::User,
        "enterprise" => SubjectType::Enterprise,
        _ => panic!("BOX_SUBJECT_TYPE must be either 'user' or 'enterprise'"),
    };

    let box_subject_id = std::env::var("BOX_SUBJECT_ID").expect("BOX_SUBJECT_ID must be set");

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
