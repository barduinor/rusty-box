use crate::config::Config;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

use super::Auth;

#[derive(Debug)]
pub enum DeveloperTokenError {
    Generic { message: String },
}

#[derive(Debug, Clone, Serialize)]
// pub struct DeveloperToken<'a> {
pub struct DeveloperToken {
    config: Config,
    access_token: String,
    expires_in: u32,
    expires_by: DateTime<Utc>,
    // store_auth_callable: Option<&'a dyn Fn()>,
}

// impl<'a> DeveloperToken<'a> {
impl DeveloperToken {
    pub fn new(
        config: Config,
        access_token: String,
        // store_auth_callable: Option<&'a dyn Fn()>,
    ) -> Self {
        let mut dev_token = DeveloperToken {
            config: config,
            access_token: access_token,
            expires_in: 3600,
            expires_by: Utc::now(),
            // store_auth_callable: store_auth_callable,
        };
        dev_token.expires_by = Utc::now() + Duration::seconds(dev_token.expires_in as i64);
        // dev_token.store_auth();
        dev_token
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_by
    }
}

#[async_trait]
// impl<'a> Auth for DeveloperToken<'a> {
impl Auth for DeveloperToken {
    type Error = DeveloperTokenError;

    async fn access_token(&mut self) -> Result<String, Self::Error> {
        if self.is_expired() {
            Err(DeveloperTokenError::Generic {
                message: "Developer token has expired".to_owned(),
            })
        } else {
            Ok(self.access_token.clone())
        }
    }
    fn to_json(&self) -> Result<String, Self::Error> {
        match serde_json::to_string(&self) {
            Ok(json) => Ok(json),
            Err(e) => Err(DeveloperTokenError::Generic {
                message: e.to_string(),
            }),
        }
    }
    // fn store_auth(&self) {
    //     if let Some(store_auth_callable) = self.store_auth_callable {
    //         store_auth_callable();
    //     }
    // }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dev_token_new() {
        let dev_token = DeveloperToken::new(Config::default(), "test".to_owned());
        assert_eq!(dev_token.access_token, "test");
        assert_eq!(dev_token.expires_in, 3600);
        assert!(dev_token.expires_by <= Utc::now() + Duration::seconds(3600));
        assert!(dev_token.is_expired() == false);
    }

    #[test]
    fn test_dev_token_json() {
        let dev_token = DeveloperToken::new(Config::default(), "test".to_owned());
        let json = dev_token.to_json().unwrap();
        // println!("{}", json);
        // {"config":{
        //     "base_api_url":"https://api.box.com",
        //     "upload_url":"https://upload.box.com/api",
        //     "oauth2_api_url":"https://api.box.com/oauth2",
        //     "oauth2_authorize_url":"https://account.box.com/api/oauth2/authorize",
        //     "api_version":"2.0",
        //     "max_retry_attempts":5,
        //     "chunk_upload_threads":5,
        //     "user_agent":"
        //     box-rust-sdk/rusty_box"
        // },
        // "access_token":"test",
        // "expires_in":3600,
        // "expires_by":"2023-06-14T23:57:25.660427Z"
        // }
        assert!(json.contains(dev_token.access_token.as_str()));
    }
}
