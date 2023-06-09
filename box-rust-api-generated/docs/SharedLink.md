# SharedLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The URL that can be used to access the item on Box.  This URL will display the item in Box's preview UI where the file can be downloaded if allowed.  This URL will continue to work even when a custom `vanity_url` has been set for this shared link. | 
**download_url** | Option<**String**> | A URL that can be used to download the file. This URL can be used in a browser to download the file. This URL includes the file extension so that the file will be saved with the right file type.  This property will be `null` for folders. | [optional]
**vanity_url** | Option<**String**> | The \"Custom URL\" that can also be used to preview the item on Box.  Custom URLs can only be created or modified in the Box Web application. | [optional]
**vanity_name** | Option<**String**> | The custom name of a shared link, as used in the `vanity_url` field. | [optional]
**access** | Option<**String**> | The access level for this shared link.  * `open` - provides access to this item to anyone with this link * `company` - only provides access to this item to people the same company * `collaborators` - only provides access to this item to people who are    collaborators on this item  If this field is omitted when creating the shared link, the access level will be set to the default access level specified by the enterprise admin. | [optional]
**effective_access** | **String** | The effective access level for the shared link. This can be a more restrictive access level than the value in the `access` field when the enterprise settings restrict the allowed access levels. | 
**effective_permission** | **String** | The effective permissions for this shared link. These result in the more restrictive combination of the share link permissions and the item permissions set by the administrator, the owner, and any ancestor item such as a folder. | 
**unshared_at** | Option<**String**> | The date and time when this link will be unshared. This field can only be set by users with paid accounts. | [optional]
**is_password_enabled** | **bool** | Defines if the shared link requires a password to access the item. | 
**permissions** | Option<[**crate::models::SharedLinkPermissions**](Shared_link_permissions.md)> |  | [optional]
**download_count** | **i32** | The number of times this item has been downloaded. | 
**preview_count** | **i32** | The number of times this item has been previewed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


