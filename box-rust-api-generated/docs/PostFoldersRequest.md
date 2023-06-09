# PostFoldersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name for the new folder.  There are some restrictions to the file name. Names containing non-printable ASCII characters, forward and backward slashes (`/`, `\\`), as well as names with trailing spaces are prohibited.  Additionally, the names `.` and `..` are not allowed either. | 
**parent** | [**crate::models::PostFoldersRequestParent**](post_folders_request_parent.md) |  | 
**folder_upload_email** | Option<[**crate::models::PostFoldersRequestFolderUploadEmail**](post_folders_request_folder_upload_email.md)> |  | [optional]
**sync_state** | Option<**String**> | Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


