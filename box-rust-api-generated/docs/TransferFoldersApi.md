# \TransferFoldersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**put_users_id_folders0**](TransferFoldersApi.md#put_users_id_folders0) | **PUT** /users/{user_id}/folders/0 | Transfer owned folders



## put_users_id_folders0

> crate::models::FolderFull put_users_id_folders0(user_id, fields, notify, put_users_id_folders0_request)
Transfer owned folders

Move all of the items (files, folders and workflows) owned by a user into another user's account  Only the root folder (`0`) can be transferred.  Folders can only be moved across users by users with administrative permissions.  All existing shared links and folder-level collaborations are transferred during the operation. Please note that while collaborations at the individual file-level are transferred during the operation, the collaborations are deleted when the original user is deleted.  This call will be performed synchronously which might lead to a slow response when the source user has a large number of items in all of its folders.  If the destination path has a metadata cascade policy attached to any of the parent folders, a metadata cascade operation will be kicked off asynchronously.  There is currently no way to check for when this operation is finished.  The destination folder's name will be in the format `{User}'s Files and Folders`, where `{User}` is the display name of the user.  To make this API call your application will need to have the \"Read and write all files and folders stored in Box\" scope enabled.  Please make sure the destination user has access to `Relay` or `Relay Lite`, and has access to the files and folders involved in the workflows being transferred.  Admins will receive an email when the operation is completed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**notify** | Option<**bool**> | Determines if users should receive email notification for the action performed. |  |
**put_users_id_folders0_request** | Option<[**PutUsersIdFolders0Request**](PutUsersIdFolders0Request.md)> |  |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

