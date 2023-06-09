/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// Group : A standard representation of a group, as returned from any group API endpoints by default



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Group {
    /// The unique identifier for this object
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `group`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// The name of the group
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the group.
    #[serde(rename = "group_type", skip_serializing_if = "Option::is_none")]
    pub group_type: Option<GroupType>,
    /// When the group object was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the group object was last modified
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl Group {
    /// A standard representation of a group, as returned from any group API endpoints by default
    pub fn new() -> Group {
        Group {
            id: None,
            r#type: None,
            name: None,
            group_type: None,
            created_at: None,
            modified_at: None,
        }
    }
}

/// `group`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "group")]
    Group,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Group
    }
}
/// The type of the group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupType {
    #[serde(rename = "managed_group")]
    ManagedGroup,
    #[serde(rename = "all_users_group")]
    AllUsersGroup,
}

impl Default for GroupType {
    fn default() -> GroupType {
        Self::ManagedGroup
    }
}

