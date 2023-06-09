# \LegalHoldPoliciesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_legal_hold_policies_id**](LegalHoldPoliciesApi.md#delete_legal_hold_policies_id) | **DELETE** /legal_hold_policies/{legal_hold_policy_id} | Remove legal hold policy
[**get_legal_hold_policies**](LegalHoldPoliciesApi.md#get_legal_hold_policies) | **GET** /legal_hold_policies | List all legal hold policies
[**get_legal_hold_policies_id**](LegalHoldPoliciesApi.md#get_legal_hold_policies_id) | **GET** /legal_hold_policies/{legal_hold_policy_id} | Get legal hold policy
[**post_legal_hold_policies**](LegalHoldPoliciesApi.md#post_legal_hold_policies) | **POST** /legal_hold_policies | Create legal hold policy
[**put_legal_hold_policies_id**](LegalHoldPoliciesApi.md#put_legal_hold_policies_id) | **PUT** /legal_hold_policies/{legal_hold_policy_id} | Update legal hold policy



## delete_legal_hold_policies_id

> delete_legal_hold_policies_id(legal_hold_policy_id)
Remove legal hold policy

Delete an existing legal hold policy.  This is an asynchronous process. The policy will not be fully deleted yet when the response returns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_hold_policy_id** | **String** | The ID of the legal hold policy | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_legal_hold_policies

> crate::models::LegalHoldPolicies get_legal_hold_policies(policy_name, fields, marker, limit)
List all legal hold policies

Retrieves a list of legal hold policies that belong to an enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_name** | Option<**String**> | Limits results to policies for which the names start with this search term. This is a case-insensitive prefix. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::LegalHoldPolicies**](LegalHoldPolicies.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_legal_hold_policies_id

> crate::models::LegalHoldPolicy get_legal_hold_policies_id(legal_hold_policy_id)
Get legal hold policy

Retrieve a legal hold policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_hold_policy_id** | **String** | The ID of the legal hold policy | [required] |

### Return type

[**crate::models::LegalHoldPolicy**](LegalHoldPolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_legal_hold_policies

> crate::models::LegalHoldPolicy post_legal_hold_policies(post_legal_hold_policies_request)
Create legal hold policy

Create a new legal hold policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_legal_hold_policies_request** | Option<[**PostLegalHoldPoliciesRequest**](PostLegalHoldPoliciesRequest.md)> |  |  |

### Return type

[**crate::models::LegalHoldPolicy**](LegalHoldPolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_legal_hold_policies_id

> crate::models::LegalHoldPolicy put_legal_hold_policies_id(legal_hold_policy_id, put_legal_hold_policies_id_request)
Update legal hold policy

Update legal hold policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_hold_policy_id** | **String** | The ID of the legal hold policy | [required] |
**put_legal_hold_policies_id_request** | Option<[**PutLegalHoldPoliciesIdRequest**](PutLegalHoldPoliciesIdRequest.md)> |  |  |

### Return type

[**crate::models::LegalHoldPolicy**](LegalHoldPolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

