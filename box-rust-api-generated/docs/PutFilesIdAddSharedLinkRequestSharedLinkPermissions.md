# PutFilesIdAddSharedLinkRequestSharedLinkPermissions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_download** | Option<**bool**> | If the shared link allows for downloading of files. This can only be set when `access` is set to `open` or `company`. | [optional]
**can_preview** | Option<**bool**> | If the shared link allows for previewing of files. This value is always `true`. For shared links on folders this also applies to any items in the folder. | [optional]
**can_edit** | Option<**bool**> | If the shared link allows for editing of files. This can only be set when `access` is set to `open` or `company`. This value can only be `true` is `can_download` is also `true`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


