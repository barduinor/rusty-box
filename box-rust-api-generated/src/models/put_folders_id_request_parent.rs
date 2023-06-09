/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PutFoldersIdRequestParent : The parent folder for this folder. Use this to move the folder or to restore it out of the trash.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutFoldersIdRequestParent {
    /// The ID of the new parent folder
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl PutFoldersIdRequestParent {
    /// The parent folder for this folder. Use this to move the folder or to restore it out of the trash.
    pub fn new() -> PutFoldersIdRequestParent {
        PutFoldersIdRequestParent {
            id: None,
        }
    }
}


