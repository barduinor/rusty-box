# ZipDownloadStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_file_count** | Option<**i32**> | The total number of files in the archive. | [optional]
**downloaded_file_count** | Option<**i32**> | The number of files that have already been downloaded. | [optional]
**skipped_file_count** | Option<**i32**> | The number of files that have been skipped as they could not be downloaded. In many cases this is due to permission issues that have surfaced between the creation of the request for the archive and the archive being downloaded. | [optional]
**skipped_folder_count** | Option<**i32**> | The number of folders that have been skipped as they could not be downloaded. In many cases this is due to permission issues that have surfaced between the creation of the request for the archive and the archive being downloaded. | [optional]
**state** | Option<**String**> | The state of the archive being downloaded. | [optional][default to InProgress]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


