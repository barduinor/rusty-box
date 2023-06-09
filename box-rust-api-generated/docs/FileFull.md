# FileFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier that represent a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | 
**etag** | Option<**String**> | The HTTP `etag` of this file. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the file if (no) changes have happened. | [optional]
**r#type** | **String** | `file` | 
**sequence_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> | The name of the file | [optional]
**sha1** | Option<**String**> | The SHA1 hash of the file. This can be used to compare the contents of a file on Box with a local file. | [optional]
**file_version** | Option<[**crate::models::FileMiniAllOfFileVersion**](File__Mini_allOf_file_version.md)> |  | [optional]
**description** | Option<**String**> | The optional description of this file | [optional]
**size** | Option<**i32**> | The file size in bytes. Be careful parsing this integer as it can get very large and cause an integer overflow. | [optional]
**path_collection** | Option<[**crate::models::FileAllOfPathCollection**](File_allOf_path_collection.md)> |  | [optional]
**created_at** | Option<**String**> | The date and time when the file was created on Box. | [optional]
**modified_at** | Option<**String**> | The date and time when the file was last updated on Box. | [optional]
**trashed_at** | Option<**String**> | The time at which this file was put in the trash. | [optional]
**purged_at** | Option<**String**> | The time at which this file is expected to be purged from the trash. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this file was originally created, which might be before it was uploaded to Box. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this file was last updated, which might be before it was uploaded to Box. | [optional]
**created_by** | Option<[**crate::models::FileAllOfCreatedBy**](File_allOf_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::FileAllOfModifiedBy**](File_allOf_modified_by.md)> |  | [optional]
**owned_by** | Option<[**crate::models::FileAllOfOwnedBy**](File_allOf_owned_by.md)> |  | [optional]
**shared_link** | Option<[**crate::models::FileAllOfSharedLink**](File_allOf_shared_link.md)> |  | [optional]
**parent** | Option<[**crate::models::FileAllOfParent**](File_allOf_parent.md)> |  | [optional]
**item_status** | Option<**String**> | Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted. | [optional]
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


