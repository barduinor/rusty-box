# FolderAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The date and time when the folder was created. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**modified_at** | Option<**String**> | The date and time when the folder was last updated. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**description** | Option<**String**> |  | [optional]
**size** | Option<**i64**> | The folder size in bytes.  Be careful parsing this integer as its value can get very large. | [optional]
**path_collection** | Option<[**crate::models::FolderAllOfPathCollection**](Folder_allOf_path_collection.md)> |  | [optional]
**created_by** | Option<[**crate::models::FolderAllOfCreatedBy**](Folder_allOf_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::FolderAllOfModifiedBy**](Folder_allOf_modified_by.md)> |  | [optional]
**trashed_at** | Option<**String**> | The time at which this folder was put in the trash. | [optional]
**purged_at** | Option<**String**> | The time at which this folder is expected to be purged from the trash. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this folder was originally created. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this folder was last updated. | [optional]
**owned_by** | Option<[**crate::models::FolderAllOfOwnedBy**](Folder_allOf_owned_by.md)> |  | [optional]
**shared_link** | Option<[**crate::models::FolderAllOfSharedLink**](Folder_allOf_shared_link.md)> |  | [optional]
**folder_upload_email** | Option<[**crate::models::FolderAllOfFolderUploadEmail**](Folder_allOf_folder_upload_email.md)> |  | [optional]
**parent** | Option<[**crate::models::FolderAllOfParent**](Folder_allOf_parent.md)> |  | [optional]
**item_status** | Option<**String**> | Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted. | [optional]
**item_collection** | Option<[**crate::models::FolderAllOfItemCollection**](Folder_allOf_item_collection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


