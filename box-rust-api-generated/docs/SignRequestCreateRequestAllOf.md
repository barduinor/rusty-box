# SignRequestCreateRequestAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_files** | Option<[**Vec<crate::models::FileBase>**](File--Base.md)> | List of files to create a signing document from. This is currently limited to ten files. Only the ID and type fields are required for each file. | [optional]
**signers** | Option<[**Vec<crate::models::SignRequestCreateSigner>**](SignRequestCreateSigner.md)> | Array of signers for the sign request. 35 is the max number of signers permitted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


