# PutFoldersIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The optional new name for this folder. | [optional]
**description** | Option<**String**> | The optional description of this folder | [optional]
**sync_state** | Option<**String**> | Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive. | [optional]
**can_non_owners_invite** | Option<**bool**> | Specifies if users who are not the owner of the folder can invite new collaborators to the folder. | [optional]
**parent** | Option<[**crate::models::PutFoldersIdRequestParent**](put_folders_id_request_parent.md)> |  | [optional]
**shared_link** | Option<[**crate::models::PutFoldersIdRequestSharedLink**](put_folders_id_request_shared_link.md)> |  | [optional]
**folder_upload_email** | Option<[**crate::models::PutFoldersIdRequestFolderUploadEmail**](put_folders_id_request_folder_upload_email.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | The tags for this item. These tags are shown in the Box web app and mobile apps next to an item.  To add or remove a tag, retrieve the item's current tags, modify them, and then update this field.  There is a limit of 100 tags per item, and 10,000 unique tags per enterprise. | [optional]
**is_collaboration_restricted_to_enterprise** | Option<**bool**> | Specifies if new invites to this folder are restricted to users within the enterprise. This does not affect existing collaborations. | [optional]
**collections** | Option<[**Vec<crate::models::Reference>**](Reference.md)> | An array of collections to make this folder a member of. Currently we only support the `favorites` collection.  To get the ID for a collection, use the [List all collections][1] endpoint.  Passing an empty array `[]` or `null` will remove the folder from all collections.  [1]: ../advanced-files-and-folders/#get-collections | [optional]
**can_non_owners_view_collaborators** | Option<**bool**> | Restricts collaborators who are not the owner of this folder from viewing other collaborations on this folder.  It also restricts non-owners from inviting new collaborators.  When setting this field to `false`, it is required to also set `can_non_owners_invite_collaborators` to `false` if it has not already been set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


