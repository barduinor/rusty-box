# \TermsOfServiceApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_terms_of_services**](TermsOfServiceApi.md#get_terms_of_services) | **GET** /terms_of_services | List terms of services
[**get_terms_of_services_id**](TermsOfServiceApi.md#get_terms_of_services_id) | **GET** /terms_of_services/{terms_of_service_id} | Get terms of service
[**post_terms_of_services**](TermsOfServiceApi.md#post_terms_of_services) | **POST** /terms_of_services | Create terms of service
[**put_terms_of_services_id**](TermsOfServiceApi.md#put_terms_of_services_id) | **PUT** /terms_of_services/{terms_of_service_id} | Update terms of service



## get_terms_of_services

> crate::models::TermsOfServices get_terms_of_services(tos_type)
List terms of services

Returns the current terms of service text and settings for the enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tos_type** | Option<**String**> | Limits the results to the terms of service of the given type. |  |

### Return type

[**crate::models::TermsOfServices**](TermsOfServices.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terms_of_services_id

> crate::models::TermsOfService get_terms_of_services_id(terms_of_service_id)
Get terms of service

Fetches a specific terms of service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terms_of_service_id** | **String** | The ID of the terms of service. | [required] |

### Return type

[**crate::models::TermsOfService**](TermsOfService.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_terms_of_services

> crate::models::Task post_terms_of_services(post_terms_of_services_request)
Create terms of service

Creates a terms of service for a given enterprise and type of user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_terms_of_services_request** | Option<[**PostTermsOfServicesRequest**](PostTermsOfServicesRequest.md)> |  |  |

### Return type

[**crate::models::Task**](Task.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_terms_of_services_id

> crate::models::TermsOfService put_terms_of_services_id(terms_of_service_id, put_terms_of_services_id_request)
Update terms of service

Updates a specific terms of service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terms_of_service_id** | **String** | The ID of the terms of service. | [required] |
**put_terms_of_services_id_request** | Option<[**PutTermsOfServicesIdRequest**](PutTermsOfServicesIdRequest.md)> |  |  |

### Return type

[**crate::models::TermsOfService**](TermsOfService.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

