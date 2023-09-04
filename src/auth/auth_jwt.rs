//! JSON Web Token (JWT) authentication
use super::access_token::AccessToken;
use super::auth_client::{AuthClient, Form};
use super::{Auth, AuthError};
use crate::config::Config;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use josekit::{
    jws::RS512,
    jwt::{self, JwtPayload},
};
use openssl::pkey::PKey;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone, Serialize, Default)]
pub struct JWTAuth {
    pub config: Config,
    client_id: String,
    client_secret: String,
    public_key_id: String,
    box_subject_type: String,
    box_subject_id: String,
    private_key: String,
    passphrase: String,
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
        public_key_id: String,
        box_subject_type: String,
        box_subject_id: String,
        private_key: String,
        passphrase: String,
    ) -> Self {
        JWTAuth {
            config,
            client_id,
            client_secret,
            public_key_id,
            box_subject_type,
            box_subject_id,
            private_key,
            passphrase,

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

fn jwt_assertion(jwt_aut: JWTAuth) -> Result<String, AuthError> {
    // JWT Header
    let mut header = josekit::jws::JwsHeader::new();
    header.set_token_type("JWT");

    header.set_key_id(jwt_aut.public_key_id);

    // JWT Payload
    let mut payload = JwtPayload::new();

    payload.set_issuer(jwt_aut.client_id);

    payload.set_subject(jwt_aut.box_subject_id);

    let box_subject_type = Some(json!(jwt_aut.box_subject_type));
    payload.set_claim("box_sub_type", box_subject_type)?;

    let audience = vec![jwt_aut.config.oauth2_api_url + "/token"];
    payload.set_audience(audience);

    let jwt_id = uuid::Uuid::new_v4().to_string();
    payload.set_jwt_id(jwt_id);

    let expires_at = std::time::SystemTime::now() + std::time::Duration::from_secs(59);
    payload.set_expires_at(&expires_at);

    // decrupt private key
    let private_key = PKey::private_key_from_pem_passphrase(
        &jwt_aut.private_key.as_bytes(),
        jwt_aut.passphrase.as_bytes(),
    )?;

    let private_key_pem = private_key.private_key_to_pem_pkcs8()?;

    let signer = RS512.signer_from_pem(&private_key_pem)?;
    let jwt = jwt::encode_with_signer(&payload, &header, &signer)?;

    Ok(jwt)
}

#[cfg(test)]
#[path = "./unit_tests/auth_jwt_test.rs"]
mod auth_jwt_test;
