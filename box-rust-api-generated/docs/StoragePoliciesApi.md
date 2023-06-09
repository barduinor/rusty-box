# \StoragePoliciesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_storage_policies**](StoragePoliciesApi.md#get_storage_policies) | **GET** /storage_policies | List storage policies
[**get_storage_policies_id**](StoragePoliciesApi.md#get_storage_policies_id) | **GET** /storage_policies/{storage_policy_id} | Get storage policy



## get_storage_policies

> crate::models::StoragePolicies get_storage_policies(fields, marker, limit)
List storage policies

Fetches all the storage policies in the enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::StoragePolicies**](StoragePolicies.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_policies_id

> crate::models::StoragePolicy get_storage_policies_id(storage_policy_id)
Get storage policy

Fetches a specific storage policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_policy_id** | **String** | The ID of the storage policy. | [required] |

### Return type

[**crate::models::StoragePolicy**](StoragePolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

