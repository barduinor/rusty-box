//! Box client implementation
use crate::auth::{Auth, AuthError};
use crate::box_client_error::Error;
use crate::http_client::{Headers, HttpClient};

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
    pub async fn headers(&mut self) -> Result<Headers, Error<AuthError>> {
        let mut headers = Headers::new();

        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), self.auth.user_agent());

        let auth_headers = self.auth.auth_header().await;

        let auth_headers = match auth_headers {
            Ok(auth_headers) => auth_headers,
            Err(e) => return Err(Error::AuthError(e)),
        };

        headers.extend(auth_headers);
        Ok(headers)
    }
}

#[cfg(test)]
use crate::auth::auth_ccg::{CCGAuth, SubjectType};

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
