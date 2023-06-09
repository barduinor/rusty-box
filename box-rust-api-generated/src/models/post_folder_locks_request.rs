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
pub struct PostFolderLocksRequest {
    #[serde(rename = "locked_operations", skip_serializing_if = "Option::is_none")]
    pub locked_operations: Option<Box<crate::models::PostFolderLocksRequestLockedOperations>>,
    #[serde(rename = "folder")]
    pub folder: Box<crate::models::PostFolderLocksRequestFolder>,
}

impl PostFolderLocksRequest {
    pub fn new(folder: crate::models::PostFolderLocksRequestFolder) -> PostFolderLocksRequest {
        PostFolderLocksRequest {
            locked_operations: None,
            folder: Box::new(folder),
        }
    }
}


