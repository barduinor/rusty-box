/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// SessionEndpoints : A list of endpoints for a chunked upload session.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SessionEndpoints {
    /// The URL to upload parts to
    #[serde(rename = "upload_part", skip_serializing_if = "Option::is_none")]
    pub upload_part: Option<String>,
    /// The URL used to commit the file
    #[serde(rename = "commit", skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    /// The URL for used to abort the session.
    #[serde(rename = "abort", skip_serializing_if = "Option::is_none")]
    pub abort: Option<String>,
    /// The URL users to list all parts.
    #[serde(rename = "list_parts", skip_serializing_if = "Option::is_none")]
    pub list_parts: Option<String>,
    /// The URL used to get the status of the upload.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The URL used to get the upload log from.
    #[serde(rename = "log_event", skip_serializing_if = "Option::is_none")]
    pub log_event: Option<String>,
}

impl SessionEndpoints {
    /// A list of endpoints for a chunked upload session.
    pub fn new() -> SessionEndpoints {
        SessionEndpoints {
            upload_part: None,
            commit: None,
            abort: None,
            list_parts: None,
            status: None,
            log_event: None,
        }
    }
}


