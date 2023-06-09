# \SharedLinksFilesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_files_id_get_shared_link**](SharedLinksFilesApi.md#get_files_id_get_shared_link) | **GET** /files/{file_id}#get_shared_link | Get shared link for file
[**get_shared_items**](SharedLinksFilesApi.md#get_shared_items) | **GET** /shared_items | Find file for shared link
[**put_files_id_add_shared_link**](SharedLinksFilesApi.md#put_files_id_add_shared_link) | **PUT** /files/{file_id}#add_shared_link | Add shared link to file
[**put_files_id_remove_shared_link**](SharedLinksFilesApi.md#put_files_id_remove_shared_link) | **PUT** /files/{file_id}#remove_shared_link | Remove shared link from file
[**put_files_id_update_shared_link**](SharedLinksFilesApi.md#put_files_id_update_shared_link) | **PUT** /files/{file_id}#update_shared_link | Update shared link on file



## get_files_id_get_shared_link

> crate::models::FileFull get_files_id_get_shared_link(file_id, fields)
Get shared link for file

Gets the information for a shared link on a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |

### Return type

[**crate::models::FileFull**](File--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shared_items

> crate::models::FileFull get_shared_items(boxapi, if_none_match, fields)
Find file for shared link

Returns the file represented by a shared link.  A shared file can be represented by a shared link, which can originate within the current enterprise or within another.  This endpoint allows an application to retrieve information about a shared file when only given a shared link.  The `shared_link_permission_options` array field can be returned by requesting it in the `fields` query parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**boxapi** | **String** | A header containing the shared link and optional password for the shared link.  The format for this header is as follows.  `shared_link=[link]&shared_link_password=[password]` | [required] |
**if_none_match** | Option<**String**> | Ensures an item is only returned if it has changed.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `304 Not Modified` if the item has not changed since. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::FileFull**](File--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id_add_shared_link

> crate::models::FileFull put_files_id_add_shared_link(file_id, fields, put_files_id_add_shared_link_request)
Add shared link to file

Adds a shared link to a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_files_id_add_shared_link_request** | Option<[**PutFilesIdAddSharedLinkRequest**](PutFilesIdAddSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::FileFull**](File--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id_remove_shared_link

> crate::models::FileFull put_files_id_remove_shared_link(file_id, fields, put_files_id_remove_shared_link_request)
Remove shared link from file

Removes a shared link from a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_files_id_remove_shared_link_request** | Option<[**PutFilesIdRemoveSharedLinkRequest**](PutFilesIdRemoveSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::FileFull**](File--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id_update_shared_link

> crate::models::FileFull put_files_id_update_shared_link(file_id, fields, put_files_id_update_shared_link_request)
Update shared link on file

Updates a shared link on a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_files_id_update_shared_link_request** | Option<[**PutFilesIdUpdateSharedLinkRequest**](PutFilesIdUpdateSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::FileFull**](File--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

