/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignRequestAllOfSigningLog {
    /// The unique identifier that represent a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`.
    #[serde(rename = "id")]
    pub id: String,
    /// The HTTP `etag` of this file. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the file if (no) changes have happened.
    #[serde(rename = "etag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub etag: Option<Option<String>>,
    /// `file`
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "sequence_id", skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<Box<String>>,
    /// The name of the file
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The SHA1 hash of the file. This can be used to compare the contents of a file on Box with a local file.
    #[serde(rename = "sha1", skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(rename = "file_version", skip_serializing_if = "Option::is_none")]
    pub file_version: Option<Box<crate::models::FileMiniAllOfFileVersion>>,
}

impl SignRequestAllOfSigningLog {
    pub fn new(id: String, r#type: RHashType) -> SignRequestAllOfSigningLog {
        SignRequestAllOfSigningLog {
            id,
            etag: None,
            r#type,
            sequence_id: None,
            name: None,
            sha1: None,
            file_version: None,
        }
    }
}

/// `file`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "file")]
    File,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::File
    }
}

