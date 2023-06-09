# \RetentionPoliciesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_retention_policies_id**](RetentionPoliciesApi.md#delete_retention_policies_id) | **DELETE** /retention_policies/{retention_policy_id} | Delete retention policy
[**get_retention_policies**](RetentionPoliciesApi.md#get_retention_policies) | **GET** /retention_policies | List retention policies
[**get_retention_policies_id**](RetentionPoliciesApi.md#get_retention_policies_id) | **GET** /retention_policies/{retention_policy_id} | Get retention policy
[**post_retention_policies**](RetentionPoliciesApi.md#post_retention_policies) | **POST** /retention_policies | Create retention policy
[**put_retention_policies_id**](RetentionPoliciesApi.md#put_retention_policies_id) | **PUT** /retention_policies/{retention_policy_id} | Update retention policy



## delete_retention_policies_id

> delete_retention_policies_id(retention_policy_id)
Delete retention policy

Permanently deletes a retention policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retention_policy_id** | **String** | The ID of the retention policy. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention_policies

> crate::models::RetentionPolicies get_retention_policies(policy_name, policy_type, created_by_user_id, fields, limit, marker)
List retention policies

Retrieves all of the retention policies for an enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_name** | Option<**String**> | Filters results by a case sensitive prefix of the name of retention policies. |  |
**policy_type** | Option<**String**> | Filters results by the type of retention policy. |  |
**created_by_user_id** | Option<**String**> | Filters results by the ID of the user who created policy. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination. |  |

### Return type

[**crate::models::RetentionPolicies**](RetentionPolicies.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention_policies_id

> crate::models::RetentionPolicy get_retention_policies_id(retention_policy_id, fields)
Get retention policy

Retrieves a retention policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retention_policy_id** | **String** | The ID of the retention policy. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::RetentionPolicy**](RetentionPolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_retention_policies

> crate::models::RetentionPolicy post_retention_policies(post_retention_policies_request)
Create retention policy

Creates a retention policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_retention_policies_request** | Option<[**PostRetentionPoliciesRequest**](PostRetentionPoliciesRequest.md)> |  |  |

### Return type

[**crate::models::RetentionPolicy**](RetentionPolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_retention_policies_id

> crate::models::RetentionPolicy put_retention_policies_id(retention_policy_id, put_retention_policies_id_request)
Update retention policy

Updates a retention policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retention_policy_id** | **String** | The ID of the retention policy. | [required] |
**put_retention_policies_id_request** | Option<[**PutRetentionPoliciesIdRequest**](PutRetentionPoliciesIdRequest.md)> |  |  |

### Return type

[**crate::models::RetentionPolicy**](RetentionPolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

