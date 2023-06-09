# FileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this file request. | [optional][readonly]
**r#type** | Option<**String**> | `file_request` | [optional][readonly]
**title** | Option<**String**> | The title of file request. This is shown in the Box UI to users uploading files.  This defaults to title of the file request that was copied to create this file request. | [optional]
**description** | Option<**String**> | The optional description of this file request. This is shown in the Box UI to users uploading files.  This defaults to description of the file request that was copied to create this file request. | [optional]
**status** | Option<**String**> | The status of the file request. This defaults to `active`.  When the status is set to `inactive`, the file request will no longer accept new submissions, and any visitor to the file request URL will receive a `HTTP 404` status code.  This defaults to status of file request that was copied to create this file request. | [optional]
**is_email_required** | Option<**bool**> | Whether a file request submitter is required to provide their email address.  When this setting is set to true, the Box UI will show an email field on the file request form.  This defaults to setting of file request that was copied to create this file request. | [optional]
**is_description_required** | Option<**bool**> | Whether a file request submitter is required to provide a description of the files they are submitting.  When this setting is set to true, the Box UI will show a description field on the file request form.  This defaults to setting of file request that was copied to create this file request. | [optional]
**expires_at** | Option<**String**> | The date after which a file request will no longer accept new submissions.  After this date, the `status` will automatically be set to `inactive`. | [optional]
**folder** | [**crate::models::FileRequestFolder**](FileRequest_folder.md) |  | 
**url** | Option<**String**> | The generated URL for this file request. This URL can be shared with users to let them upload files to the associated folder. | [optional][readonly]
**etag** | Option<**String**> | The HTTP `etag` of this file. This can be used in combination with the `If-Match` header when updating a file request. By providing that header, a change will only be performed on the  file request if the `etag` on the file request still matches the `etag` provided in the `If-Match` header. | [optional]
**created_by** | Option<[**crate::models::FileRequestCreatedBy**](FileRequest_created_by.md)> |  | [optional]
**created_at** | **String** | The date and time when the file request was created. | 
**updated_by** | Option<[**crate::models::FileRequestUpdatedBy**](FileRequest_updated_by.md)> |  | [optional]
**updated_at** | **String** | The date and time when the file request was last updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


