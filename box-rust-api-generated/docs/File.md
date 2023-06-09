# File

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier that represent a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | 
**etag** | Option<**String**> | The HTTP `etag` of this file. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the file if (no) changes have happened. | [optional]
**r#type** | **String** | `file` | 
**sequence_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> | The name of the file | [optional]
**sha1** | Option<**String**> | The SHA1 hash of the file. This can be used to compare the contents of a file on Box with a local file. | [optional]
**file_version** | Option<[**crate::models::FileMiniAllOfFileVersion**](File__Mini_allOf_file_version.md)> |  | [optional]
**description** | Option<**String**> | The optional description of this file | [optional]
**size** | Option<**i32**> | The file size in bytes. Be careful parsing this integer as it can get very large and cause an integer overflow. | [optional]
**path_collection** | Option<[**crate::models::FileAllOfPathCollection**](File_allOf_path_collection.md)> |  | [optional]
**created_at** | Option<**String**> | The date and time when the file was created on Box. | [optional]
**modified_at** | Option<**String**> | The date and time when the file was last updated on Box. | [optional]
**trashed_at** | Option<**String**> | The time at which this file was put in the trash. | [optional]
**purged_at** | Option<**String**> | The time at which this file is expected to be purged from the trash. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this file was originally created, which might be before it was uploaded to Box. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this file was last updated, which might be before it was uploaded to Box. | [optional]
**created_by** | Option<[**crate::models::FileAllOfCreatedBy**](File_allOf_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::FileAllOfModifiedBy**](File_allOf_modified_by.md)> |  | [optional]
**owned_by** | Option<[**crate::models::FileAllOfOwnedBy**](File_allOf_owned_by.md)> |  | [optional]
**shared_link** | Option<[**crate::models::FileAllOfSharedLink**](File_allOf_shared_link.md)> |  | [optional]
**parent** | Option<[**crate::models::FileAllOfParent**](File_allOf_parent.md)> |  | [optional]
**item_status** | Option<**String**> | Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


