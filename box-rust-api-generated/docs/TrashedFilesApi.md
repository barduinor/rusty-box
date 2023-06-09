# \TrashedFilesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_files_id_trash**](TrashedFilesApi.md#delete_files_id_trash) | **DELETE** /files/{file_id}/trash | Permanently remove file
[**get_files_id_trash**](TrashedFilesApi.md#get_files_id_trash) | **GET** /files/{file_id}/trash | Get trashed file
[**post_files_id**](TrashedFilesApi.md#post_files_id) | **POST** /files/{file_id} | Restore file



## delete_files_id_trash

> delete_files_id_trash(file_id)
Permanently remove file

Permanently deletes a file that is in the trash. This action cannot be undone.

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


## get_files_id_trash

> crate::models::TrashFile get_files_id_trash(file_id, fields)
Get trashed file

Retrieves a file that has been moved to the trash.  Please note that only if the file itself has been moved to the trash can it be retrieved with this API call. If instead one of its parent folders was moved to the trash, only that folder can be inspected using the [`GET /folders/:id/trash`](e://get_folders_id_trash) API.  To list all items that have been moved to the trash, please use the [`GET /folders/trash/items`](e://get-folders-trash-items/) API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::TrashFile**](TrashFile.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_id

> crate::models::TrashFileRestored post_files_id(file_id, fields, post_files_id_request)
Restore file

Restores a file that has been moved to the trash.  An optional new parent ID can be provided to restore the file to in case the original folder has been deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_files_id_request** | Option<[**PostFilesIdRequest**](PostFilesIdRequest.md)> |  |  |

### Return type

[**crate::models::TrashFileRestored**](TrashFileRestored.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

