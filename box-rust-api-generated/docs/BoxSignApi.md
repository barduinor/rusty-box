# \BoxSignApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sign_requests**](BoxSignApi.md#get_sign_requests) | **GET** /sign_requests | List sign requests
[**get_sign_requests_id**](BoxSignApi.md#get_sign_requests_id) | **GET** /sign_requests/{sign_request_id} | Get sign request by ID
[**post_sign_requests**](BoxSignApi.md#post_sign_requests) | **POST** /sign_requests | Create sign request
[**post_sign_requests_id_cancel**](BoxSignApi.md#post_sign_requests_id_cancel) | **POST** /sign_requests/{sign_request_id}/cancel | Cancel sign request
[**post_sign_requests_id_resend**](BoxSignApi.md#post_sign_requests_id_resend) | **POST** /sign_requests/{sign_request_id}/resend | Resend sign request



## get_sign_requests

> crate::models::SignRequests get_sign_requests(marker, limit)
List sign requests

Gets sign requests created by a user. If the `sign_files` and/or `parent_folder` are deleted, the sign request will not return in the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::SignRequests**](SignRequests.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sign_requests_id

> crate::models::SignRequest get_sign_requests_id(sign_request_id)
Get sign request by ID

Gets a sign request by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_request_id** | **String** | The ID of the sign request | [required] |

### Return type

[**crate::models::SignRequest**](SignRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sign_requests

> crate::models::SignRequest post_sign_requests(sign_request_create_request)
Create sign request

Creates a sign request. This involves preparing a document for signing and sending the sign request to signers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_request_create_request** | Option<[**SignRequestCreateRequest**](SignRequestCreateRequest.md)> |  |  |

### Return type

[**crate::models::SignRequest**](SignRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sign_requests_id_cancel

> crate::models::SignRequest post_sign_requests_id_cancel(sign_request_id)
Cancel sign request

Cancels a sign request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_request_id** | **String** | The ID of the sign request | [required] |

### Return type

[**crate::models::SignRequest**](SignRequest.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sign_requests_id_resend

> post_sign_requests_id_resend(sign_request_id)
Resend sign request

Resends a sign request email to all outstanding signers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_request_id** | **String** | The ID of the sign request | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

