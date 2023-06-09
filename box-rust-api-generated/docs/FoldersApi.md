# \FoldersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_folders_id**](FoldersApi.md#delete_folders_id) | **DELETE** /folders/{folder_id} | Delete folder
[**get_folders_id**](FoldersApi.md#get_folders_id) | **GET** /folders/{folder_id} | Get folder information
[**get_folders_id_items**](FoldersApi.md#get_folders_id_items) | **GET** /folders/{folder_id}/items | List items in folder
[**post_folders**](FoldersApi.md#post_folders) | **POST** /folders | Create folder
[**post_folders_id_copy**](FoldersApi.md#post_folders_id_copy) | **POST** /folders/{folder_id}/copy | Copy folder
[**put_folders_id**](FoldersApi.md#put_folders_id) | **PUT** /folders/{folder_id} | Update folder



## delete_folders_id

> delete_folders_id(folder_id, if_match, recursive)
Delete folder

Deletes a folder, either permanently or by moving it to the trash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**if_match** | Option<**String**> | Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since. |  |
**recursive** | Option<**bool**> | Delete a folder that is not empty by recursively deleting the folder and all of its content. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders_id

> crate::models::FolderFull get_folders_id(folder_id, fields, if_none_match, boxapi)
Get folder information

Retrieves details for a folder, including the first 100 entries in the folder.  To fetch more items within the folder, please use the [Get items in a folder](#get-folders-id-items) endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.  Additionally this field can be used to query any metadata applied to the file by specifying the `metadata` field as well as the scope and key of the template to retrieve, for example `?field=metadata.enterprise_12345.contractTemplate`. |  |
**if_none_match** | Option<**String**> | Ensures an item is only returned if it has changed.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `304 Not Modified` if the item has not changed since. |  |
**boxapi** | Option<**String**> | The URL, and optional password, for the shared link of this item.  This header can be used to access items that have not been explicitly shared with a user.  Use the format `shared_link=[link]` or if a password is required then use `shared_link=[link]&shared_link_password=[password]`.  This header can be used on the file or folder shared, as well as on any files or folders nested within the item. |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders_id_items

> crate::models::Items get_folders_id_items(folder_id, fields, usemarker, marker, offset, limit, boxapi, sort, direction)
List items in folder

Retrieves a page of items in a folder. These items can be files, folders, and web links.  To request more information about the folder itself, like its size, please use the [Get a folder](#get-folders-id) endpoint instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.  Additionally this field can be used to query any metadata applied to the file by specifying the `metadata` field as well as the scope and key of the template to retrieve, for example `?field=metadata.enterprise_12345.contractTemplate`. |  |
**usemarker** | Option<**bool**> | Specifies whether to use marker-based pagination instead of offset-based pagination. Only one pagination method can be used at a time.  By setting this value to true, the API will return a `marker` field that can be passed as a parameter to this endpoint to get the next page of the response. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**boxapi** | Option<**String**> | The URL, and optional password, for the shared link of this item.  This header can be used to access items that have not been explicitly shared with a user.  Use the format `shared_link=[link]` or if a password is required then use `shared_link=[link]&shared_link_password=[password]`.  This header can be used on the file or folder shared, as well as on any files or folders nested within the item. |  |
**sort** | Option<**String**> | Defines the **second** attribute by which items are sorted.  Items are always sorted by their `type` first, with folders listed before files, and files listed before web links.  This parameter is not supported for marker-based pagination on the root folder (the folder with an ID of `0`). |  |
**direction** | Option<**String**> | The direction to sort results in. This can be either in alphabetical ascending (`ASC`) or descending (`DESC`) order. |  |

### Return type

[**crate::models::Items**](Items.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_folders

> crate::models::FolderFull post_folders(fields, post_folders_request)
Create folder

Creates a new empty folder within the specified parent folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_folders_request** | Option<[**PostFoldersRequest**](PostFoldersRequest.md)> |  |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_folders_id_copy

> crate::models::FolderFull post_folders_id_copy(folder_id, fields, post_folders_id_copy_request)
Copy folder

Creates a copy of a folder within a destination folder.  The original folder will not be changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier of the folder to copy.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder with the ID `0` can not be copied. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_folders_id_copy_request** | Option<[**PostFoldersIdCopyRequest**](PostFoldersIdCopyRequest.md)> |  |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_folders_id

> crate::models::FolderFull put_folders_id(folder_id, fields, if_match, put_folders_id_request)
Update folder

Updates a folder. This can be also be used to move the folder, create shared links, update collaborations, and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**if_match** | Option<**String**> | Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since. |  |
**put_folders_id_request** | Option<[**PutFoldersIdRequest**](PutFoldersIdRequest.md)> |  |  |

### Return type

[**crate::models::FolderFull**](Folder--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

