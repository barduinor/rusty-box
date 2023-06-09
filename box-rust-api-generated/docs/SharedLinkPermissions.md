# SharedLinkPermissions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_download** | **bool** | Defines if the shared link allows for the item to be downloaded. For shared links on folders, this also applies to any items in the folder.  This value can be set to `true` when the effective access level is set to `open` or `company`, not `collaborators`. | 
**can_preview** | **bool** | Defines if the shared link allows for the item to be previewed.  This value is always `true`. For shared links on folders this also applies to any items in the folder. | 
**can_edit** | **bool** | Defines if the shared link allows for the item to be edited.  This value can only be `true` if `can_download` is also `true` and if the item has a type of `file`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


