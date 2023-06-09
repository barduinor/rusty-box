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

/// struct for passing parameters to the method [`get_files_id_content`]
#[derive(Clone, Debug, Default)]
pub struct GetFilesIdContentParams {
    /// The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`.
    pub file_id: String,
    /// The byte range of the content to download.  The format `bytes={start_byte}-{end_byte}` can be used to specify what section of the file to download.
    pub range: Option<String>,
    /// The URL, and optional password, for the shared link of this item.  This header can be used to access items that have not been explicitly shared with a user.  Use the format `shared_link=[link]` or if a password is required then use `shared_link=[link]&shared_link_password=[password]`.  This header can be used on the file or folder shared, as well as on any files or folders nested within the item.
    pub boxapi: Option<String>,
    /// The file version to download
    pub version: Option<String>,
    /// An optional access token that can be used to pre-authenticate this request, which means that a download link can be shared with a browser or a third party service without them needing to know how to handle the authentication. When using this parameter, please make sure that the access token is sufficiently scoped down to only allow read access to that file and no other files or folders.
    pub access_token: Option<String>
}


/// struct for typed errors of method [`get_files_id_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFilesIdContentError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Returns the contents of a file in binary format.
pub async fn get_files_id_content(configuration: &configuration::Configuration, params: GetFilesIdContentParams) -> Result<(), Error<GetFilesIdContentError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let file_id = params.file_id;
    let range = params.range;
    let boxapi = params.boxapi;
    let version = params.version;
    let access_token = params.access_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{file_id}/content", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = version {
        local_var_req_builder = local_var_req_builder.query(&[("version", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = access_token {
        local_var_req_builder = local_var_req_builder.query(&[("access_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = range {
        local_var_req_builder = local_var_req_builder.header("range", local_var_param_value.to_string());
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
        Ok(())
    } else {
        let local_var_entity: Option<GetFilesIdContentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

