# PostFilesIdCopyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | An optional new name for the copied file.  There are some restrictions to the file name. Names containing non-printable ASCII characters, forward and backward slashes (`/`, `\\`), and protected names like `.` and `..` are automatically sanitized by removing the non-allowed characters. | [optional]
**version** | Option<**String**> | An optional ID of the specific file version to copy. | [optional]
**parent** | [**crate::models::PostFilesIdCopyRequestParent**](post_files_id_copy_request_parent.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


