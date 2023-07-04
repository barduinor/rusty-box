//! Client Credentials Grant (CCG) authentication
use super::access_token::AccessToken;
use super::auth_client::{AuthClient, Form};
use super::{Auth, AuthError};
use crate::config::Config;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;

/// Client Credentials Grant (CCG) authentication
#[derive(Debug, Clone, Serialize, Default)]
pub struct OAuth {
    pub config: Config,
    client_id: String,
    client_secret: String,
    access_token: AccessToken,
    expires_by: DateTime<Utc>,

    #[serde(skip)]
    client: AuthClient,

    #[serde(skip)]
    store: Option<fn(String)>,
}

impl OAuth {
    pub fn new(
        config: Config,
        client_id: String,
        client_secret: String,
        store: Option<fn(String)>,
    ) -> Self {
        OAuth {
            config,
            client_id,
            client_secret,
            access_token: AccessToken::new(),
            expires_by: Utc::now(),
            client: AuthClient::default(),
            store,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_by
    }

    pub fn authorization_url(
        &self,
        redirect_url: Option<String>,
        scope: Option<String>, // TODO: vector of strings?
        state: Option<String>,
    ) -> Result<(String, String), AuthError> {
        let url = self.config.oauth2_authorize_url.clone();
        let url = url + "?client_id=" + &self.client_id;
        let url = url + "&response_type=code";

        let url = match scope {
            Some(scope) => url + "&scope=" + scope.as_str(),
            None => url,
        };

        let local_state = match state {
            Some(state) => state,
            None => "box_csrf_token_".to_string() + &generate_state(16),
        };
        let url = url + "&state=" + local_state.as_str();

        let url = match redirect_url {
            Some(redirect_url) => url + "&redirect_uri=" + &urlencode(redirect_url.as_str()),
            None => url,
        };

        Ok((url, local_state))
    }

    pub async fn request_access_token(
        &mut self,
        // client_id: String,
        // client_secret: String,
        code: String,
    ) -> Result<AccessToken, AuthError> {
        let url = self.config.oauth2_api_url.clone() + "/token";

        let headers = None; // TODO: Add headers to rquest

        let mut payload = Form::new();
        payload.insert("client_id", &self.client_id);
        payload.insert("client_secret", &self.client_secret);
        payload.insert("grant_type", "authorization_code");
        payload.insert("code", &code);

        let now = Utc::now();

        let response = self.client.post_form(&url, headers, &payload).await;

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
        match self.store {
            Some(store) => {
                let json_access_token = self.to_json().await?;
                store(json_access_token);
            }
            None => return Ok(access_token),
        };
        Ok(access_token)
    }

    pub fn set_access_token(&mut self, access_token: AccessToken) -> Result<(), AuthError> {
        self.access_token = access_token;
        match self.store {
            Some(store) => {
                let json_access_token = serde_json::to_string(&self)?;
                store(json_access_token);
            }
            None => return Ok(()),
        };
        Ok(())
    }

    async fn refresh_access_token(&mut self) -> Result<AccessToken, AuthError> {
        let url = self.config.oauth2_api_url.clone() + "/token";

        let refresh_token = self.access_token.refresh_token.clone().unwrap_or_default();

        let headers = None; // TODO: Add headers to rquest

        let mut payload = Form::new();
        payload.insert("grant_type", "client_credentials");
        payload.insert("client_id", &self.client_id);
        payload.insert("client_secret", &self.client_secret);
        payload.insert("grant_type", "refresh_token");
        payload.insert("refresh_token", &refresh_token);

        let now = Utc::now();

        let response = self.client.post_form(&url, headers, &payload).await;

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
        match self.store {
            Some(store) => {
                let json_access_token = self.to_json().await?;
                store(json_access_token);
            }
            None => return Ok(access_token),
        };
        Ok(access_token)
    }
}

#[async_trait]
impl<'a> Auth<'a> for OAuth {
    async fn access_token(&mut self) -> Result<String, AuthError> {
        if self.is_expired() {
            match self.refresh_access_token().await {
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

fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

fn generate_state(length: u8) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.into())
        .map(char::from)
        .collect()
}

#[cfg(test)]
fn store(json_access_token: String) {
    // println!("{}", json_access_token);
    assert!(json_access_token.len() > 0);
    assert!(json_access_token.contains("ACCESS_TOKEN"));
    assert!(json_access_token.contains("REFRESH_TOKEN"));
}
#[test]
fn test_generate_state() {
    let state = generate_state(16);
    assert_eq!(state.len(), 16);
}

#[test]
fn test_urlencode() {
    let url = "https://example.com";
    let encoded_url = urlencode(url);
    assert_eq!(encoded_url, "https%3A%2F%2Fexample.com");
}

#[test]
fn test_authorization_url_default() {
    let config = Config::new();
    let auth = OAuth::new(
        config,
        "client_id".to_owned(),
        "client_secret".to_owned(),
        None,
    );

    let (auth_url, state) = auth.authorization_url(None, None, None).unwrap_or_default();

    // check if auth_url contains all required params
    assert!(auth_url.contains("client_id=client_id"));
    assert!(auth_url.contains("response_type=code"));
    assert!(auth_url.contains("state=box_csrf_token_"));
    assert!(state.contains("box_csrf_token_"));

    // check if auth_url does not contain optional scope or redirect_url
    assert!(!auth_url.contains("scope="));
    assert!(!auth_url.contains("redirect_uri="));
}

#[test]
fn test_authorization_url_state() {
    let config = Config::new();
    let auth = OAuth::new(
        config,
        "client_id".to_owned(),
        "client_secret".to_owned(),
        None,
    );

    let (auth_url, state) = auth
        .authorization_url(None, None, Some("ABCDEF".to_string()))
        .unwrap_or_default();

    // check if auth_url contains all required params
    assert!(auth_url.contains("client_id=client_id"));
    assert!(auth_url.contains("response_type=code"));
    assert!(auth_url.contains("state=ABCDEF"));
    assert_eq!(state, "ABCDEF");

    // check if auth_url does not contain optional scope or redirect_url
    assert!(!auth_url.contains("scope="));
    assert!(!auth_url.contains("redirect_uri="));
}

#[test]
fn test_authorization_url_redirect() {
    let config = Config::new();
    let auth = OAuth::new(
        config,
        "client_id".to_owned(),
        "client_secret".to_owned(),
        None,
    );

    let (auth_url, state) = auth
        .authorization_url(Some("https://example.com".to_string()), None, None)
        .unwrap_or_default();

    let encoded_redirect = "redirect_uri=".to_string() + &urlencode("https://example.com");

    // check if auth_url contains all required params
    assert!(auth_url.contains("client_id=client_id"));
    assert!(auth_url.contains("response_type=code"));
    assert!(auth_url.contains("state=box_csrf_token_"));
    assert!(state.contains("box_csrf_token_"));
    assert!(auth_url.contains(&encoded_redirect));

    // check if auth_url does not contain optional scope or redirect_url
    assert!(!auth_url.contains("scope="));
}
#[test]
fn test_authorization_url_scope() {
    let config = Config::new();
    let auth = OAuth::new(
        config,
        "client_id".to_owned(),
        "client_secret".to_owned(),
        None,
    );

    let (auth_url, state) = auth
        .authorization_url(None, Some("admin_readwrite".to_string()), None)
        .unwrap_or_default();

    // check if auth_url contains all required params
    assert!(auth_url.contains("client_id=client_id"));
    assert!(auth_url.contains("response_type=code"));
    assert!(auth_url.contains("state=box_csrf_token_"));
    assert!(state.contains("box_csrf_token_"));
    assert!(auth_url.contains("scope=admin_readwrite"));

    // check if auth_url does not contain optional scope or redirect_url
}

#[test]
fn test_oauth_new() {
    let config = Config::new();
    let client_id = "client_id".to_owned();
    let client_secret = "client_secret".to_owned();
    let auth = OAuth::new(config, client_id, client_secret, None);

    assert_eq!(auth.client_id, "client_id".to_owned());
    assert_eq!(auth.client_secret, "client_secret".to_owned());
}

#[test]
fn test_oauth_set_access_token() {
    let config = Config::new();
    let mut auth = OAuth::new(
        config,
        "client_id".to_owned(),
        "client_secret".to_owned(),
        None,
    );
    let access_token = AccessToken {
        access_token: Some("access_token".to_owned()),
        refresh_token: Some("refresh_token".to_owned()),
        ..Default::default()
    };
    match auth.set_access_token(access_token.clone()) {
        Ok(_) => assert_eq!(auth.access_token, access_token),
        Err(_) => panic!("Error setting access token"),
    };
}

#[test]
fn test_store() {
    let config = Config::new();
    let mut auth = OAuth::new(
        config,
        "client_id".to_owned(),
        "client_secret".to_owned(),
        Some(store),
    );
    let fake_access_token = AccessToken {
        access_token: Some("ACCESS_TOKEN".to_string()),
        expires_in: Some(3333),
        token_type: Some(super::access_token::TokenType::Bearer),
        restricted_to: None,
        refresh_token: Some("REFRESH_TOKEN".to_string()),
        issued_token_type: None,
    };
    match auth.set_access_token(fake_access_token) {
        Ok(_) => {}
        Err(_) => panic!("Error setting access token"),
    };
}
