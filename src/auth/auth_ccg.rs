use super::{Auth, AuthError};
use crate::http_client::{BaseHttpClient, Form, HttpClient};
use crate::http_client::{Headers, HttpError};
use crate::{config::Config, rest_api::authorization::models::access_token::AccessToken};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

// #[derive(Debug, Clone, Serialize, PartialEq)]
// pub enum CCGError {
//     Generic { message: String },
// }
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum SubjectType {
    Enterprise,
    User,
}
impl SubjectType {
    fn value(&self) -> String {
        match self {
            Self::Enterprise => "enterprise".to_owned(),
            Self::User => "user".to_owned(),
        }
    }
}
impl Default for SubjectType {
    fn default() -> SubjectType {
        Self::Enterprise
    }
}

// TODO: CCG support both enterprise and user auth. Should there be a specific implementation for each one?
#[derive(Debug, Clone, Serialize, Default)]
pub struct CCGAuth {
    pub config: Config,
    client_id: String,
    client_secret: String,
    box_subject_type: SubjectType,
    box_subject_id: String,
    access_token: AccessToken,
    expires_by: DateTime<Utc>,
    #[serde(skip)]
    client: HttpClient,
}

impl CCGAuth {
    pub fn new(
        config: Config,
        client_id: String,
        client_secret: String,
        box_subject_type: SubjectType,
        box_subject_id: String,
    ) -> Self {
        CCGAuth {
            config: config,
            client_id: client_id,
            client_secret: client_secret,
            box_subject_type: box_subject_type,
            box_subject_id: box_subject_id,
            access_token: AccessToken::new(),
            expires_by: Utc::now(),
            client: HttpClient::default(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_by
    }

    async fn fetch_access_token(&mut self) -> Result<AccessToken, AuthError> {
        let url = &(self.config.oauth2_api_url.clone() + "/token");

        let headers = None; // TODO: Add headers to rquest

        let box_subject_type = self.box_subject_type.value();

        let mut payload = Form::new();
        payload.insert("grant_type", "client_credentials");
        payload.insert("client_id", &self.client_id);
        payload.insert("client_secret", &self.client_secret);
        payload.insert("box_subject_type", &box_subject_type);
        payload.insert("box_subject_id", &self.box_subject_id);

        let now = Utc::now();

        let response = self.client.post_form(url, headers, &payload).await;

        let data = match response {
            Ok(data) => data,
            Err(HttpError::Client(e)) => return Err(AuthError::Client(e)),
            Err(HttpError::StatusCode(e)) => return Err(AuthError::StatusCode(e)),
        };

        // let xx = serde_json::from_str::<AccessToken>(&data)?;

        let access_token = match serde_json::from_str::<AccessToken>(&data) {
            Ok(access_token) => access_token,
            Err(e) => {
                let error = format!("Error parsing access token: {:?}", e);
                return Err(AuthError::Generic { message: error });
            }
        };
        let expires_in = access_token.expires_in.unwrap_or_default();
        self.expires_by = now + Duration::seconds(expires_in);
        self.access_token = access_token.clone();
        Ok(access_token)
    }
}

#[async_trait]
impl<'a> Auth<'a> for CCGAuth {
    async fn access_token(&mut self) -> Result<String, AuthError> {
        if self.is_expired() {
            match self.fetch_access_token().await {
                Ok(access_token) => Ok(access_token.access_token.unwrap_or_default()),
                Err(e) => {
                    let error = format!("Error fetching access token: {:?}", e);
                    Err(AuthError::Generic { message: error })
                }
            }
        } else {
            let access_token = match self.access_token.access_token.clone() {
                Some(token) => token,
                None => {
                    return Err(AuthError::Generic {
                        message: "CCG token is not set".to_owned(),
                    })
                }
            };
            Ok(access_token)
        }
    }
    fn to_json(&self) -> Result<String, AuthError> {
        match serde_json::to_string(&self) {
            Ok(json) => Ok(json),
            Err(e) => Err(AuthError::Generic {
                message: e.to_string(),
            }),
        }
    }
    fn base_api_url(&self) -> String {
        self.config.base_api_url().clone()
    }

    async fn auth_header(&mut self) -> Result<Headers, AuthError> {
        let mut header = Headers::new();

        let header_name = "Authorization".to_string();
        let header_value = format!("Bearer {}", self.access_token().await?);

        header.insert(header_name, header_value);

        Ok(header)
    }

    fn user_agent(&self) -> String {
        self.config.user_agent()
    }
}

#[cfg(test)]
use std::env;

#[tokio::test]
async fn test_ccg_new() {
    let config = Config::new();
    let client_id = "client_id".to_owned();
    let client_secret = "client_secret".to_owned();
    let box_subject_type = SubjectType::Enterprise;
    let box_subject_id = "box_subject_id".to_owned();
    let ccg_auth = CCGAuth::new(
        config,
        client_id,
        client_secret,
        box_subject_type,
        box_subject_id,
    );
    assert_eq!(ccg_auth.client_id, "client_id".to_owned());
    assert_eq!(ccg_auth.client_secret, "client_secret".to_owned());
    assert_eq!(ccg_auth.box_subject_type, SubjectType::Enterprise);
    assert_eq!(ccg_auth.box_subject_id, "box_subject_id".to_owned());
}

#[tokio::test]
async fn test_ccg_request() {
    dotenv::from_filename(".ccg.env").ok();
    let config = Config::new();
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let box_subject_type = SubjectType::Enterprise;
    let box_subject_id = env::var("BOX_ENTERPRISE_ID").expect("BOX_ENTERPRISE_ID must be set");

    let mut auth = CCGAuth::new(
        config,
        client_id,
        client_secret,
        box_subject_type,
        box_subject_id,
    );

    let access_token = auth.access_token().await;
    // println!("access_token: {:#?}", access_token);

    assert_eq!(access_token.is_ok(), true);
    assert_eq!(auth.is_expired(), false);
    assert_eq!(auth.access_token.access_token.is_some(), true);
    assert_eq!(
        access_token.unwrap(),
        auth.access_token.access_token.unwrap()
    );
}
