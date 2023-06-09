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

/// struct for passing parameters to the method [`delete_metadata_cascade_policies_id`]
#[derive(Clone, Debug, Default)]
pub struct DeleteMetadataCascadePoliciesIdParams {
    /// The ID of the metadata cascade policy.
    pub metadata_cascade_policy_id: String
}

/// struct for passing parameters to the method [`get_metadata_cascade_policies`]
#[derive(Clone, Debug, Default)]
pub struct GetMetadataCascadePoliciesParams {
    /// Specifies which folder to return policies for. This can not be used on the root folder with ID `0`.
    pub folder_id: String,
    /// The ID of the enterprise ID for which to find metadata cascade policies. If not specified, it defaults to the current enterprise.
    pub owner_enterprise_id: Option<String>,
    /// Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`.
    pub marker: Option<String>,
    /// The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response.
    pub offset: Option<i64>
}

/// struct for passing parameters to the method [`get_metadata_cascade_policies_id`]
#[derive(Clone, Debug, Default)]
pub struct GetMetadataCascadePoliciesIdParams {
    /// The ID of the metadata cascade policy.
    pub metadata_cascade_policy_id: String
}

/// struct for passing parameters to the method [`post_metadata_cascade_policies`]
#[derive(Clone, Debug, Default)]
pub struct PostMetadataCascadePoliciesParams {
    pub post_metadata_cascade_policies_request: Option<crate::models::PostMetadataCascadePoliciesRequest>
}

/// struct for passing parameters to the method [`post_metadata_cascade_policies_id_apply`]
#[derive(Clone, Debug, Default)]
pub struct PostMetadataCascadePoliciesIdApplyParams {
    /// The ID of the cascade policy to force-apply.
    pub metadata_cascade_policy_id: String,
    pub post_metadata_cascade_policies_id_apply_request: Option<crate::models::PostMetadataCascadePoliciesIdApplyRequest>
}


/// struct for typed errors of method [`delete_metadata_cascade_policies_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMetadataCascadePoliciesIdError {
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_metadata_cascade_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetadataCascadePoliciesError {
    Status400(crate::models::ClientError),
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_metadata_cascade_policies_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetadataCascadePoliciesIdError {
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_metadata_cascade_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostMetadataCascadePoliciesError {
    Status400(crate::models::ClientError),
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status409(crate::models::ConflictError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_metadata_cascade_policies_id_apply`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostMetadataCascadePoliciesIdApplyError {
    Status404(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Deletes a metadata cascade policy.
pub async fn delete_metadata_cascade_policies_id(configuration: &configuration::Configuration, params: DeleteMetadataCascadePoliciesIdParams) -> Result<(), Error<DeleteMetadataCascadePoliciesIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let metadata_cascade_policy_id = params.metadata_cascade_policy_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/metadata_cascade_policies/{metadata_cascade_policy_id}", local_var_configuration.base_path, metadata_cascade_policy_id=crate::apis::urlencode(metadata_cascade_policy_id));
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
        let local_var_entity: Option<DeleteMetadataCascadePoliciesIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a list of all the metadata cascade policies that are applied to a given folder. This can not be used on the root folder with ID `0`.
pub async fn get_metadata_cascade_policies(configuration: &configuration::Configuration, params: GetMetadataCascadePoliciesParams) -> Result<crate::models::MetadataCascadePolicies, Error<GetMetadataCascadePoliciesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let folder_id = params.folder_id;
    let owner_enterprise_id = params.owner_enterprise_id;
    let marker = params.marker;
    let offset = params.offset;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/metadata_cascade_policies", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("folder_id", &folder_id.to_string())]);
    if let Some(ref local_var_str) = owner_enterprise_id {
        local_var_req_builder = local_var_req_builder.query(&[("owner_enterprise_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetMetadataCascadePoliciesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a specific metadata cascade policy assigned to a folder.
pub async fn get_metadata_cascade_policies_id(configuration: &configuration::Configuration, params: GetMetadataCascadePoliciesIdParams) -> Result<crate::models::MetadataCascadePolicy, Error<GetMetadataCascadePoliciesIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let metadata_cascade_policy_id = params.metadata_cascade_policy_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/metadata_cascade_policies/{metadata_cascade_policy_id}", local_var_configuration.base_path, metadata_cascade_policy_id=crate::apis::urlencode(metadata_cascade_policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetMetadataCascadePoliciesIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new metadata cascade policy that applies a given metadata template to a given folder and automatically cascades it down to any files within that folder.  In order for the policy to be applied a metadata instance must first be applied to the folder the policy is to be applied to.
pub async fn post_metadata_cascade_policies(configuration: &configuration::Configuration, params: PostMetadataCascadePoliciesParams) -> Result<crate::models::MetadataCascadePolicy, Error<PostMetadataCascadePoliciesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let post_metadata_cascade_policies_request = params.post_metadata_cascade_policies_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/metadata_cascade_policies", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_metadata_cascade_policies_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostMetadataCascadePoliciesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Force the metadata on a folder with a metadata cascade policy to be applied to all of its children. This can be used after creating a new cascade policy to enforce the metadata to be cascaded down to all existing files within that folder.
pub async fn post_metadata_cascade_policies_id_apply(configuration: &configuration::Configuration, params: PostMetadataCascadePoliciesIdApplyParams) -> Result<(), Error<PostMetadataCascadePoliciesIdApplyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let metadata_cascade_policy_id = params.metadata_cascade_policy_id;
    let post_metadata_cascade_policies_id_apply_request = params.post_metadata_cascade_policies_id_apply_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/metadata_cascade_policies/{metadata_cascade_policy_id}/apply", local_var_configuration.base_path, metadata_cascade_policy_id=crate::apis::urlencode(metadata_cascade_policy_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_metadata_cascade_policies_id_apply_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PostMetadataCascadePoliciesIdApplyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

