# SignRequestAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | object type | [optional]
**source_files** | Option<[**Vec<crate::models::FileBase>**](File--Base.md)> | List of files to create a signing document from. This is currently limited to ten files. Only the ID and type fields are required for each file. | [optional]
**signers** | Option<[**Vec<crate::models::SignRequestSigner>**](SignRequestSigner.md)> | Array of signers for the sign request | [optional]
**signature_color** | Option<**String**> | Force a specific color for the signature (blue, black, or red). | [optional]
**id** | Option<**String**> | Sign request ID | [optional]
**prepare_url** | Option<**String**> | This URL is returned if `is_document_preparation_needed` is set to `true` in the request. It is used to prepare the sign request via UI. The sign request is not sent until preparation is complete. | [optional]
**signing_log** | Option<[**crate::models::SignRequestAllOfSigningLog**](SignRequest_allOf_signing_log.md)> |  | [optional]
**status** | Option<**String**> | Describes the status of the sign request | [optional]
**sign_files** | Option<[**crate::models::SignRequestAllOfSignFiles**](SignRequest_allOf_sign_files.md)> |  | [optional]
**auto_expire_at** | Option<**String**> | Uses `days_valid` to calculate the date and time, in GMT, the sign request will expire if unsigned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


