# \WatermarksFoldersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_folders_id_watermark**](WatermarksFoldersApi.md#delete_folders_id_watermark) | **DELETE** /folders/{folder_id}/watermark | Remove watermark from folder
[**get_folders_id_watermark**](WatermarksFoldersApi.md#get_folders_id_watermark) | **GET** /folders/{folder_id}/watermark | Get watermark for folder
[**put_folders_id_watermark**](WatermarksFoldersApi.md#put_folders_id_watermark) | **PUT** /folders/{folder_id}/watermark | Apply watermark to folder



## delete_folders_id_watermark

> delete_folders_id_watermark(folder_id)
Remove watermark from folder

Removes the watermark from a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders_id_watermark

> crate::models::Watermark get_folders_id_watermark(folder_id)
Get watermark for folder

Retrieve the watermark for a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |

### Return type

[**crate::models::Watermark**](Watermark.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_folders_id_watermark

> crate::models::Watermark put_folders_id_watermark(folder_id, put_folders_id_watermark_request)
Apply watermark to folder

Applies or update a watermark on a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**put_folders_id_watermark_request** | Option<[**PutFoldersIdWatermarkRequest**](PutFoldersIdWatermarkRequest.md)> |  |  |

### Return type

[**crate::models::Watermark**](Watermark.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

