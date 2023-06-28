//! Structures for the access token response from the Box API.
/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

use crate::rest_api::files::models::file_scope::FileScope;

/// AccessToken : A token that can be used to make authenticated API calls.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccessToken {
    /// The requested access token.
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// The time in seconds in seconds by which this token will expire.
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    /// The type of access token returned.
    #[serde(rename = "token_type", skip_serializing_if = "Option::is_none")]
    pub token_type: Option<TokenType>,
    /// The permissions that this access token permits, providing a list of resources (files, folders, etc) and the scopes permitted for each of those resources.
    #[serde(rename = "restricted_to", skip_serializing_if = "Option::is_none")]
    pub restricted_to: Option<Vec<FileScope>>,
    /// The refresh token for this access token, which can be used to request a new access token when the current one expires.
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// The type of downscoped access token returned. This is only returned if an access token has been downscoped.
    #[serde(rename = "issued_token_type", skip_serializing_if = "Option::is_none")]
    pub issued_token_type: Option<IssuedTokenType>,
}

impl AccessToken {
    /// A token that can be used to make authenticated API calls.
    pub fn new() -> AccessToken {
        AccessToken {
            access_token: None,
            expires_in: None,
            token_type: None,
            restricted_to: None,
            refresh_token: None,
            issued_token_type: None,
        }
    }
}

/// The type of access token returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "bearer")]
    Bearer,
}

impl Default for TokenType {
    fn default() -> TokenType {
        Self::Bearer
    }
}
/// The type of downscoped access token returned. This is only returned if an access token has been downscoped.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IssuedTokenType {
    #[serde(rename = "urn:ietf:params:oauth:token-type:access_token")]
    UrnColonIetfColonParamsColonOauthColonTokenTypeColonAccessToken,
}

impl Default for IssuedTokenType {
    fn default() -> IssuedTokenType {
        Self::UrnColonIetfColonParamsColonOauthColonTokenTypeColonAccessToken
    }
}
