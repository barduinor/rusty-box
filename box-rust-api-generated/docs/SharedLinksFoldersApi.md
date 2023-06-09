# \SharedLinksFoldersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_folders_id_get_shared_link**](SharedLinksFoldersApi.md#get_folders_id_get_shared_link) | **GET** /folders/{folder_id}#get_shared_link | Get shared link for folder
[**get_shared_items_folders**](SharedLinksFoldersApi.md#get_shared_items_folders) | **GET** /shared_items#folders | Find folder for shared link
[**put_folders_id_add_shared_link**](SharedLinksFoldersApi.md#put_folders_id_add_shared_link) | **PUT** /folders/{folder_id}#add_shared_link | Add shared link to folder
[**put_folders_id_remove_shared_link**](SharedLinksFoldersApi.md#put_folders_id_remove_shared_link) | **PUT** /folders/{folder_id}#remove_shared_link | Remove shared link from folder
[**put_folders_id_update_shared_link**](SharedLinksFoldersApi.md#put_folders_id_update_shared_link) | **PUT** /folders/{folder_id}#update_shared_link | Update shared link on folder



## get_folders_id_get_shared_link

> crate::models::FolderFull get_folders_id_get_shared_link(folder_id, fields)
Get shared link for folder

Gets the information for a shared link on a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shared_items_folders

> crate::models::FolderFull get_shared_items_folders(boxapi, if_none_match, fields)
Find folder for shared link

Return the folder represented by a shared link.  A shared folder can be represented by a shared link, which can originate within the current enterprise or within another.  This endpoint allows an application to retrieve information about a shared folder when only given a shared link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**boxapi** | **String** | A header containing the shared link and optional password for the shared link.  The format for this header is as follows.  `shared_link=[link]&shared_link_password=[password]` | [required] |
**if_none_match** | Option<**String**> | Ensures an item is only returned if it has changed.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `304 Not Modified` if the item has not changed since. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_folders_id_add_shared_link

> crate::models::FolderFull put_folders_id_add_shared_link(folder_id, fields, put_folders_id_add_shared_link_request)
Add shared link to folder

Adds a shared link to a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_folders_id_add_shared_link_request** | Option<[**PutFoldersIdAddSharedLinkRequest**](PutFoldersIdAddSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_folders_id_remove_shared_link

> crate::models::FolderFull put_folders_id_remove_shared_link(folder_id, fields, put_folders_id_remove_shared_link_request)
Remove shared link from folder

Removes a shared link from a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_folders_id_remove_shared_link_request** | Option<[**PutFoldersIdRemoveSharedLinkRequest**](PutFoldersIdRemoveSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_folders_id_update_shared_link

> crate::models::FolderFull put_folders_id_update_shared_link(folder_id, fields, put_folders_id_update_shared_link_request)
Update shared link on folder

Updates a shared link on a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_folders_id_update_shared_link_request** | Option<[**PutFoldersIdUpdateSharedLinkRequest**](PutFoldersIdUpdateSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

