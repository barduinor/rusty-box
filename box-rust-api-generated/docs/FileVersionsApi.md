# \FileVersionsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_files_id_versions_id**](FileVersionsApi.md#delete_files_id_versions_id) | **DELETE** /files/{file_id}/versions/{file_version_id} | Remove file version
[**get_files_id_versions**](FileVersionsApi.md#get_files_id_versions) | **GET** /files/{file_id}/versions | List all file versions
[**get_files_id_versions_id**](FileVersionsApi.md#get_files_id_versions_id) | **GET** /files/{file_id}/versions/{file_version_id} | Get file version
[**post_files_id_versions_current**](FileVersionsApi.md#post_files_id_versions_current) | **POST** /files/{file_id}/versions/current | Promote file version
[**put_files_id_versions_id**](FileVersionsApi.md#put_files_id_versions_id) | **PUT** /files/{file_id}/versions/{file_version_id} | Restore file version



## delete_files_id_versions_id

> delete_files_id_versions_id(file_id, file_version_id, if_match)
Remove file version

Move a file version to the trash.  Versions are only tracked for Box users with premium accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**file_version_id** | **String** | The ID of the file version | [required] |
**if_match** | Option<**String**> | Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_versions

> crate::models::FileVersions get_files_id_versions(file_id, fields, limit, offset)
List all file versions

Retrieve a list of the past versions for a file.  Versions are only tracked by Box users with premium accounts. To fetch the ID of the current version of a file, use the `GET /file/:id` API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]

### Return type

[**crate::models::FileVersions**](FileVersions.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_versions_id

> crate::models::FileVersionFull get_files_id_versions_id(file_id, file_version_id, fields)
Get file version

Retrieve a specific version of a file.  Versions are only tracked for Box users with premium accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**file_version_id** | **String** | The ID of the file version | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::FileVersionFull**](FileVersion--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_id_versions_current

> crate::models::FileVersionFull post_files_id_versions_current(file_id, fields, post_files_id_versions_current_request)
Promote file version

Promote a specific version of a file.  If previous versions exist, this method can be used to promote one of the older versions to the top of the version history.  This creates a new copy of the old version and puts it at the top of the versions history. The file will have the exact same contents as the older version, with the the same hash digest, `etag`, and name as the original.  Other properties such as comments do not get updated to their former values.  Don't use this endpoint to restore Box Notes, as it works with file formats such as PDF, DOC, PPTX or similar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_files_id_versions_current_request** | Option<[**PostFilesIdVersionsCurrentRequest**](PostFilesIdVersionsCurrentRequest.md)> |  |  |

### Return type

[**crate::models::FileVersionFull**](FileVersion--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id_versions_id

> crate::models::FileVersionFull put_files_id_versions_id(file_id, file_version_id, put_files_id_versions_id_request)
Restore file version

Restores a specific version of a file after it was deleted. Don't use this endpoint to restore Box Notes, as it works with file formats such as PDF, DOC, PPTX or similar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**file_version_id** | **String** | The ID of the file version | [required] |
**put_files_id_versions_id_request** | Option<[**PutFilesIdVersionsIdRequest**](PutFilesIdVersionsIdRequest.md)> |  |  |

### Return type

[**crate::models::FileVersionFull**](FileVersion--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

