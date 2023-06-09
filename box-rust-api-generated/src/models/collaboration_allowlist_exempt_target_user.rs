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
pub struct CollaborationAllowlistExemptTargetUser {
    /// The unique identifier for this enterprise.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `enterprise`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// The name of the enterprise
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CollaborationAllowlistExemptTargetUser {
    pub fn new() -> CollaborationAllowlistExemptTargetUser {
        CollaborationAllowlistExemptTargetUser {
            id: None,
            r#type: None,
            name: None,
        }
    }
}

/// `enterprise`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "enterprise")]
    Enterprise,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Enterprise
    }
}

