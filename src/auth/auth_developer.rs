use crate::{config::Config, rest_api::authorization::models::access_token::AccessToken};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

use super::{Auth, AuthError};

#[derive(Debug)]
pub enum DeveloperTokenError {
    Generic { message: String },
}

#[derive(Debug, Clone, Serialize, Default)]
// pub struct DeveloperToken<'a> {
pub struct DeveloperToken {
    config: Config,
    // access_token: String,
    // expires_in: u32,
    access_token: AccessToken,
    expires_by: DateTime<Utc>,
    // store_auth_callable: Option<&'a dyn Fn()>,
}

// impl<'a> DeveloperToken<'a> {
impl DeveloperToken {
    pub fn new(
        config: Config,
        developer_token: String,
        // store_auth_callable: Option<&'a dyn Fn()>,
    ) -> Self {
        let mut access_token = AccessToken::new();
        access_token.access_token = Some(developer_token);
        let dev_token = DeveloperToken {
            config: config,
            access_token: access_token,
            // expires_in: 3600,
            expires_by: Utc::now() + Duration::seconds(3600),
            // store_auth_callable: store_auth_callable,
        };
        dev_token
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_by - Duration::seconds(60 * 5)
    }
}

#[async_trait]
impl<'a> Auth<'a> for DeveloperToken {
    async fn access_token(&mut self) -> Result<String, AuthError> {
        if self.is_expired() {
            Err(AuthError::Generic {
                message: "Developer token has expired".to_owned(),
            })
        } else {
            let access_token = match self.access_token.access_token.clone() {
                Some(token) => token,
                None => {
                    return Err(AuthError::Generic {
                        message: "Developer token is not set".to_owned(),
                    })
                }
            };
            Ok(access_token)
        }
    }

    async fn to_json(&mut self) -> Result<String, AuthError> {
        self.access_token().await?;
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

    fn user_agent(&self) -> String {
        self.config.user_agent()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_dev_token_new() {
        let mut dev_token = DeveloperToken::new(Config::default(), "test".to_owned());
        let access_token = dev_token.access_token().await.unwrap_or_default();
        assert_eq!(access_token, "test");
        assert!(dev_token.expires_by <= Utc::now() + Duration::seconds(3600));
        assert!(dev_token.is_expired() == false);
    }
}
