# WebLinkAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent** | Option<[**crate::models::TrashWebLinkParent**](TrashWebLink_parent.md)> |  | [optional]
**description** | Option<**String**> | The description accompanying the web link. This is visible within the Box web application. | [optional]
**path_collection** | Option<[**crate::models::TrashWebLinkRestoredPathCollection**](TrashWebLinkRestored_path_collection.md)> |  | [optional]
**created_at** | Option<**String**> | When this file was created on Box’s servers. | [optional]
**modified_at** | Option<**String**> | When this file was last updated on the Box servers. | [optional]
**trashed_at** | Option<**String**> | When this file was moved to the trash. | [optional]
**purged_at** | Option<**String**> | When this file will be permanently deleted. | [optional]
**created_by** | Option<[**crate::models::TrashWebLinkCreatedBy**](TrashWebLink_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::TrashWebLinkModifiedBy**](TrashWebLink_modified_by.md)> |  | [optional]
**owned_by** | Option<[**crate::models::TrashWebLinkOwnedBy**](TrashWebLink_owned_by.md)> |  | [optional]
**shared_link** | Option<[**crate::models::WebLinkAllOfSharedLink**](WebLink_allOf_shared_link.md)> |  | [optional]
**item_status** | Option<**String**> | Whether this item is deleted or not. Values include `active`, `trashed` if the file has been moved to the trash, and `deleted` if the file has been permanently deleted | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


