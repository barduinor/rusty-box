# TrashWebLinkRestored

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | `web_link` | [optional]
**id** | Option<**String**> | The unique identifier for this web link | [optional]
**sequence_id** | **String** |  | 
**etag** | Option<**String**> | The entity tag of this web link. Used with `If-Match` headers. | [optional]
**name** | Option<**String**> | The name of the web link | [optional]
**url** | Option<**String**> | The URL this web link points to | [optional]
**parent** | Option<[**crate::models::TrashWebLinkParent**](TrashWebLink_parent.md)> |  | [optional]
**description** | Option<**String**> | The description accompanying the web link. This is visible within the Box web application. | [optional]
**path_collection** | [**crate::models::TrashWebLinkRestoredPathCollection**](TrashWebLinkRestored_path_collection.md) |  | 
**created_at** | Option<**String**> | When this file was created on Box’s servers. | [optional]
**modified_at** | Option<**String**> | When this file was last updated on the Box servers. | [optional]
**trashed_at** | Option<**String**> | The time at which this bookmark was put in the trash - becomes `null` after restore. | [optional]
**purged_at** | Option<**String**> | The time at which this bookmark will be permanently deleted - becomes `null` after restore. | [optional]
**created_by** | Option<[**crate::models::TrashWebLinkCreatedBy**](TrashWebLink_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::TrashWebLinkModifiedBy**](TrashWebLink_modified_by.md)> |  | [optional]
**owned_by** | Option<[**crate::models::TrashWebLinkOwnedBy**](TrashWebLink_owned_by.md)> |  | [optional]
**shared_link** | Option<**String**> | The shared link for this bookmark. This will be `null` if a bookmark had been trashed, even though the original shared link does become active again. | [optional]
**item_status** | Option<**String**> | Whether this item is deleted or not. Values include `active`, `trashed` if the file has been moved to the trash, and `deleted` if the file has been permanently deleted | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


