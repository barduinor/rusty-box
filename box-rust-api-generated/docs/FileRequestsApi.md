# \FileRequestsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_file_requests_id**](FileRequestsApi.md#delete_file_requests_id) | **DELETE** /file_requests/{file_request_id} | Delete file request
[**get_file_requests_id**](FileRequestsApi.md#get_file_requests_id) | **GET** /file_requests/{file_request_id} | Get file request
[**post_file_requests_id_copy**](FileRequestsApi.md#post_file_requests_id_copy) | **POST** /file_requests/{file_request_id}/copy | Copy file request
[**put_file_requests_id**](FileRequestsApi.md#put_file_requests_id) | **PUT** /file_requests/{file_request_id} | Update file request



## delete_file_requests_id

> delete_file_requests_id(file_request_id)
Delete file request

Deletes a file request permanently.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_request_id** | **String** | The unique identifier that represent a file request.  The ID for any file request can be determined by visiting a file request builder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/filerequest/123` the `file_request_id` is `123`. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_requests_id

> crate::models::FileRequest get_file_requests_id(file_request_id)
Get file request

Retrieves the information about a file request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_request_id** | **String** | The unique identifier that represent a file request.  The ID for any file request can be determined by visiting a file request builder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/filerequest/123` the `file_request_id` is `123`. | [required] |

### Return type

[**crate::models::FileRequest**](FileRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_file_requests_id_copy

> crate::models::FileRequest post_file_requests_id_copy(file_request_id, file_request_copy_request)
Copy file request

Copies an existing file request that is already present on one folder, and applies it to another folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_request_id** | **String** | The unique identifier that represent a file request.  The ID for any file request can be determined by visiting a file request builder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/filerequest/123` the `file_request_id` is `123`. | [required] |
**file_request_copy_request** | Option<[**FileRequestCopyRequest**](FileRequestCopyRequest.md)> |  |  |

### Return type

[**crate::models::FileRequest**](FileRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_file_requests_id

> crate::models::FileRequest put_file_requests_id(file_request_id, if_match, file_request_update_request)
Update file request

Updates a file request. This can be used to activate or deactivate a file request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_request_id** | **String** | The unique identifier that represent a file request.  The ID for any file request can be determined by visiting a file request builder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/filerequest/123` the `file_request_id` is `123`. | [required] |
**if_match** | Option<**String**> | Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since. |  |
**file_request_update_request** | Option<[**FileRequestUpdateRequest**](FileRequestUpdateRequest.md)> |  |  |

### Return type

[**crate::models::FileRequest**](FileRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

