use super::o_auth2_error::OAuth2Error;

// struct for passing parameters to the method [`get_authorize`]
#[derive(Clone, Debug, Default)]
pub struct GetAuthorizeParams {
    /// The type of response we'd like to receive.
    pub response_type: String,
    /// The Client ID of the application that is requesting to authenticate the user. To get the Client ID for your application, log in to your Box developer console and click the **Edit Application** link for the application you're working with. In the OAuth 2.0 Parameters section of the configuration page, find the item labelled `client_id`. The text of that item is your application's Client ID.
    pub client_id: String,
    /// The URI to which Box redirects the browser after the user has granted or denied the application permission. This URI match one of the redirect URIs in the configuration of your application. It must be a valid HTTPS URI and it needs to be able to handle the redirection to complete the next step in the OAuth 2.0 flow. Although this parameter is optional, it must be a part of the authorization URL if you configured multiple redirect URIs for the application in the developer console. A missing parameter causes a `redirect_uri_missing` error after the user grants application access.
    pub redirect_uri: Option<String>,
    /// A custom string of your choice. Box will pass the same string to the redirect URL when authentication is complete. This parameter can be used to identify a user on redirect, as well as protect against hijacked sessions and other exploits.
    pub state: Option<String>,
    /// A comma-separated list of application scopes you'd like to authenticate the user for. This defaults to all the scopes configured for the application in its configuration page.
    pub scope: Option<String>,
}

// struct for passing parameters to the method [`post_oauth2_revoke`]
#[derive(Clone, Debug, Default)]
pub struct PostOauth2RevokeParams {
    /// The Client ID of the application requesting to revoke the access token.
    pub client_id: Option<String>,
    /// The client secret of the application requesting to revoke an access token.
    pub client_secret: Option<String>,
    /// The access token to revoke.
    pub token: Option<String>,
}

// struct for passing parameters to the method [`post_oauth2_token`]
#[derive(Clone, Debug, Default)]
pub struct PostOauth2TokenParams {
    /// The type of request being made, either using a client-side obtained authorization code, a refresh token, a JWT assertion, client credentials grant or another access token for the purpose of downscoping a token.
    pub grant_type: String,
    /// The Client ID of the application requesting an access token.  Used in combination with `authorization_code`, `client_credentials`, or `urn:ietf:params:oauth:grant-type:jwt-bearer` as the `grant_type`.
    pub client_id: Option<String>,
    /// The client secret of the application requesting an access token.  Used in combination with `authorization_code`, `client_credentials`, or `urn:ietf:params:oauth:grant-type:jwt-bearer` as the `grant_type`.
    pub client_secret: Option<String>,
    /// The client-side authorization code passed to your application by Box in the browser redirect after the user has successfully granted your application permission to make API calls on their behalf.  Used in combination with `authorization_code` as the `grant_type`.
    pub code: Option<String>,
    /// A refresh token used to get a new access token with.  Used in combination with `refresh_token` as the `grant_type`.
    pub refresh_token: Option<String>,
    /// A JWT assertion for which to request a new access token.  Used in combination with `urn:ietf:params:oauth:grant-type:jwt-bearer` as the `grant_type`.
    pub assertion: Option<String>,
    /// The token to exchange for a downscoped token. This can be a regular access token, a JWT assertion, or an app token.  Used in combination with `urn:ietf:params:oauth:grant-type:token-exchange` as the `grant_type`.
    pub subject_token: Option<String>,
    /// The type of `subject_token` passed in.  Used in combination with `urn:ietf:params:oauth:grant-type:token-exchange` as the `grant_type`.
    pub subject_token_type: Option<String>,
    /// The token used to create an annotator token. This is a JWT assertion.  Used in combination with `urn:ietf:params:oauth:grant-type:token-exchange` as the `grant_type`.
    pub actor_token: Option<String>,
    /// The type of `actor_token` passed in.  Used in combination with `urn:ietf:params:oauth:grant-type:token-exchange` as the `grant_type`.
    pub actor_token_type: Option<String>,
    /// The space-delimited list of scopes that you want apply to the new access token.  The `subject_token` will need to have all of these scopes or the call will error with **401 Unauthorized**.
    pub scope: Option<String>,
    /// Full URL for the file that the token should be generated for.
    pub resource: Option<String>,
    /// Used in combination with `client_credentials` as the `grant_type`.
    pub box_subject_type: Option<String>,
    /// Used in combination with `client_credentials` as the `grant_type`. Value is determined by `box_subject_type`. If `user` use user ID and if `enterprise` use enterprise ID.
    pub box_subject_id: Option<String>,
    /// Full URL of the shared link on the file or folder that the token should be generated for.
    pub box_shared_link: Option<String>,
}

// struct for passing parameters to the method [`post_oauth2_token_refresh`]
#[derive(Clone, Debug, Default)]
pub struct PostOauth2TokenRefreshParams {
    /// The type of request being made, in this case a refresh request.
    pub grant_type: String,
    /// The client ID of the application requesting to refresh the token.
    pub client_id: String,
    /// The client secret of the application requesting to refresh the token.
    pub client_secret: String,
    /// The refresh token to refresh.
    pub refresh_token: String,
}

// struct for typed errors of method [`get_authorize`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAuthorizeError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

// struct for typed errors of method [`post_oauth2_revoke`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostOauth2RevokeError {
    Status400(OAuth2Error),
    DefaultResponse(OAuth2Error),
    UnknownValue(serde_json::Value),
}

// struct for typed errors of method [`post_oauth2_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostOauth2TokenError {
    Status400(OAuth2Error),
    DefaultResponse(OAuth2Error),
    UnknownValue(serde_json::Value),
}

// struct for typed errors of method [`post_oauth2_token_refresh`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostOauth2TokenRefreshError {
    Status400(OAuth2Error),
    DefaultResponse(OAuth2Error),
    UnknownValue(serde_json::Value),
}
