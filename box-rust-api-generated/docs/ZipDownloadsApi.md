# \ZipDownloadsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_zip_downloads_id_content**](ZipDownloadsApi.md#get_zip_downloads_id_content) | **GET** /zip_downloads/{zip_download_id}/content | Download zip archive
[**get_zip_downloads_id_status**](ZipDownloadsApi.md#get_zip_downloads_id_status) | **GET** /zip_downloads/{zip_download_id}/status | Get zip download status
[**post_zip_downloads**](ZipDownloadsApi.md#post_zip_downloads) | **POST** /zip_downloads | Create zip download



## get_zip_downloads_id_content

> std::path::PathBuf get_zip_downloads_id_content(zip_download_id)
Download zip archive

Returns the contents of a `zip` archive in binary format. This URL does not require any form of authentication and could be used in a user's browser to download the archive to a user's device.  By default, this URL is only valid for a few seconds from the creation of the request for this archive. Once a download has started it can not be stopped and resumed, instead a new request for a zip archive would need to be created.  The URL of this endpoint should not be considered as fixed. Instead, use the [Create zip download](e://post_zip_downloads) API to request to create a `zip` archive, and then follow the `download_url` field in the response to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zip_download_id** | **String** | The unique identifier that represent this `zip` archive. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zip_downloads_id_status

> crate::models::ZipDownloadStatus get_zip_downloads_id_status(zip_download_id)
Get zip download status

Returns the download status of a `zip` archive, allowing an application to inspect the progress of the download as well as the number of items that might have been skipped.  This endpoint can only be accessed once the download has started. Subsequently this endpoint is valid for 12 hours from the start of the download.  The URL of this endpoint should not be considered as fixed. Instead, use the [Create zip download](e://post_zip_downloads) API to request to create a `zip` archive, and then follow the `status_url` field in the response to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zip_download_id** | **String** | The unique identifier that represent this `zip` archive. | [required] |

### Return type

[**crate::models::ZipDownloadStatus**](ZipDownloadStatus.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_zip_downloads

> crate::models::ZipDownload post_zip_downloads(zip_download_request)
Create zip download

Creates a request to download multiple files and folders as a single `zip` archive file. This API does not return the archive but instead performs all the checks to ensure that the user has access to all the items, and then returns a `download_url` and a `status_url` that can be used to download the archive.  The limit for an archive is either the Account's upload limit or 10,000 files, whichever is met first

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zip_download_request** | Option<[**ZipDownloadRequest**](ZipDownloadRequest.md)> |  |  |

### Return type

[**crate::models::ZipDownload**](ZipDownload.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

