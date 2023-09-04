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

/// The type of subject that is being authenticated (user or enterprise)
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

#[derive(Debug, Clone, Serialize, Default)]
pub struct JWTAuth {
    pub config: Config,
    client_id: String,
    client_secret: String,

    box_subject_type: SubjectType,
    box_subject_id: String,

    public_key_id: String,
    #[serde(skip)]
    private_key: String,
    #[serde(skip)]
    passphrase: String,
    access_token: AccessToken,
    expires_by: DateTime<Utc>,
    #[serde(skip)]
    client: AuthClient,
}

impl JWTAuth {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: Config,
        client_id: String,
        client_secret: String,

        box_subject_type: SubjectType,
        box_subject_id: String,

        public_key_id: String,
        private_key: String,
        passphrase: String,
    ) -> Self {
        JWTAuth {
            config,
            client_id,
            client_secret,

            box_subject_type,
            box_subject_id,

            public_key_id,
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

        let jwt_token = jwt_assertion(self.clone())?;

        let mut payload = Form::new();
        payload.insert("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");
        payload.insert("client_id", &self.client_id);
        payload.insert("client_secret", &self.client_secret);

        payload.insert("assertion", &jwt_token);

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

    let box_subject_type = Some(json!(jwt_aut.box_subject_type.value()));
    payload.set_claim("box_sub_type", box_subject_type)?;

    let audience = vec![jwt_aut.config.oauth2_api_url + "/token"];
    payload.set_audience(audience);

    let jwt_id = uuid::Uuid::new_v4().to_string();
    payload.set_jwt_id(jwt_id);

    let expires_at = std::time::SystemTime::now() + std::time::Duration::from_secs(59);
    payload.set_expires_at(&expires_at);

    // decrupt private key
    let private_key = PKey::private_key_from_pem_passphrase(
        jwt_aut.private_key.as_bytes(),
        jwt_aut.passphrase.as_bytes(),
    )?;

    let private_key_pem = private_key.private_key_to_pem_pkcs8()?;

    let signer = RS512.signer_from_pem(private_key_pem)?;
    let jwt = jwt::encode_with_signer(&payload, &header, &signer)?;

    Ok(jwt)
}

#[async_trait]
impl<'a> Auth<'a> for JWTAuth {
    async fn access_token(&mut self) -> Result<String, AuthError> {
        if self.is_expired() {
            match self.fetch_access_token().await {
                Ok(access_token) => Ok(access_token.access_token.unwrap_or_default()),
                Err(e) => Err(e),
            }
        } else {
            let access_token = match self.access_token.access_token.clone() {
                Some(token) => token,
                None => return Err(AuthError::Generic("CCG token is not set".to_owned())),
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
#[path = "./unit_tests/auth_jwt_test.rs"]
mod auth_jwt_test;
