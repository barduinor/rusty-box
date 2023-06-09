# FileFullAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version_number** | Option<**String**> | The version number of this file | [optional]
**comment_count** | Option<**i32**> | The number of comments on this file | [optional]
**permissions** | Option<[**crate::models::FileFullAllOfPermissions**](File__Full_allOf_permissions.md)> |  | [optional]
**tags** | Option<[**crate::models::Array**](array.md)> |  | [optional]
**lock** | Option<[**crate::models::FileFullAllOfLock**](File__Full_allOf_lock.md)> |  | [optional]
**extension** | Option<**String**> | Indicates the (optional) file extension for this file. By default, this is set to an empty string. | [optional]
**is_package** | Option<**bool**> | Indicates if the file is a package. Packages are commonly used by Mac Applications and can include iWork files. | [optional]
**expiring_embed_link** | Option<[**crate::models::FileFullAllOfExpiringEmbedLink**](File__Full_allOf_expiring_embed_link.md)> |  | [optional]
**watermark_info** | Option<[**crate::models::FileFullAllOfWatermarkInfo**](File__Full_allOf_watermark_info.md)> |  | [optional]
**is_accessible_via_shared_link** | Option<**bool**> | Specifies if the file can be accessed via the direct shared link or a shared link to a parent folder. | [optional]
**allowed_invitee_roles** | Option<**Vec<String>**> | A list of the types of roles that user can be invited at when sharing this file. | [optional]
**is_externally_owned** | Option<**bool**> | Specifies if this file is owned by a user outside of the authenticated enterprise. | [optional]
**has_collaborations** | Option<**bool**> | Specifies if this file has any other collaborators. | [optional]
**metadata** | Option<[**crate::models::Map**](map.md)> |  | [optional]
**expires_at** | Option<**String**> | When the file will automatically be deleted | [optional]
**representations** | Option<[**crate::models::FileFullAllOfRepresentations**](File__Full_allOf_representations.md)> |  | [optional]
**classification** | Option<[**crate::models::FileFullAllOfClassification**](File__Full_allOf_classification.md)> |  | [optional]
**uploader_display_name** | Option<**String**> | The display name of the user that uploaded the file. In most cases this is the name of the user logged in at the time of the upload.  If the file was uploaded using a File Request form that requires the user to provide an email address, this field is populated with that email address. If an email address was not required in the File Request form, this field is set to return a value of `File Request`.  In all other anonymous cases where no email was provided this field will default to a value of `Someone`. | [optional]
**disposition_at** | Option<**String**> | The retention expiration timestamp for the given file | [optional]
**shared_link_permission_options** | Option<**Vec<String>**> | A list of the types of roles that user can be invited at when sharing this file. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


