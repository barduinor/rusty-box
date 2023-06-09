# \MetadataInstancesFoldersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_folders_id_metadata_id_id**](MetadataInstancesFoldersApi.md#delete_folders_id_metadata_id_id) | **DELETE** /folders/{folder_id}/metadata/{scope}/{template_key} | Remove metadata instance from folder
[**get_folders_id_metadata**](MetadataInstancesFoldersApi.md#get_folders_id_metadata) | **GET** /folders/{folder_id}/metadata | List metadata instances on folder
[**get_folders_id_metadata_id_id**](MetadataInstancesFoldersApi.md#get_folders_id_metadata_id_id) | **GET** /folders/{folder_id}/metadata/{scope}/{template_key} | Get metadata instance on folder
[**post_folders_id_metadata_id_id**](MetadataInstancesFoldersApi.md#post_folders_id_metadata_id_id) | **POST** /folders/{folder_id}/metadata/{scope}/{template_key} | Create metadata instance on folder
[**put_folders_id_metadata_id_id**](MetadataInstancesFoldersApi.md#put_folders_id_metadata_id_id) | **PUT** /folders/{folder_id}/metadata/{scope}/{template_key} | Update metadata instance on folder



## delete_folders_id_metadata_id_id

> delete_folders_id_metadata_id_id(folder_id, scope, template_key)
Remove metadata instance from folder

Deletes a piece of folder metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
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


## get_folders_id_metadata

> crate::models::Metadatas get_folders_id_metadata(folder_id)
List metadata instances on folder

Retrieves all metadata for a given folder. This can not be used on the root folder with ID `0`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |

### Return type

[**crate::models::Metadatas**](Metadatas.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders_id_metadata_id_id

> crate::models::Metadata get_folders_id_metadata_id_id(folder_id, scope, template_key)
Get metadata instance on folder

Retrieves the instance of a metadata template that has been applied to a folder. This can not be used on the root folder with ID `0`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
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


## post_folders_id_metadata_id_id

> crate::models::Metadata post_folders_id_metadata_id_id(folder_id, scope, template_key, request_body)
Create metadata instance on folder

Applies an instance of a metadata template to a folder.  In most cases only values that are present in the metadata template will be accepted, except for the `global.properties` template which accepts any key-value pair.  To display the metadata template in the Box web app the enterprise needs to be configured to enable **Cascading Folder Level Metadata** for the user in the admin console.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
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


## put_folders_id_metadata_id_id

> crate::models::Metadata put_folders_id_metadata_id_id(folder_id, scope, template_key, a_metadata_instance_update_operation)
Update metadata instance on folder

Updates a piece of metadata on a folder.  The metadata instance can only be updated if the template has already been applied to the folder before. When editing metadata, only values that match the metadata template schema will be accepted.  The update is applied atomically. If any errors occur during the application of the operations, the metadata instance will not be changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
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

