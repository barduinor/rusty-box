# \MetadataTemplatesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_metadata_templates_id_id_schema**](MetadataTemplatesApi.md#delete_metadata_templates_id_id_schema) | **DELETE** /metadata_templates/{scope}/{template_key}/schema | Remove metadata template
[**get_metadata_templates**](MetadataTemplatesApi.md#get_metadata_templates) | **GET** /metadata_templates | Find metadata template by instance ID
[**get_metadata_templates_enterprise**](MetadataTemplatesApi.md#get_metadata_templates_enterprise) | **GET** /metadata_templates/enterprise | List all metadata templates for enterprise
[**get_metadata_templates_global**](MetadataTemplatesApi.md#get_metadata_templates_global) | **GET** /metadata_templates/global | List all global metadata templates
[**get_metadata_templates_id**](MetadataTemplatesApi.md#get_metadata_templates_id) | **GET** /metadata_templates/{template_id} | Get metadata template by ID
[**get_metadata_templates_id_id_schema**](MetadataTemplatesApi.md#get_metadata_templates_id_id_schema) | **GET** /metadata_templates/{scope}/{template_key}/schema | Get metadata template by name
[**post_metadata_templates_schema**](MetadataTemplatesApi.md#post_metadata_templates_schema) | **POST** /metadata_templates/schema | Create metadata template
[**put_metadata_templates_id_id_schema**](MetadataTemplatesApi.md#put_metadata_templates_id_id_schema) | **PUT** /metadata_templates/{scope}/{template_key}/schema | Update metadata template



## delete_metadata_templates_id_id_schema

> delete_metadata_templates_id_id_schema(scope, template_key)
Remove metadata template

Delete a metadata template and its instances. This deletion is permanent and can not be reversed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_metadata_templates

> crate::models::MetadataTemplates get_metadata_templates(metadata_instance_id)
Find metadata template by instance ID

Finds a metadata template by searching for the ID of an instance of the template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_instance_id** | **uuid::Uuid** | The ID of an instance of the metadata template to find. | [required] |

### Return type

[**crate::models::MetadataTemplates**](MetadataTemplates.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_templates_enterprise

> crate::models::MetadataTemplates get_metadata_templates_enterprise(marker, limit)
List all metadata templates for enterprise

Used to retrieve all metadata templates created to be used specifically within the user's enterprise

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::MetadataTemplates**](MetadataTemplates.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_templates_global

> crate::models::MetadataTemplates get_metadata_templates_global(marker, limit)
List all global metadata templates

Used to retrieve all generic, global metadata templates available to all enterprises using Box.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::MetadataTemplates**](MetadataTemplates.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_templates_id

> crate::models::MetadataTemplate get_metadata_templates_id(template_id)
Get metadata template by ID

Retrieves a metadata template by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The ID of the template | [required] |

### Return type

[**crate::models::MetadataTemplate**](MetadataTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_templates_id_id_schema

> crate::models::MetadataTemplate get_metadata_templates_id_id_schema(scope, template_key)
Get metadata template by name

Retrieves a metadata template by its `scope` and `templateKey` values.  To find the `scope` and `templateKey` for a template, list all templates for an enterprise or globally, or list all templates applied to a file or folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the metadata template | [required] |
**template_key** | **String** | The name of the metadata template | [required] |

### Return type

[**crate::models::MetadataTemplate**](MetadataTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_metadata_templates_schema

> crate::models::MetadataTemplate post_metadata_templates_schema(post_metadata_templates_schema_request)
Create metadata template

Creates a new metadata template that can be applied to files and folders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_metadata_templates_schema_request** | Option<[**PostMetadataTemplatesSchemaRequest**](PostMetadataTemplatesSchemaRequest.md)> |  |  |

### Return type

[**crate::models::MetadataTemplate**](MetadataTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_metadata_templates_id_id_schema

> crate::models::MetadataTemplate put_metadata_templates_id_id_schema(scope, template_key, a_metadata_template_update_operation)
Update metadata template

Updates a metadata template.  The metadata template can only be updated if the template already exists.  The update is applied atomically. If any errors occur during the application of the operations, the metadata template will not be changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the metadata template | [required] |
**template_key** | **String** | The name of the metadata template | [required] |
**a_metadata_template_update_operation** | Option<[**Vec<crate::models::AMetadataTemplateUpdateOperation>**](A_metadata_template_update_operation.md)> |  |  |

### Return type

[**crate::models::MetadataTemplate**](MetadataTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

