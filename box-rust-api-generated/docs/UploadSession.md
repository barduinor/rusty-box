# UploadSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this session | [optional]
**r#type** | Option<**String**> | `upload_session` | [optional]
**session_expires_at** | Option<**String**> | The date and time when this session expires. | [optional]
**part_size** | Option<**i64**> | The  size in bytes that must be used for all parts of of the upload.  Only the last part is allowed to be of a smaller size. | [optional]
**total_parts** | Option<**i32**> | The total number of parts expected in this upload session, as determined by the file size and part size. | [optional]
**num_parts_processed** | Option<**i32**> | The number of parts that have been uploaded and processed by the server. This starts at `0`.  When committing a file files, inspecting this property can provide insight if all parts have been uploaded correctly. | [optional]
**session_endpoints** | Option<[**crate::models::UploadSessionSessionEndpoints**](UploadSession_session_endpoints.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


