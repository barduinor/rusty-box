# TrashFolderRestored

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting a folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folders/123` the `folder_id` is `123`. | [optional]
**etag** | Option<**String**> | The HTTP `etag` of this folder. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the folder if (no) changes have happened. | [optional]
**r#type** | Option<**String**> | `folder` | [optional]
**sequence_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> | The name of the folder. | [optional]
**created_at** | Option<**String**> | The date and time when the folder was created. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**modified_at** | Option<**String**> | The date and time when the folder was last updated. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**description** | Option<**String**> |  | [optional]
**size** | Option<**i64**> | The folder size in bytes.  Be careful parsing this integer as its value can get very large. | [optional]
**path_collection** | Option<[**crate::models::FileAllOfPathCollection**](File_allOf_path_collection.md)> |  | [optional]
**created_by** | Option<[**crate::models::FolderAllOfCreatedBy**](Folder_allOf_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::FolderAllOfModifiedBy**](Folder_allOf_modified_by.md)> |  | [optional]
**trashed_at** | Option<**String**> | The time at which this folder was put in the trash - becomes `null` after restore. | [optional]
**purged_at** | Option<**String**> | The time at which this folder is expected to be purged from the trash  - becomes `null` after restore. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this folder was originally created. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this folder was last updated. | [optional]
**owned_by** | Option<[**crate::models::FolderAllOfOwnedBy**](Folder_allOf_owned_by.md)> |  | [optional]
**shared_link** | Option<**String**> | The shared link for this file. This will be `null` if a folder had been trashed, even though the original shared link does become active again. | [optional]
**folder_upload_email** | Option<**String**> | The folder upload email for this folder. This will be `null` if a folder has been trashed, even though the original upload email does become active again. | [optional]
**parent** | Option<[**crate::models::TrashFolderParent**](TrashFolder_parent.md)> |  | [optional]
**item_status** | Option<**String**> | Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


