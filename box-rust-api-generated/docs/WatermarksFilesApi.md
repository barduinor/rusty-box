# \WatermarksFilesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_files_id_watermark**](WatermarksFilesApi.md#delete_files_id_watermark) | **DELETE** /files/{file_id}/watermark | Remove watermark from file
[**get_files_id_watermark**](WatermarksFilesApi.md#get_files_id_watermark) | **GET** /files/{file_id}/watermark | Get watermark on file
[**put_files_id_watermark**](WatermarksFilesApi.md#put_files_id_watermark) | **PUT** /files/{file_id}/watermark | Apply watermark to file



## delete_files_id_watermark

> delete_files_id_watermark(file_id)
Remove watermark from file

Removes the watermark from a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_watermark

> crate::models::Watermark get_files_id_watermark(file_id)
Get watermark on file

Retrieve the watermark for a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |

### Return type

[**crate::models::Watermark**](Watermark.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id_watermark

> crate::models::Watermark put_files_id_watermark(file_id, put_files_id_watermark_request)
Apply watermark to file

Applies or update a watermark on a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**put_files_id_watermark_request** | Option<[**PutFilesIdWatermarkRequest**](PutFilesIdWatermarkRequest.md)> |  |  |

### Return type

[**crate::models::Watermark**](Watermark.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

