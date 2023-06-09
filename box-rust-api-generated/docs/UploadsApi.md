# \UploadsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_files_content**](UploadsApi.md#post_files_content) | **POST** /files/content | Upload file
[**post_files_id_content**](UploadsApi.md#post_files_id_content) | **POST** /files/{file_id}/content | Upload file version



## post_files_content

> crate::models::Files post_files_content(attributes, file, fields, content_md5)
Upload file

Uploads a small file to Box. For file sizes over 50MB we recommend using the Chunk Upload APIs.  # Request body order  The `attributes` part of the body must come **before** the `file` part. Requests that do not follow this format when uploading the file will receive a HTTP `400` error with a `metadata_after_file_contents` error code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attributes** | [**crate::models::PostFilesContentRequestAttributes**](post_files_content_request_attributes.md) |  | [required] |
**file** | **std::path::PathBuf** | The content of the file to upload to Box.  <Message warning>    The `attributes` part of the body must come **before** the   `file` part. Requests that do not follow this format when   uploading the file will receive a HTTP `400` error with a   `metadata_after_file_contents` error code.  </Message> | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**content_md5** | Option<**String**> | An optional header containing the SHA1 hash of the file to ensure that the file was not corrupted in transit. |  |

### Return type

[**crate::models::Files**](Files.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_id_content

> crate::models::Files post_files_id_content(file_id, attributes, file, if_match, fields, content_md5)
Upload file version

Update a file's content. For file sizes over 50MB we recommend using the Chunk Upload APIs.  # Request body order  The `attributes` part of the body must come **before** the `file` part. Requests that do not follow this format when uploading the file will receive a HTTP `400` error with a `metadata_after_file_contents` error code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**attributes** | [**crate::models::PostFilesIdContentRequestAttributes**](post_files_id_content_request_attributes.md) |  | [required] |
**file** | **std::path::PathBuf** | The content of the file to upload to Box.  <Message warning>    The `attributes` part of the body must come **before** the   `file` part. Requests that do not follow this format when   uploading the file will receive a HTTP `400` error with a   `metadata_after_file_contents` error code.  </Message> | [required] |
**if_match** | Option<**String**> | Ensures this item hasn't recently changed before making changes.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `412 Precondition Failed` if it has changed since. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**content_md5** | Option<**String**> | An optional header containing the SHA1 hash of the file to ensure that the file was not corrupted in transit. |  |

### Return type

[**crate::models::Files**](Files.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

