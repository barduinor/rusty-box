# \TemplatesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sign_templates**](TemplatesApi.md#get_sign_templates) | **GET** /sign_templates | List Box Sign templates
[**get_sign_templates_id**](TemplatesApi.md#get_sign_templates_id) | **GET** /sign_templates/{template_id} | Get Box Sign template by ID



## get_sign_templates

> crate::models::SignTemplates get_sign_templates(marker, limit)
List Box Sign templates

Gets Box Sign templates created by a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::SignTemplates**](SignTemplates.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sign_templates_id

> crate::models::SignTemplate get_sign_templates_id(template_id)
Get Box Sign template by ID

Fetches details of a specific Box Sign template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The ID of a Box Sign template. | [required] |

### Return type

[**crate::models::SignTemplate**](SignTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

