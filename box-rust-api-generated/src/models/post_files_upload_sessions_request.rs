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
pub struct PostFilesUploadSessionsRequest {
    /// The ID of the folder to upload the new file to.
    #[serde(rename = "folder_id")]
    pub folder_id: String,
    /// The total number of bytes of the file to be uploaded
    #[serde(rename = "file_size")]
    pub file_size: i64,
    /// The name of new file
    #[serde(rename = "file_name")]
    pub file_name: String,
}

impl PostFilesUploadSessionsRequest {
    pub fn new(folder_id: String, file_size: i64, file_name: String) -> PostFilesUploadSessionsRequest {
        PostFilesUploadSessionsRequest {
            folder_id,
            file_size,
            file_name,
        }
    }
}


