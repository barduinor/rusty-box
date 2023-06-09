# \MetadataInstancesFilesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_files_id_metadata_id_id**](MetadataInstancesFilesApi.md#delete_files_id_metadata_id_id) | **DELETE** /files/{file_id}/metadata/{scope}/{template_key} | Remove metadata instance from file
[**get_files_id_metadata**](MetadataInstancesFilesApi.md#get_files_id_metadata) | **GET** /files/{file_id}/metadata | List metadata instances on file
[**get_files_id_metadata_id_id**](MetadataInstancesFilesApi.md#get_files_id_metadata_id_id) | **GET** /files/{file_id}/metadata/{scope}/{template_key} | Get metadata instance on file
[**post_files_id_metadata_id_id**](MetadataInstancesFilesApi.md#post_files_id_metadata_id_id) | **POST** /files/{file_id}/metadata/{scope}/{template_key} | Create metadata instance on file
[**put_files_id_metadata_id_id**](MetadataInstancesFilesApi.md#put_files_id_metadata_id_id) | **PUT** /files/{file_id}/metadata/{scope}/{template_key} | Update metadata instance on file



## delete_files_id_metadata_id_id

> delete_files_id_metadata_id_id(file_id, scope, template_key)
Remove metadata instance from file

Deletes a piece of file metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**scope** | **String** | The scope of the metadata template | [required] |
**template_key** | **String** | The name of the metadata template | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_metadata

> crate::models::Metadatas get_files_id_metadata(file_id)
List metadata instances on file

Retrieves all metadata for a given file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |

### Return type

[**crate::models::Metadatas**](Metadatas.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_metadata_id_id

> crate::models::Metadata get_files_id_metadata_id_id(file_id, scope, template_key)
Get metadata instance on file

Retrieves the instance of a metadata template that has been applied to a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**scope** | **String** | The scope of the metadata template | [required] |
**template_key** | **String** | The name of the metadata template | [required] |

### Return type

[**crate::models::Metadata**](Metadata.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_id_metadata_id_id

> crate::models::Metadata post_files_id_metadata_id_id(file_id, scope, template_key, request_body)
Create metadata instance on file

Applies an instance of a metadata template to a file.  In most cases only values that are present in the metadata template will be accepted, except for the `global.properties` template which accepts any key-value pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**scope** | **String** | The scope of the metadata template | [required] |
**template_key** | **String** | The name of the metadata template | [required] |
**request_body** | Option<[**::std::collections::HashMap<String, String>**](String.md)> |  |  |

### Return type

[**crate::models::Metadata**](Metadata.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id_metadata_id_id

> crate::models::Metadata put_files_id_metadata_id_id(file_id, scope, template_key, a_metadata_instance_update_operation)
Update metadata instance on file

Updates a piece of metadata on a file.  The metadata instance can only be updated if the template has already been applied to the file before. When editing metadata, only values that match the metadata template schema will be accepted.  The update is applied atomically. If any errors occur during the application of the operations, the metadata instance will not be changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**scope** | **String** | The scope of the metadata template | [required] |
**template_key** | **String** | The name of the metadata template | [required] |
**a_metadata_instance_update_operation** | Option<[**Vec<crate::models::AMetadataInstanceUpdateOperation>**](A_metadata_instance_update_operation.md)> |  |  |

### Return type

[**crate::models::Metadata**](Metadata.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

