# \TermsOfServiceUserStatusesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_terms_of_service_user_statuses**](TermsOfServiceUserStatusesApi.md#get_terms_of_service_user_statuses) | **GET** /terms_of_service_user_statuses | List terms of service user statuses
[**post_terms_of_service_user_statuses**](TermsOfServiceUserStatusesApi.md#post_terms_of_service_user_statuses) | **POST** /terms_of_service_user_statuses | Create terms of service status for new user
[**put_terms_of_service_user_statuses_id**](TermsOfServiceUserStatusesApi.md#put_terms_of_service_user_statuses_id) | **PUT** /terms_of_service_user_statuses/{terms_of_service_user_status_id} | Update terms of service status for existing user



## get_terms_of_service_user_statuses

> crate::models::TermsOfServiceUserStatuses get_terms_of_service_user_statuses(tos_id, user_id)
List terms of service user statuses

Retrieves an overview of users and their status for a terms of service, including Whether they have accepted the terms and when.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tos_id** | **String** | The ID of the terms of service. | [required] |
**user_id** | Option<**String**> | Limits results to the given user ID. |  |

### Return type

[**crate::models::TermsOfServiceUserStatuses**](TermsOfServiceUserStatuses.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_terms_of_service_user_statuses

> crate::models::TermsOfServiceUserStatus post_terms_of_service_user_statuses(post_terms_of_service_user_statuses_request)
Create terms of service status for new user

Sets the status for a terms of service for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_terms_of_service_user_statuses_request** | Option<[**PostTermsOfServiceUserStatusesRequest**](PostTermsOfServiceUserStatusesRequest.md)> |  |  |

### Return type

[**crate::models::TermsOfServiceUserStatus**](TermsOfServiceUserStatus.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_terms_of_service_user_statuses_id

> crate::models::TermsOfServiceUserStatus put_terms_of_service_user_statuses_id(terms_of_service_user_status_id, put_terms_of_service_user_statuses_id_request)
Update terms of service status for existing user

Updates the status for a terms of service for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terms_of_service_user_status_id** | **String** | The ID of the terms of service status. | [required] |
**put_terms_of_service_user_statuses_id_request** | Option<[**PutTermsOfServiceUserStatusesIdRequest**](PutTermsOfServiceUserStatusesIdRequest.md)> |  |  |

### Return type

[**crate::models::TermsOfServiceUserStatus**](TermsOfServiceUserStatus.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

