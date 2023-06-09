# FileVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier that represent a file version. | 
**r#type** | **String** | `file_version` | 
**sha1** | Option<**String**> | The SHA1 hash of this version of the file. | [optional]
**name** | Option<**String**> | The name of the file version | [optional]
**size** | Option<**i64**> | Size of the file version in bytes | [optional]
**created_at** | Option<**String**> | When the file version object was created | [optional]
**modified_at** | Option<**String**> | When the file version object was last updated | [optional]
**modified_by** | Option<[**crate::models::FileVersionAllOfModifiedBy**](FileVersion_allOf_modified_by.md)> |  | [optional]
**trashed_at** | Option<**String**> | When the file version object was trashed. | [optional]
**trashed_by** | Option<[**crate::models::FileVersionAllOfTrashedBy**](FileVersion_allOf_trashed_by.md)> |  | [optional]
**restored_at** | Option<**String**> | When the file version was restored from the trash. | [optional]
**restored_by** | Option<[**crate::models::FileVersionAllOfRestoredBy**](FileVersion_allOf_restored_by.md)> |  | [optional]
**purged_at** | Option<**String**> | When the file version object will be permanently deleted. | [optional]
**uploader_display_name** | Option<**String**> | The display name of the user that uploaded the file. In most cases this is the name of the user logged in at the time of the upload.  If the file was uploaded using a File Request form that requires the user to provide an email address, this field is populated with that email address. If an email address was not required in the File Request form, this field is set to return a value of `File Request`.  In all other anonymous cases where no email was provided this field will default to a value of `Someone`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


