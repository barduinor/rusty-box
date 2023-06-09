# FolderFullAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sync_state** | Option<**String**> | Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive. | [optional]
**has_collaborations** | Option<**bool**> | Specifies if this folder has any other collaborators. | [optional]
**permissions** | Option<[**crate::models::FolderFullAllOfPermissions**](Folder__Full_allOf_permissions.md)> |  | [optional]
**tags** | Option<[**crate::models::Array**](array.md)> |  | [optional]
**can_non_owners_invite** | Option<**bool**> |  | [optional]
**is_externally_owned** | Option<**bool**> | Specifies if this folder is owned by a user outside of the authenticated enterprise. | [optional]
**metadata** | Option<[**crate::models::Map**](map.md)> |  | [optional]
**is_collaboration_restricted_to_enterprise** | Option<**bool**> |  | [optional]
**allowed_shared_link_access_levels** | Option<**Vec<String>**> | A list of access levels that are available for this folder.  For some folders, like the root folder, this will always be an empty list as sharing is not allowed at that level. | [optional]
**allowed_invitee_roles** | Option<**Vec<String>**> | A list of the types of roles that user can be invited at when sharing this folder. | [optional]
**watermark_info** | Option<[**crate::models::FolderFullAllOfWatermarkInfo**](Folder__Full_allOf_watermark_info.md)> |  | [optional]
**is_accessible_via_shared_link** | Option<**bool**> | Specifies if the folder can be accessed with the direct shared link or a shared link to a parent folder. | [optional]
**can_non_owners_view_collaborators** | Option<**bool**> | Specifies if collaborators who are not owners of this folder are restricted from viewing other collaborations on this folder.  It also restricts non-owners from inviting new collaborators. | [optional]
**classification** | Option<[**crate::models::FolderFullAllOfClassification**](Folder__Full_allOf_classification.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


