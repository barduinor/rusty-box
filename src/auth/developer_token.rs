use std::error::Error;

use crate::config::Config;
use chrono::{DateTime, Duration, Utc};

use super::Auth;

pub enum DeveloperTokenError {
    Expired,
}

pub struct DeveloperToken {
    config: Config,
    access_token: String,
    expires_in: u32,
    expires_by: DateTime<Utc>,
}

impl DeveloperToken {
    fn new(config: Config, access_token: String) -> Self {
        let mut dev_token = DeveloperToken {
            config: config,
            access_token: access_token,
            expires_in: 3600,
            expires_by: Utc::now(),
        };
        dev_token.expires_by = Utc::now() + Duration::seconds(dev_token.expires_in as i64);
        dev_token
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_by
    }
}

impl Auth for DeveloperToken {
    type Error = DeveloperTokenError;
    fn access_token(&self) -> Result<String, Self::Error> {
        if self.is_expired() {
            Err(DeveloperTokenError::Expired)
        } else {
            Ok(self.access_token.clone())
        }
    }
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
}
