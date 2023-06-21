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

use super::models::post_users_request::PostUsersRequest;
use super::models::post_users_terminate_sessions_request::PostUsersTerminateSessionsRequest;
use super::models::put_users_id_request::PutUsersIdRequest;
use super::models::user::User;
use super::models::user_full::UserFull;
use super::models::users::Users;

use crate::rest_api::api::api_base::*;
use crate::rest_api::api::models::api_configuration_old::Configuration;
use crate::rest_api::api::models::client_error::ClientError;
use crate::rest_api::api::models::session_termination_message::SessionTerminationMessage;

use reqwest;

/// struct for passing parameters to the method [`delete_users_id`]
#[derive(Clone, Debug, Default)]
pub struct DeleteUsersIdParams {
    /// The ID of the user.
    pub user_id: String,
    /// Whether the user will receive email notification of the deletion
    pub notify: Option<bool>,
    /// Whether the user should be deleted even if this user still own files
    pub force: Option<bool>,
}

/// struct for passing parameters to the method [`get_users`]
#[derive(Clone, Debug, Default)]
pub struct GetUsersParams {
    /// Limits the results to only users who's `name` or `login` start with the search term.  For externally managed users, the search term needs to completely match the in order to find the user, and it will only return one user at a time.
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

/// struct for passing parameters to the method [`get_users_id`]
#[derive(Clone, Debug, Default)]
pub struct GetUsersIdParams {
    /// The ID of the user.
    pub user_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
}

/// struct for passing parameters to the method [`get_users_me`]
#[derive(Clone, Debug, Default)]
pub struct GetUsersMeParams {
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
}

/// struct for passing parameters to the method [`post_users`]
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

/// struct for passing parameters to the method [`put_users_id`]
#[derive(Clone, Debug, Default)]
pub struct PutUsersIdParams {
    /// The ID of the user.
    pub user_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    pub put_users_id_request: Option<PutUsersIdRequest>,
}

/// struct for typed errors of method [`delete_users_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUsersIdError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersIdError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users_me`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersMeError {
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_users`]
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

/// struct for typed errors of method [`put_users_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutUsersIdError {
    Status400(ClientError),
    Status403(ClientError),
    DefaultResponse(ClientError),
    UnknownValue(serde_json::Value),
}

/// Deletes a user. By default this will fail if the user still owns any content. Move their owned content first before proceeding, or use the `force` field to delete the user and their files.
pub async fn delete_users_id(
    configuration: &Configuration,
    params: DeleteUsersIdParams,
) -> Result<(), Error<DeleteUsersIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;
    let notify = params.notify;
    let force = params.force;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/users/{user_id}",
        local_var_configuration.base_path,
        user_id = urlencode(user_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = notify {
        local_var_req_builder =
            local_var_req_builder.query(&[("notify", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = force {
        local_var_req_builder =
            local_var_req_builder.query(&[("force", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteUsersIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of all users for the Enterprise along with their `user_id`, `public_name`, and `login`.  The application and the authenticated user need to have the permission to look up users in the entire enterprise.
pub async fn get_users(
    configuration: &Configuration,
    params: GetUsersParams,
) -> Result<Users, Error<GetUsersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_term = params.filter_term;
    let user_type = params.user_type;
    let external_app_user_id = params.external_app_user_id;
    let fields = params.fields;
    let offset = params.offset;
    let limit = params.limit;
    let usemarker = params.usemarker;
    let marker = params.marker;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_term {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter_term", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("user_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = external_app_user_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("external_app_user_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("fields".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "fields",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder =
            local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = usemarker {
        local_var_req_builder =
            local_var_req_builder.query(&[("usemarker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = marker {
        local_var_req_builder =
            local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUsersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves information about a user in the enterprise.  The application and the authenticated user need to have the permission to look up users in the entire enterprise.  This endpoint also returns a limited set of information for external users who are collaborated on content owned by the enterprise for authenticated users with the right scopes. In this case, disallowed fields will return null instead.
pub async fn get_users_id(
    configuration: &Configuration,
    params: GetUsersIdParams,
) -> Result<UserFull, Error<GetUsersIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;
    let fields = params.fields;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/users/{user_id}",
        local_var_configuration.base_path,
        user_id = urlencode(user_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("fields".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "fields",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUsersIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves information about the user who is currently authenticated.  In the case of a client-side authenticated OAuth 2.0 application this will be the user who authorized the app.  In the case of a JWT, server-side authenticated application this will be the service account that belongs to the application by default.  Use the `As-User` header to change who this API call is made on behalf of.
// pub async fn me(client: BoxClient) -> Result<UserFull, AuthError> {
//     let uri = client.auth.base_api_url() + "/users/me";
//     let headers = HashMap::new();
//     let payload = HashMap::new();
//     let resp = client.http.get(&uri, Some(&headers), &payload).await;
//     match resp {
//         Ok(res) => {
//             let user: UserFull = serde_json::from_str(&res).unwrap();
//             Ok(user)
//         }
//         Err(e) => Err(AuthError::Generic {
//             message: e.to_string(),
//             //TODO: fix error type
//         }),
//     }
// }

pub async fn get_users_me(
    configuration: &Configuration,
    params: GetUsersMeParams,
) -> Result<UserFull, Error<GetUsersMeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fields = params.fields;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users/me", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("fields".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "fields",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;

    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUsersMeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new managed user in an enterprise. This endpoint is only available to users and applications with the right admin permissions.
pub async fn post_users(
    configuration: &Configuration,
    params: PostUsersParams,
) -> Result<User, Error<PostUsersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fields = params.fields;
    let post_users_request = params.post_users_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/users", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("fields".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "fields",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_users_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostUsersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Validates the roles and permissions of the user, and creates asynchronous jobs to terminate the user's sessions. Returns the status for the POST request.
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

/// Updates a managed or app user in an enterprise. This endpoint is only available to users and applications with the right admin permissions.
pub async fn put_users_id(
    configuration: &Configuration,
    params: PutUsersIdParams,
) -> Result<UserFull, Error<PutUsersIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;
    let fields = params.fields;
    let put_users_id_request = params.put_users_id_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/users/{user_id}",
        local_var_configuration.base_path,
        user_id = urlencode(user_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(
                &local_var_str
                    .into_iter()
                    .map(|p| ("fields".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "fields",
                &local_var_str
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_users_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutUsersIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
