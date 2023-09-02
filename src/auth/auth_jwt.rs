//! JSON Web Token (JWT) authentication
use super::access_token::AccessToken;
use super::auth_client::{AuthClient, Form};
use super::{Auth, AuthError};
use crate::config::Config;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct JWTAuth {
    pub config: Config,

    client_id: String,
    client_secret: String,
    publick_key_id: String,
    private_key: String,
    passphrase: String,
    enterprise_id: String,

    access_token: AccessToken,
    expires_by: DateTime<Utc>,
    #[serde(skip)]
    client: AuthClient,
}

impl JWTAuth {
    pub fn new(
        config: Config,
        client_id: String,
        client_secret: String,
        publick_key_id: String,
        private_key: String,
        passphrase: String,
        enterprise_id: String,
    ) -> Self {
        JWTAuth {
            config,
            client_id,
            client_secret,
            publick_key_id,
            private_key,
            passphrase,
            enterprise_id,
            access_token: AccessToken::new(),
            expires_by: Utc::now(),
            client: AuthClient::default(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_by - Duration::seconds(60 * 5)
    }

    async fn fetch_access_token(&mut self) -> Result<AccessToken, AuthError> {
        let url = &(self.config.oauth2_api_url.clone() + "/token");

        let headers = None; // TODO: Add headers to rquest

        let mut payload = Form::new();
        payload.insert("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");
        payload.insert("client_id", &self.client_id);
        payload.insert("client_secret", &self.client_secret);

        payload.insert("assertion", ""); //TODO: Create assertion

        let now = Utc::now();

        let response = self.client.post_form(url, headers, &payload).await;

        let data = match response {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        let access_token = match serde_json::from_str::<AccessToken>(&data) {
            Ok(access_token) => access_token,
            Err(e) => {
                return Err(AuthError::Serde(e));
            }
        };
        let expires_in = access_token.expires_in.unwrap_or_default();
        self.expires_by = now + Duration::seconds(expires_in);
        self.access_token = access_token.clone();
        Ok(access_token)
    }
}

fn create_jet_assertion() {}
