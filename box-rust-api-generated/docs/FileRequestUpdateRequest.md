# FileRequestUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | An optional new title for the file request. This can be used to change the title of the file request.  This will default to the value on the existing file request. | [optional]
**description** | Option<**String**> | An optional new description for the file request. This can be used to change the description of the file request.  This will default to the value on the existing file request. | [optional]
**status** | Option<**String**> | An optional new status of the file request.  When the status is set to `inactive`, the file request will no longer accept new submissions, and any visitor to the file request URL will receive a `HTTP 404` status code.  This will default to the value on the existing file request. | [optional]
**is_email_required** | Option<**bool**> | Whether a file request submitter is required to provide their email address.  When this setting is set to true, the Box UI will show an email field on the file request form.  This will default to the value on the existing file request. | [optional]
**is_description_required** | Option<**bool**> | Whether a file request submitter is required to provide a description of the files they are submitting.  When this setting is set to true, the Box UI will show a description field on the file request form.  This will default to the value on the existing file request. | [optional]
**expires_at** | Option<**String**> | The date after which a file request will no longer accept new submissions.  After this date, the `status` will automatically be set to `inactive`.  This will default to the value on the existing file request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


