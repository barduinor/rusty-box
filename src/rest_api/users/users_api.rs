/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */
//

use std::collections::HashMap;

use serde_json::json;

use super::models::post_users_request::PostUsersRequest;
use super::models::post_users_terminate_sessions_request::PostUsersTerminateSessionsRequest;
use super::models::put_users_id_request::PutUsersIdRequest;
use super::models::user_full::UserFull;
use super::models::users::Users;

use crate::auth::AuthError;
use crate::box_client::BoxClient;
use crate::http_client::BaseHttpClient;
use crate::rest_api::api::api_base::*;
use crate::rest_api::api::models::api_configuration_old::Configuration;
use crate::rest_api::api::models::client_error::ClientError;
use crate::rest_api::api::models::session_termination_message::SessionTerminationMessage;

/// struct for passing parameters to the method
/// DEPRECATED
#[derive(Clone, Debug, Default)]
pub struct DeleteUsersIdParams {
    /// The ID of the user.
    pub user_id: String,
    /// Whether the user will receive email notification of the deletion
    pub notify: Option<bool>,
    /// Whether the user should be deleted even if this user still own files
    pub force: Option<bool>,
}

/// struct for passing parameters to the method [`list`]
#[derive(Clone, Debug, Default)]
pub struct GetUsersParams {
    /// Limits the results to only users who's `name` or `login` start with the search term.  
    /// For externally managed users, the search term needs to completely match the in order to find the user, and it will only return one user at a time.
    pub filter_term: Option<String>,
    /// Limits the results to the kind of user specified.  * `all` returns every kind of user for whom the   `login` or `name` partially matches the   `filter_term`. It will only return an external user   if the login matches the `filter_term` completely,   and in that case it will only return that user. * `managed` returns all managed and app users for whom   the `login` or `name` partially matches the   `filter_term`. * `external` returns all external users for whom the   `login` matches the `filter_term` exactly.
    pub user_type: Option<String>,
    /// Limits the results to app users with the given `external_app_user_id` value.  When creating an app user, an `external_app_user_id` value can be set. This value can then be used in this endpoint to find any users that match that `external_app_user_id` value.
    pub external_app_user_id: Option<String>,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    /// The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response.
    pub offset: Option<i64>,
    /// The maximum number of items to return per page.
    pub limit: Option<i64>,
    /// Specifies whether to use marker-based pagination instead of offset-based pagination. Only one pagination method can be used at a time.  By setting this value to true, the API will return a `marker` field that can be passed as a parameter to this endpoint to get the next page of the response.
    pub usemarker: Option<bool>,
    /// Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`.
    pub marker: Option<String>,
}

/// struct for passing parameters to the method
/// DEPRECATED
#[derive(Clone, Debug, Default)]
pub struct GetUsersIdParams {
    /// The ID of the user.
    pub user_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
}

/// struct for passing parameters to the method
/// DEPRECATED
#[derive(Clone, Debug, Default)]
pub struct GetUsersMeParams {
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
}

/// struct for passing parameters to the method
/// DEPRECATED
#[derive(Clone, Debug, Default)]
pub struct PostUsersParams {
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    pub post_users_request: Option<PostUsersRequest>,
}

/// struct for passing parameters to the method [`post_users_terminate_sessions`]
#[derive(Clone, Debug, Default)]
pub struct PostUsersTerminateSessionsParams {
    pub post_users_terminate_sessions_request: Option<PostUsersTerminateSessionsRequest>,
}

/// struct for passing parameters to the method
/// DEPRECATED
#[derive(Clone, Debug, Default)]
pub struct PutUsersIdParams {
    /// The ID of the user.
    pub user_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    pub put_users_id_request: Option<PutUsersIdRequest>,
}

/// struct for typed errors of method
/// DEPRECATED
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUsersIdError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method
/// DEPRECATED
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersIdError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

// struct for typed errors of method [`get_users_me`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersMeError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method
/// DEPRECATED
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUsersError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_users_terminate_sessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostUsersTerminateSessionsError {
    Status400(ClientError),
    Status403(ClientError),
    Status404(ClientError),
    Status429(ClientError),
    Status500(ClientError),
    Status503(ClientError),
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method
/// DEPRECATED
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutUsersIdError {
    Status400(ClientError),
    Status403(ClientError),
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// Deletes a user.
/// By default this will fail if the user still owns any content.
/// Move their owned content first before proceeding, or use the `force` field to delete the user and their files.
pub async fn delete(
    client: &mut BoxClient<'_>,
    user_id: &str,
    notify: Option<bool>,
    force: Option<bool>,
) -> Result<(), AuthError> {
    let uri = client.auth.base_api_url() + "/users" + format!("/{}", user_id).as_str();
    let headers = client.headers().await?;

    //TODO: these need to go in the query string
    let value = serde_json::json!({
        "notify": notify,
        "force": force,
    });

    let resp = client.http.delete(&uri, Some(&headers), &value).await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(AuthError::RequestError(e)),
    }
}
// pub async fn delete_users_id(
//     configuration: &Configuration,
//     params: DeleteUsersIdParams,
// ) -> Result<(), Error<DeleteUsersIdError>> {}

/// Returns a list of all users for the Enterprise along with their `user_id`, `public_name`, and `login`.  
/// The application and the authenticated user need to have the permission to look up users in the entire enterprise.
pub async fn list(
    client: &mut BoxClient<'_>,
    params: Option<GetUsersParams>,
) -> Result<Users, AuthError> {
    let uri = client.auth.base_api_url() + "/users";
    let headers = client.headers().await?;

    let params = params.unwrap_or_default();

    let filter_term = params.filter_term.unwrap_or_default();
    let user_type = params.user_type.unwrap_or_default();
    let external_app_user_id = params.external_app_user_id.unwrap_or_default();

    let fields = params
        .fields
        .unwrap_or(vec![])
        .into_iter()
        .collect::<Vec<String>>()
        .join(",")
        .to_string();

    let offset = params.offset.unwrap_or_default().to_string();
    let limit = params.limit.unwrap_or_default().to_string();
    let usemarker = params.usemarker.unwrap_or_default().to_string();
    let marker = params.marker.unwrap_or_default().to_string();

    let mut payload = HashMap::new();

    if filter_term != String::default() {
        payload.insert("filter_term", filter_term.as_str());
    }
    if user_type != String::default() {
        payload.insert("user_type", user_type.as_str());
    }
    if external_app_user_id != String::default() {
        payload.insert("external_app_user_id", external_app_user_id.as_str());
    }
    if fields != String::default() {
        payload.insert("fields", fields.as_str());
    }
    if offset != "0" {
        payload.insert("offset", offset.as_str());
    }
    if limit != "0" {
        payload.insert("limit", limit.as_str());
    }
    if usemarker != String::default() {
        payload.insert("usemarker", usemarker.as_str());
    }
    if marker != String::default() {
        payload.insert("marker", marker.as_str());
    }

    let resp = client.http.get(&uri, Some(&headers), &payload).await;

    match resp {
        Ok(res) => {
            let users = serde_json::from_str::<Users>(&res)?;
            Ok(users)
        }
        Err(e) => Err(AuthError::RequestError(e)),
    }
}

// pub async fn get_users(
//     configuration: &Configuration,
//     params: GetUsersParams,
// ) -> Result<Users, Error<GetUsersError>> {}

/// Retrieves information about a user in the enterprise.  
/// The application and the authenticated user need to have the permission to look up users in the entire enterprise.  
/// This endpoint also returns a limited set of information for external users who are collaborated on content owned by the enterprise for authenticated users with the right scopes.
/// In this case, disallowed fields will return null instead.
pub async fn id(
    client: &mut BoxClient<'_>,
    user_id: &str,
    fields: Option<Vec<String>>,
) -> Result<UserFull, AuthError> {
    let uri = client.auth.base_api_url() + "/users" + format!("/{}", user_id).as_str();
    let headers = client.headers().await?;

    let fields = fields
        .unwrap_or(vec![])
        .into_iter()
        .collect::<Vec<String>>()
        .join(",")
        .to_string();

    let mut payload = HashMap::new();
    payload.insert("fields", fields.as_str());

    let resp = client.http.get(&uri, Some(&headers), &payload).await;
    match resp {
        Ok(res) => {
            let user = serde_json::from_str::<UserFull>(&res)?;
            Ok(user)
        }
        Err(e) => Err(AuthError::RequestError(e)),
    }
}

// pub async fn get_users_id(
//     configuration: &Configuration,
//     params: GetUsersIdParams,
// ) -> Result<UserFull, Error<GetUsersIdError>> {}

/// Retrieves information about the user who is currently authenticated.  
/// In the case of a client-side authenticated OAuth 2.0 application this will be the user who authorized the app.  
/// In the case of a JWT, server-side authenticated application this will be the service account that belongs to the application by default.  
/// Use the `As-User` header to change who this API call is made on behalf of.
pub async fn me(
    client: &mut BoxClient<'_>,
    fields: Option<Vec<String>>,
) -> Result<UserFull, AuthError> {
    let uri = client.auth.base_api_url() + "/users/me";
    let headers = client.headers().await?;

    let fields = fields
        .unwrap_or(vec![])
        .into_iter()
        .collect::<Vec<String>>()
        .join(",")
        .to_string();

    let mut payload = HashMap::new();
    payload.insert("fields", fields.as_str());

    let resp = client.http.get(&uri, Some(&headers), &payload).await;
    match resp {
        Ok(res) => {
            let user = serde_json::from_str::<UserFull>(&res)?;
            Ok(user)
        }
        Err(e) => Err(AuthError::RequestError(e)),
    }
}

/// Creates a new managed user in an enterprise.
/// This endpoint is only available to users and applications with the right admin permissions.

pub async fn create(
    client: &mut BoxClient<'_>,
    user: PostUsersRequest,
    // fields: Option<Vec<String>>,
) -> Result<UserFull, AuthError> {
    let uri = client.auth.base_api_url() + "/users";
    let headers = client.headers().await?;

    // TODO: Implement query fields on the post request
    // let fields = fields
    //     .unwrap_or(vec![])
    //     .into_iter()
    //     .collect::<Vec<String>>()
    //     .join(",")
    //     .to_string();

    // let mut payload = HashMap::new();
    // payload.insert("fields", fields.as_str());

    // convert the postusersrequest to json
    let value_json = serde_json::to_string(&user)?;
    let value = serde_json::from_str(&value_json)?;

    let resp = client.http.post(&uri, Some(&headers), &value).await;
    match resp {
        Ok(res) => {
            let user = serde_json::from_str::<UserFull>(&res)?;
            Ok(user)
        }
        Err(e) => Err(AuthError::RequestError(e)),
    }
}
// pub async fn post_users(
//     configuration: &Configuration,
//     params: PostUsersParams,
// ) -> Result<User, Error<PostUsersError>> {}

/// Validates the roles and permissions of the user,
/// and creates asynchronous jobs to terminate the user's sessions.
/// Returns the status for the POST request.
pub async fn terminate_sessions_by_user_ids(
    client: &mut BoxClient<'_>,
    user_ids: Vec<String>,
) -> Result<Option<String>, AuthError> {
    let uri = client.auth.base_api_url() + "/users/terminate_sessions";
    let headers = client.headers().await?;

    let mut value = HashMap::new();
    value.insert("user_ids", user_ids);
    let value = json!(value);

    let resp = client.http.post(&uri, Some(&headers), &value).await;
    match resp {
        Ok(res) => Ok(Some(res)),
        Err(e) => Err(AuthError::RequestError(e)),
    }
}

/// Validates the roles and permissions of the user,
/// and creates asynchronous jobs to terminate the user's sessions.
/// Returns the status for the POST request.
pub async fn terminate_sessions_by_user_logins(
    client: &mut BoxClient<'_>,
    user_logins: Vec<String>,
) -> Result<Option<String>, AuthError> {
    let uri = client.auth.base_api_url() + "/users/terminate_sessions";
    let headers = client.headers().await?;

    let mut value = HashMap::new();
    value.insert("user_logins", user_logins);
    let value = json!(value);

    let resp = client.http.post(&uri, Some(&headers), &value).await;
    match resp {
        Ok(res) => Ok(Some(res)),
        Err(e) => Err(AuthError::RequestError(e)),
    }
}

/// Validates the roles and permissions of the user,
/// and creates asynchronous jobs to terminate the user's sessions.
/// Returns the status for the POST request.
pub async fn terminate_sessions_by_group_ids(
    client: &mut BoxClient<'_>,
    group_ids: Vec<String>,
    // fields: Option<Vec<String>>,
) -> Result<Option<String>, AuthError> {
    let uri = client.auth.base_api_url() + "/users/terminate_sessions";
    let headers = client.headers().await?;

    let mut value = HashMap::new();
    value.insert("user_ids", group_ids);
    let value = json!(value);

    let resp = client.http.post(&uri, Some(&headers), &value).await;
    match resp {
        Ok(res) => Ok(Some(res)),
        Err(e) => Err(AuthError::RequestError(e)),
    }
}
pub async fn post_users_terminate_sessions(
    configuration: &Configuration,
    params: PostUsersTerminateSessionsParams,
) -> Result<SessionTerminationMessage, Error<PostUsersTerminateSessionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let post_users_terminate_sessions_request = params.post_users_terminate_sessions_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/users/terminate_sessions",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_users_terminate_sessions_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostUsersTerminateSessionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a managed or app user in an enterprise.
/// This endpoint is only available to users and applications with the right admin permissions.
pub async fn update(
    client: &mut BoxClient<'_>,
    user_id: &str,
    user: PutUsersIdRequest,
    // fields: Option<Vec<String>>,
) -> Result<UserFull, AuthError> {
    let uri = client.auth.base_api_url() + "/users" + format!("/{}", user_id).as_str();
    let headers = client.headers().await?;

    // TODO: Implement query fields on the post request
    // let fields = fields
    //     .unwrap_or(vec![])
    //     .into_iter()
    //     .collect::<Vec<String>>()
    //     .join(",")
    //     .to_string();

    // let mut payload = HashMap::new();
    // payload.insert("fields", fields.as_str());

    // convert the postusersrequest to json
    let value_json = serde_json::to_string(&user)?;
    let value = serde_json::from_str(&value_json)?;

    let resp = client.http.put(&uri, Some(&headers), &value).await;
    match resp {
        Ok(res) => {
            let user = serde_json::from_str::<UserFull>(&res)?;
            Ok(user)
        }
        Err(e) => Err(AuthError::RequestError(e)),
    }
}
// pub async fn put_users_id(
//     configuration: &Configuration,
//     params: PutUsersIdParams,
// ) -> Result<UserFull, Error<PutUsersIdError>> {}
