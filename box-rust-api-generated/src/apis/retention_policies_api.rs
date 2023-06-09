/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`delete_retention_policies_id`]
#[derive(Clone, Debug, Default)]
pub struct DeleteRetentionPoliciesIdParams {
    /// The ID of the retention policy.
    pub retention_policy_id: String
}

/// struct for passing parameters to the method [`get_retention_policies`]
#[derive(Clone, Debug, Default)]
pub struct GetRetentionPoliciesParams {
    /// Filters results by a case sensitive prefix of the name of retention policies.
    pub policy_name: Option<String>,
    /// Filters results by the type of retention policy.
    pub policy_type: Option<String>,
    /// Filters results by the ID of the user who created policy.
    pub created_by_user_id: Option<String>,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>,
    /// The maximum number of items to return per page.
    pub limit: Option<i64>,
    /// Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.
    pub marker: Option<String>
}

/// struct for passing parameters to the method [`get_retention_policies_id`]
#[derive(Clone, Debug, Default)]
pub struct GetRetentionPoliciesIdParams {
    /// The ID of the retention policy.
    pub retention_policy_id: String,
    /// A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.
    pub fields: Option<Vec<String>>
}

/// struct for passing parameters to the method [`post_retention_policies`]
#[derive(Clone, Debug, Default)]
pub struct PostRetentionPoliciesParams {
    pub post_retention_policies_request: Option<crate::models::PostRetentionPoliciesRequest>
}

/// struct for passing parameters to the method [`put_retention_policies_id`]
#[derive(Clone, Debug, Default)]
pub struct PutRetentionPoliciesIdParams {
    /// The ID of the retention policy.
    pub retention_policy_id: String,
    pub put_retention_policies_id_request: Option<crate::models::PutRetentionPoliciesIdRequest>
}


/// struct for typed errors of method [`delete_retention_policies_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRetentionPoliciesIdError {
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_retention_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRetentionPoliciesError {
    Status400(crate::models::ClientError),
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_retention_policies_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRetentionPoliciesIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_retention_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostRetentionPoliciesError {
    Status400(crate::models::ClientError),
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_retention_policies_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutRetentionPoliciesIdError {
    Status400(crate::models::ClientError),
    Status403(crate::models::ClientError),
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Permanently deletes a retention policy.
pub async fn delete_retention_policies_id(configuration: &configuration::Configuration, params: DeleteRetentionPoliciesIdParams) -> Result<(), Error<DeleteRetentionPoliciesIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let retention_policy_id = params.retention_policy_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/retention_policies/{retention_policy_id}", local_var_configuration.base_path, retention_policy_id=crate::apis::urlencode(retention_policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<DeleteRetentionPoliciesIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves all of the retention policies for an enterprise.
pub async fn get_retention_policies(configuration: &configuration::Configuration, params: GetRetentionPoliciesParams) -> Result<crate::models::RetentionPolicies, Error<GetRetentionPoliciesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let policy_name = params.policy_name;
    let policy_type = params.policy_type;
    let created_by_user_id = params.created_by_user_id;
    let fields = params.fields;
    let limit = params.limit;
    let marker = params.marker;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/retention_policies", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = policy_name {
        local_var_req_builder = local_var_req_builder.query(&[("policy_name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = policy_type {
        local_var_req_builder = local_var_req_builder.query(&[("policy_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_by_user_id {
        local_var_req_builder = local_var_req_builder.query(&[("created_by_user_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<GetRetentionPoliciesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a retention policy.
pub async fn get_retention_policies_id(configuration: &configuration::Configuration, params: GetRetentionPoliciesIdParams) -> Result<crate::models::RetentionPolicy, Error<GetRetentionPoliciesIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let retention_policy_id = params.retention_policy_id;
    let fields = params.fields;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/retention_policies/{retention_policy_id}", local_var_configuration.base_path, retention_policy_id=crate::apis::urlencode(retention_policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<GetRetentionPoliciesIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a retention policy.
pub async fn post_retention_policies(configuration: &configuration::Configuration, params: PostRetentionPoliciesParams) -> Result<crate::models::RetentionPolicy, Error<PostRetentionPoliciesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let post_retention_policies_request = params.post_retention_policies_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/retention_policies", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_retention_policies_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostRetentionPoliciesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a retention policy.
pub async fn put_retention_policies_id(configuration: &configuration::Configuration, params: PutRetentionPoliciesIdParams) -> Result<crate::models::RetentionPolicy, Error<PutRetentionPoliciesIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let retention_policy_id = params.retention_policy_id;
    let put_retention_policies_id_request = params.put_retention_policies_id_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/retention_policies/{retention_policy_id}", local_var_configuration.base_path, retention_policy_id=crate::apis::urlencode(retention_policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_retention_policies_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutRetentionPoliciesIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

