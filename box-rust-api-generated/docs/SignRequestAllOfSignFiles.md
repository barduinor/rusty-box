# SignRequestAllOfSignFiles

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**files** | Option<[**Vec<crate::models::FileMini>**](File--Mini.md)> |  | [optional]
**is_ready_for_download** | Option<**bool**> | Indicates whether the `sign_files` documents are processing and the PDFs may be out of date. A change to any document requires processing on all `sign_files`. We recommended waiting until processing is finished (and this value is true) before downloading the PDFs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


