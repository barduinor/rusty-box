# \UploadsChunkedApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_files_upload_sessions_id**](UploadsChunkedApi.md#delete_files_upload_sessions_id) | **DELETE** /files/upload_sessions/{upload_session_id} | Remove upload session
[**get_files_upload_sessions_id**](UploadsChunkedApi.md#get_files_upload_sessions_id) | **GET** /files/upload_sessions/{upload_session_id} | Get upload session
[**get_files_upload_sessions_id_parts**](UploadsChunkedApi.md#get_files_upload_sessions_id_parts) | **GET** /files/upload_sessions/{upload_session_id}/parts | List parts
[**post_files_id_upload_sessions**](UploadsChunkedApi.md#post_files_id_upload_sessions) | **POST** /files/{file_id}/upload_sessions | Create upload session for existing file
[**post_files_upload_sessions**](UploadsChunkedApi.md#post_files_upload_sessions) | **POST** /files/upload_sessions | Create upload session
[**post_files_upload_sessions_id_commit**](UploadsChunkedApi.md#post_files_upload_sessions_id_commit) | **POST** /files/upload_sessions/{upload_session_id}/commit | Commit upload session
[**put_files_upload_sessions_id**](UploadsChunkedApi.md#put_files_upload_sessions_id) | **PUT** /files/upload_sessions/{upload_session_id} | Upload part of file



## delete_files_upload_sessions_id

> delete_files_upload_sessions_id(upload_session_id)
Remove upload session

Abort an upload session and discard all data uploaded.  This cannot be reversed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_session_id** | **String** | The ID of the upload session. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_upload_sessions_id

> crate::models::UploadSession get_files_upload_sessions_id(upload_session_id)
Get upload session

Return information about an upload session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_session_id** | **String** | The ID of the upload session. | [required] |

### Return type

[**crate::models::UploadSession**](UploadSession.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_upload_sessions_id_parts

> crate::models::UploadParts get_files_upload_sessions_id_parts(upload_session_id, offset, limit)
List parts

Return a list of the chunks uploaded to the upload session so far.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_session_id** | **String** | The ID of the upload session. | [required] |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::UploadParts**](UploadParts.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_id_upload_sessions

> crate::models::UploadSession post_files_id_upload_sessions(file_id, post_files_id_upload_sessions_request)
Create upload session for existing file

Creates an upload session for an existing file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**post_files_id_upload_sessions_request** | Option<[**PostFilesIdUploadSessionsRequest**](PostFilesIdUploadSessionsRequest.md)> |  |  |

### Return type

[**crate::models::UploadSession**](UploadSession.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_upload_sessions

> crate::models::UploadSession post_files_upload_sessions(post_files_upload_sessions_request)
Create upload session

Creates an upload session for a new file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_files_upload_sessions_request** | Option<[**PostFilesUploadSessionsRequest**](PostFilesUploadSessionsRequest.md)> |  |  |

### Return type

[**crate::models::UploadSession**](UploadSession.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_upload_sessions_id_commit

> crate::models::Files post_files_upload_sessions_id_commit(upload_session_id, digest, if_match, if_none_match, post_files_upload_sessions_id_commit_request)
Commit upload session

Close an upload session and create a file from the uploaded chunks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_session_id** | **String** | The ID of the upload session. | [required] |
**digest** | **String** | The [RFC3230][1] message digest of the whole file.  Only SHA1 is supported. The SHA1 digest must be Base64 encoded. The format of this header is as `sha=BASE64_ENCODED_DIGEST`.  [1]: https://tools.ietf.org/html/rfc3230 | [required] |
**if_match** | Option<**String**> | Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since. |  |
**if_none_match** | Option<**String**> | Ensures an item is only returned if it has changed.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `304 Not Modified` if the item has not changed since. |  |
**post_files_upload_sessions_id_commit_request** | Option<[**PostFilesUploadSessionsIdCommitRequest**](PostFilesUploadSessionsIdCommitRequest.md)> |  |  |

### Return type

[**crate::models::Files**](Files.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_upload_sessions_id

> crate::models::UploadedPart put_files_upload_sessions_id(upload_session_id, digest, content_range, body)
Upload part of file

Updates a chunk of an upload session for a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_session_id** | **String** | The ID of the upload session. | [required] |
**digest** | **String** | The [RFC3230][1] message digest of the chunk uploaded.  Only SHA1 is supported. The SHA1 digest must be base64 encoded. The format of this header is as `sha=BASE64_ENCODED_DIGEST`.  To get the value for the `SHA` digest, use the openSSL command to encode the file part: `openssl sha1 -binary <FILE_PART_NAME> | base64`  [1]: https://tools.ietf.org/html/rfc3230 | [required] |
**content_range** | **String** | The byte range of the chunk.  Must not overlap with the range of a part already uploaded this session. Each part’s size must be exactly equal in size to the part size specified in the upload session that you created. One exception is the last part of the file, as this can be smaller.  When providing the value for `content-range`, remember that:  * The lower bound of each part's byte range   must be a multiple of the part size. * The higher bound must be a multiple of the part size - 1. | [required] |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**crate::models::UploadedPart**](UploadedPart.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

