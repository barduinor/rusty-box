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

/// struct for passing parameters to the method [`delete_web_links_id`]
#[derive(Clone, Debug, Default)]
pub struct DeleteWebLinksIdParams {
    /// The ID of the web link.
    pub web_link_id: String
}

/// struct for passing parameters to the method [`get_web_links_id`]
#[derive(Clone, Debug, Default)]
pub struct GetWebLinksIdParams {
    /// The ID of the web link.
    pub web_link_id: String,
    /// The URL, and optional password, for the shared link of this item.  This header can be used to access items that have not been explicitly shared with a user.  Use the format `shared_link=[link]` or if a password is required then use `shared_link=[link]&shared_link_password=[password]`.  This header can be used on the file or folder shared, as well as on any files or folders nested within the item.
    pub boxapi: Option<String>
}

/// struct for passing parameters to the method [`post_web_links`]
#[derive(Clone, Debug, Default)]
pub struct PostWebLinksParams {
    pub post_web_links_request: Option<crate::models::PostWebLinksRequest>
}

/// struct for passing parameters to the method [`put_web_links_id`]
#[derive(Clone, Debug, Default)]
pub struct PutWebLinksIdParams {
    /// The ID of the web link.
    pub web_link_id: String,
    pub put_web_links_id_request: Option<crate::models::PutWebLinksIdRequest>
}


/// struct for typed errors of method [`delete_web_links_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebLinksIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_web_links_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebLinksIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_web_links`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostWebLinksError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_web_links_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutWebLinksIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Deletes a web link.
pub async fn delete_web_links_id(configuration: &configuration::Configuration, params: DeleteWebLinksIdParams) -> Result<(), Error<DeleteWebLinksIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let web_link_id = params.web_link_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/web_links/{web_link_id}", local_var_configuration.base_path, web_link_id=crate::apis::urlencode(web_link_id));
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
        let local_var_entity: Option<DeleteWebLinksIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve information about a web link.
pub async fn get_web_links_id(configuration: &configuration::Configuration, params: GetWebLinksIdParams) -> Result<crate::models::WebLink, Error<GetWebLinksIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let web_link_id = params.web_link_id;
    let boxapi = params.boxapi;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/web_links/{web_link_id}", local_var_configuration.base_path, web_link_id=crate::apis::urlencode(web_link_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = boxapi {
        local_var_req_builder = local_var_req_builder.header("boxapi", local_var_param_value.to_string());
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
        let local_var_entity: Option<GetWebLinksIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a web link object within a folder.
pub async fn post_web_links(configuration: &configuration::Configuration, params: PostWebLinksParams) -> Result<crate::models::WebLink, Error<PostWebLinksError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let post_web_links_request = params.post_web_links_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/web_links", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_web_links_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostWebLinksError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a web link object.
pub async fn put_web_links_id(configuration: &configuration::Configuration, params: PutWebLinksIdParams) -> Result<crate::models::WebLink, Error<PutWebLinksIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let web_link_id = params.web_link_id;
    let put_web_links_id_request = params.put_web_links_id_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/web_links/{web_link_id}", local_var_configuration.base_path, web_link_id=crate::apis::urlencode(web_link_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_web_links_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutWebLinksIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

