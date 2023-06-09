# ZipDownload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**download_url** | Option<**String**> | The URL that can be used to download the `zip` archive. A `Get` request to this URL will start streaming the items requested. By default, this URL is only valid for a few seconds, until the `expires_at` time, unless a download is started after which it is valid for the duration of the download.  It is important to note that the domain and path of this URL might change between API calls, and therefore it's important to use this URL as-is. | [optional]
**status_url** | Option<**String**> | The URL that can be used to get the status of the `zip` archive being downloaded. A `Get` request to this URL will return the number of files in the archive as well as the number of items already downloaded or skipped. By default, this URL is only valid for a few seconds, until the `expires_at` time, unless a download is started after which the URL is valid for 12 hours from the start of the download.  It is important to note that the domain and path of this URL might change between API calls, and therefore it's important to use this URL as-is. | [optional]
**expires_at** | Option<**String**> | The time and date when this archive will expire. After this time the `status_url` and `download_url` will return an error.  By default, these URLs are only valid for a few seconds, unless a download is started after which the `download_url` is valid for the duration of the download, and the `status_url` is valid for 12 hours from the start of the download. | [optional]
**name_conflicts** | Option<[**Vec<Vec<crate::models::ZipDownloadNameConflictsInnerInner>>**](array.md)> | A list of conflicts that occurred when trying to create the archive. This would occur when multiple items have been requested with the same name.  To solve these conflicts, the API will automatically rename an item and return a mapping between the original item's name and its new name.  For every conflict, both files will be renamed and therefore this list will always be a multiple of 2. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


