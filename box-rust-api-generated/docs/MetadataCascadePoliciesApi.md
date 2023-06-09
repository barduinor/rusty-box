# \MetadataCascadePoliciesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_metadata_cascade_policies_id**](MetadataCascadePoliciesApi.md#delete_metadata_cascade_policies_id) | **DELETE** /metadata_cascade_policies/{metadata_cascade_policy_id} | Remove metadata cascade policy
[**get_metadata_cascade_policies**](MetadataCascadePoliciesApi.md#get_metadata_cascade_policies) | **GET** /metadata_cascade_policies | List metadata cascade policies
[**get_metadata_cascade_policies_id**](MetadataCascadePoliciesApi.md#get_metadata_cascade_policies_id) | **GET** /metadata_cascade_policies/{metadata_cascade_policy_id} | Get metadata cascade policy
[**post_metadata_cascade_policies**](MetadataCascadePoliciesApi.md#post_metadata_cascade_policies) | **POST** /metadata_cascade_policies | Create metadata cascade policy
[**post_metadata_cascade_policies_id_apply**](MetadataCascadePoliciesApi.md#post_metadata_cascade_policies_id_apply) | **POST** /metadata_cascade_policies/{metadata_cascade_policy_id}/apply | Force-apply metadata cascade policy to folder



## delete_metadata_cascade_policies_id

> delete_metadata_cascade_policies_id(metadata_cascade_policy_id)
Remove metadata cascade policy

Deletes a metadata cascade policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_cascade_policy_id** | **String** | The ID of the metadata cascade policy. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_cascade_policies

> crate::models::MetadataCascadePolicies get_metadata_cascade_policies(folder_id, owner_enterprise_id, marker, offset)
List metadata cascade policies

Retrieves a list of all the metadata cascade policies that are applied to a given folder. This can not be used on the root folder with ID `0`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | Specifies which folder to return policies for. This can not be used on the root folder with ID `0`. | [required] |
**owner_enterprise_id** | Option<**String**> | The ID of the enterprise ID for which to find metadata cascade policies. If not specified, it defaults to the current enterprise. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]

### Return type

[**crate::models::MetadataCascadePolicies**](MetadataCascadePolicies.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_cascade_policies_id

> crate::models::MetadataCascadePolicy get_metadata_cascade_policies_id(metadata_cascade_policy_id)
Get metadata cascade policy

Retrieve a specific metadata cascade policy assigned to a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_cascade_policy_id** | **String** | The ID of the metadata cascade policy. | [required] |

### Return type

[**crate::models::MetadataCascadePolicy**](MetadataCascadePolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_metadata_cascade_policies

> crate::models::MetadataCascadePolicy post_metadata_cascade_policies(post_metadata_cascade_policies_request)
Create metadata cascade policy

Creates a new metadata cascade policy that applies a given metadata template to a given folder and automatically cascades it down to any files within that folder.  In order for the policy to be applied a metadata instance must first be applied to the folder the policy is to be applied to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_metadata_cascade_policies_request** | Option<[**PostMetadataCascadePoliciesRequest**](PostMetadataCascadePoliciesRequest.md)> |  |  |

### Return type

[**crate::models::MetadataCascadePolicy**](MetadataCascadePolicy.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_metadata_cascade_policies_id_apply

> post_metadata_cascade_policies_id_apply(metadata_cascade_policy_id, post_metadata_cascade_policies_id_apply_request)
Force-apply metadata cascade policy to folder

Force the metadata on a folder with a metadata cascade policy to be applied to all of its children. This can be used after creating a new cascade policy to enforce the metadata to be cascaded down to all existing files within that folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_cascade_policy_id** | **String** | The ID of the cascade policy to force-apply. | [required] |
**post_metadata_cascade_policies_id_apply_request** | Option<[**PostMetadataCascadePoliciesIdApplyRequest**](PostMetadataCascadePoliciesIdApplyRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

