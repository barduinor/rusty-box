use crate::{config::Config, rest_api::authorization::models::access_token::AccessToken};
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

use super::Auth;
use crate::clients::{BaseHttpClient, Form, HttpClient};

#[derive(Debug)]
pub enum CCGAuthError {
    Generic { message: String },
}
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
#[derive(Debug, Clone, Serialize)]
pub struct CCGAuth {
    config: Config,
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

    async fn fetch_access_token(&mut self) -> Result<AccessToken, CCGAuthError> {
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
        let access_token = match response {
            Ok(response) => {
                let access_token = match serde_json::from_str(&response) {
                    Ok(access_token) => access_token,
                    Err(e) => {
                        return Err(CCGAuthError::Generic {
                            message: e.to_string(),
                        })
                    }
                };
                self.access_token = access_token;
                self.expires_by =
                    now + Duration::seconds(self.access_token.expires_in.unwrap_or_default());
                Ok(self.access_token.clone())
            }
            Err(e) => Err(CCGAuthError::Generic {
                message: e.to_string(),
            }),
        };
        access_token
    }
}

impl Auth for CCGAuth {
    type Error = CCGAuthError;

    fn access_token(&self) -> Result<String, Self::Error> {
        if self.is_expired() {
            todo!("CCG token has expired or is new, try to get a new token first");
            // Err(CCGError::Generic {
            //     message: "CCG token has expired".to_owned(),
            // })
        } else {
            let access_token = match self.access_token.access_token.clone() {
                Some(token) => token,
                None => {
                    return Err(CCGAuthError::Generic {
                        message: "CCG token is not set".to_owned(),
                    })
                }
            };
            Ok(access_token)
        }
    }
    fn to_json(&self) -> Result<String, Self::Error> {
        match serde_json::to_string(&self) {
            Ok(json) => Ok(json),
            Err(e) => Err(CCGAuthError::Generic {
                message: e.to_string(),
            }),
        }
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
    let mut ccg_auth = CCGAuth::new(
        config,
        client_id,
        client_secret,
        box_subject_type,
        box_subject_id,
    );
    ccg_auth.fetch_access_token().await.unwrap();
    let access_token = ccg_auth.access_token();

    assert_eq!(access_token.is_ok(), true);
    assert_eq!(ccg_auth.is_expired(), false);
    assert_eq!(ccg_auth.access_token.access_token.is_some(), true);
    assert_eq!(
        access_token.unwrap(),
        ccg_auth.access_token.access_token.unwrap()
    );
}
