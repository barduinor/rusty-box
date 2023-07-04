//! Developer token authentication
use crate::config::Config;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

use super::{access_token::AccessToken, Auth, AuthError};

/// Developer token authentication. This is used for testing and development.
/// Has a life time of 1 hour, and has to be manually generated in the Box developer console.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DevAuth {
    config: Config,
    access_token: AccessToken,
    expires_by: DateTime<Utc>,
}

impl DevAuth {
    pub fn new(config: Config, developer_token: String) -> Self {
        let mut access_token = AccessToken::new();
        access_token.access_token = Some(developer_token);

        DevAuth {
            config,
            access_token,
            expires_by: Utc::now() + Duration::seconds(3600),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_by - Duration::seconds(60 * 5)
    }
}

#[async_trait]
impl<'a> Auth<'a> for DevAuth {
    async fn access_token(&mut self) -> Result<String, AuthError> {
        if self.is_expired() {
            Err(AuthError::Generic("Developer token has expired".to_owned()))
        } else {
            let access_token = match self.access_token.access_token.clone() {
                Some(token) => token,
                None => return Err(AuthError::Generic("Developer token is not set".to_owned())),
            };
            Ok(access_token)
        }
    }

    async fn to_json(&mut self) -> Result<String, AuthError> {
        self.access_token().await?;
        match serde_json::to_string(&self) {
            Ok(json) => Ok(json),
            Err(e) => Err(AuthError::Serde(e)),
        }
    }

    fn base_api_url(&self) -> String {
        self.config.base_api_url()
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
        let mut dev_token = DevAuth::new(Config::default(), "test".to_owned());
        let access_token = dev_token.access_token().await.unwrap_or_default();
        assert_eq!(access_token, "test");
        assert!(dev_token.expires_by <= Utc::now() + Duration::seconds(3600));
        assert!(!dev_token.is_expired());
    }
}
