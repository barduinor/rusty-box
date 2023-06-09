# SearchResultWithSharedLinkItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for this web link | 
**etag** | Option<**String**> | The entity tag of this web link. Used with `If-Match` headers. | [optional]
**r#type** | **String** | `web_link` | 
**sequence_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> | The name of the web link | [optional]
**sha1** | Option<**String**> | The SHA1 hash of the file. This can be used to compare the contents of a file on Box with a local file. | [optional]
**file_version** | Option<[**crate::models::FileMiniAllOfFileVersion**](File__Mini_allOf_file_version.md)> |  | [optional]
**description** | Option<**String**> | The description accompanying the web link. This is visible within the Box web application. | [optional]
**size** | Option<**i64**> | The folder size in bytes.  Be careful parsing this integer as its value can get very large. | [optional]
**path_collection** | Option<[**crate::models::TrashWebLinkRestoredPathCollection**](TrashWebLinkRestored_path_collection.md)> |  | [optional]
**created_at** | Option<**String**> | When this file was created on Box’s servers. | [optional]
**modified_at** | Option<**String**> | When this file was last updated on the Box servers. | [optional]
**trashed_at** | Option<**String**> | When this file was moved to the trash. | [optional]
**purged_at** | Option<**String**> | When this file will be permanently deleted. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this folder was originally created. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this folder was last updated. | [optional]
**created_by** | Option<[**crate::models::TrashWebLinkCreatedBy**](TrashWebLink_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::TrashWebLinkModifiedBy**](TrashWebLink_modified_by.md)> |  | [optional]
**owned_by** | Option<[**crate::models::TrashWebLinkOwnedBy**](TrashWebLink_owned_by.md)> |  | [optional]
**shared_link** | Option<[**crate::models::WebLinkAllOfSharedLink**](WebLink_allOf_shared_link.md)> |  | [optional]
**parent** | Option<[**crate::models::TrashWebLinkParent**](TrashWebLink_parent.md)> |  | [optional]
**item_status** | Option<**String**> | Whether this item is deleted or not. Values include `active`, `trashed` if the file has been moved to the trash, and `deleted` if the file has been permanently deleted | [optional]
**folder_upload_email** | Option<[**crate::models::FolderAllOfFolderUploadEmail**](Folder_allOf_folder_upload_email.md)> |  | [optional]
**item_collection** | Option<[**crate::models::FolderAllOfItemCollection**](Folder_allOf_item_collection.md)> |  | [optional]
**url** | Option<**String**> | The URL this web link points to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


