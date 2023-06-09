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
pub struct RecentItemItem {
    /// The unique identifier for this web link
    #[serde(rename = "id")]
    pub id: String,
    /// The entity tag of this web link. Used with `If-Match` headers.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// `web_link`
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "sequence_id", skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<Box<String>>,
    /// The name of the web link
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The SHA1 hash of the file. This can be used to compare the contents of a file on Box with a local file.
    #[serde(rename = "sha1", skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(rename = "file_version", skip_serializing_if = "Option::is_none")]
    pub file_version: Option<Box<crate::models::FileMiniAllOfFileVersion>>,
    /// The description accompanying the web link. This is visible within the Box web application.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The folder size in bytes.  Be careful parsing this integer as its value can get very large.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "path_collection", skip_serializing_if = "Option::is_none")]
    pub path_collection: Option<Box<crate::models::TrashWebLinkRestoredPathCollection>>,
    /// When this file was created on Box’s servers.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When this file was last updated on the Box servers.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// When this file was moved to the trash.
    #[serde(rename = "trashed_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trashed_at: Option<Option<String>>,
    /// When this file will be permanently deleted.
    #[serde(rename = "purged_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub purged_at: Option<Option<String>>,
    /// The date and time at which this folder was originally created.
    #[serde(rename = "content_created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_created_at: Option<Option<String>>,
    /// The date and time at which this folder was last updated.
    #[serde(rename = "content_modified_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_modified_at: Option<Option<String>>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::TrashWebLinkCreatedBy>>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::TrashWebLinkModifiedBy>>,
    #[serde(rename = "owned_by", skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<Box<crate::models::TrashWebLinkOwnedBy>>,
    #[serde(rename = "shared_link", skip_serializing_if = "Option::is_none")]
    pub shared_link: Option<Box<crate::models::WebLinkAllOfSharedLink>>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<crate::models::TrashWebLinkParent>>,
    /// Whether this item is deleted or not. Values include `active`, `trashed` if the file has been moved to the trash, and `deleted` if the file has been permanently deleted
    #[serde(rename = "item_status", skip_serializing_if = "Option::is_none")]
    pub item_status: Option<ItemStatus>,
    #[serde(rename = "folder_upload_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folder_upload_email: Option<Option<Box<crate::models::FolderAllOfFolderUploadEmail>>>,
    #[serde(rename = "item_collection", skip_serializing_if = "Option::is_none")]
    pub item_collection: Option<Box<crate::models::FolderAllOfItemCollection>>,
    /// The URL this web link points to
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl RecentItemItem {
    pub fn new(id: String, r#type: RHashType) -> RecentItemItem {
        RecentItemItem {
            id,
            etag: None,
            r#type,
            sequence_id: None,
            name: None,
            sha1: None,
            file_version: None,
            description: None,
            size: None,
            path_collection: None,
            created_at: None,
            modified_at: None,
            trashed_at: None,
            purged_at: None,
            content_created_at: None,
            content_modified_at: None,
            created_by: None,
            modified_by: None,
            owned_by: None,
            shared_link: None,
            parent: None,
            item_status: None,
            folder_upload_email: None,
            item_collection: None,
            url: None,
        }
    }
}

/// `web_link`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "web_link")]
    WebLink,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::WebLink
    }
}
/// Whether this item is deleted or not. Values include `active`, `trashed` if the file has been moved to the trash, and `deleted` if the file has been permanently deleted
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "trashed")]
    Trashed,
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for ItemStatus {
    fn default() -> ItemStatus {
        Self::Active
    }
}

