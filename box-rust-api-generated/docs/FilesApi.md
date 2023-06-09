# \FilesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_files_id**](FilesApi.md#delete_files_id) | **DELETE** /files/{file_id} | Delete file
[**get_files_id**](FilesApi.md#get_files_id) | **GET** /files/{file_id} | Get file information
[**get_files_id_thumbnail_id**](FilesApi.md#get_files_id_thumbnail_id) | **GET** /files/{file_id}/thumbnail.{extension} | Get file thumbnail
[**options_files_content**](FilesApi.md#options_files_content) | **OPTIONS** /files/content | Preflight check before upload
[**post_files_id_copy**](FilesApi.md#post_files_id_copy) | **POST** /files/{file_id}/copy | Copy file
[**put_files_id**](FilesApi.md#put_files_id) | **PUT** /files/{file_id} | Update file



## delete_files_id

> delete_files_id(file_id, if_match)
Delete file

Deletes a file, either permanently or by moving it to the trash.  The the enterprise settings determine whether the item will be permanently deleted from Box or moved to the trash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**if_match** | Option<**String**> | Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id

> crate::models::FileFull get_files_id(file_id, fields, if_none_match, boxapi, x_rep_hints)
Get file information

Retrieves the details about a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested.  Additionally this field can be used to query any metadata applied to the file by specifying the `metadata` field as well as the scope and key of the template to retrieve, for example `?field=metadata.enterprise_12345.contractTemplate`. |  |
**if_none_match** | Option<**String**> | Ensures an item is only returned if it has changed.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `304 Not Modified` if the item has not changed since. |  |
**boxapi** | Option<**String**> | The URL, and optional password, for the shared link of this item.  This header can be used to access items that have not been explicitly shared with a user.  Use the format `shared_link=[link]` or if a password is required then use `shared_link=[link]&shared_link_password=[password]`.  This header can be used on the file or folder shared, as well as on any files or folders nested within the item. |  |
**x_rep_hints** | Option<**String**> | A header required to request specific `representations` of a file. Use this in combination with the `fields` query parameter to request a specific file representation.  The general format for these representations is `X-Rep-Hints: [...]` where `[...]` is one or many hints in the format `[fileType?query]`.  For example, to request a `png` representation in `32x32` as well as `64x64` pixel dimensions provide the following hints.  `x-rep-hints: [jpg?dimensions=32x32][jpg?dimensions=64x64]`  Additionally, a `text` representation is available for all document file types in Box using the `[extracted_text]` representation.  `x-rep-hints: [extracted_text]` |  |

### Return type

[**crate::models::FileFull**](File--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_thumbnail_id

> std::path::PathBuf get_files_id_thumbnail_id(file_id, extension, min_height, min_width, max_height, max_width)
Get file thumbnail

Retrieves a thumbnail, or smaller image representation, of a file.  Sizes of `32x32`,`64x64`, `128x128`, and `256x256` can be returned in the `.png` format and sizes of `32x32`, `160x160`, and `320x320` can be returned in the `.jpg` format.  Thumbnails can be generated for the image and video file formats listed [found on our community site][1].  [1]: https://community.box.com/t5/Migrating-and-Previewing-Content/File-Types-and-Fonts-Supported-in-Box-Content-Preview/ta-p/327

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**extension** | **String** | The file format for the thumbnail | [required] |
**min_height** | Option<**i32**> | The minimum height of the thumbnail |  |
**min_width** | Option<**i32**> | The minimum width of the thumbnail |  |
**max_height** | Option<**i32**> | The maximum height of the thumbnail |  |
**max_width** | Option<**i32**> | The maximum width of the thumbnail |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, image/jpg, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_files_content

> crate::models::UploadUrl options_files_content(options_files_content_request)
Preflight check before upload

Performs a check to verify that a file will be accepted by Box before you upload the entire file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**options_files_content_request** | Option<[**OptionsFilesContentRequest**](OptionsFilesContentRequest.md)> |  |  |

### Return type

[**crate::models::UploadUrl**](UploadUrl.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_id_copy

> crate::models::FileFull post_files_id_copy(file_id, fields, post_files_id_copy_request)
Copy file

Creates a copy of a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_files_id_copy_request** | Option<[**PostFilesIdCopyRequest**](PostFilesIdCopyRequest.md)> |  |  |

### Return type

[**crate::models::FileFull**](File--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id

> crate::models::FileFull put_files_id(file_id, fields, if_match, put_files_id_request)
Update file

Updates a file. This can be used to rename or move a file, create a shared link, or lock a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**if_match** | Option<**String**> | Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since. |  |
**put_files_id_request** | Option<[**PutFilesIdRequest**](PutFilesIdRequest.md)> |  |  |

### Return type

[**crate::models::FileFull**](File--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

