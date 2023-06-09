# \StoragePolicyAssignmentsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_storage_policy_assignments_id**](StoragePolicyAssignmentsApi.md#delete_storage_policy_assignments_id) | **DELETE** /storage_policy_assignments/{storage_policy_assignment_id} | Unassign storage policy
[**get_storage_policy_assignments**](StoragePolicyAssignmentsApi.md#get_storage_policy_assignments) | **GET** /storage_policy_assignments | List storage policy assignments
[**get_storage_policy_assignments_id**](StoragePolicyAssignmentsApi.md#get_storage_policy_assignments_id) | **GET** /storage_policy_assignments/{storage_policy_assignment_id} | Get storage policy assignment
[**post_storage_policy_assignments**](StoragePolicyAssignmentsApi.md#post_storage_policy_assignments) | **POST** /storage_policy_assignments | Assign storage policy
[**put_storage_policy_assignments_id**](StoragePolicyAssignmentsApi.md#put_storage_policy_assignments_id) | **PUT** /storage_policy_assignments/{storage_policy_assignment_id} | Update storage policy assignment



## delete_storage_policy_assignments_id

> delete_storage_policy_assignments_id(storage_policy_assignment_id)
Unassign storage policy

Delete a storage policy assignment.  Deleting a storage policy assignment on a user will have the user inherit the enterprise's default storage policy.  There is a rate limit for calling this endpoint of only twice per user in a 24 hour time frame.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_policy_assignment_id** | **String** | The ID of the storage policy assignment. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_policy_assignments

> crate::models::StoragePolicyAssignments get_storage_policy_assignments(resolved_for_type, resolved_for_id, marker)
List storage policy assignments

Fetches all the storage policy assignment for an enterprise or user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resolved_for_type** | **String** | The target type to return assignments for | [required] |
**resolved_for_id** | **String** | The ID of the user or enterprise to return assignments for | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |

### Return type

[**crate::models::StoragePolicyAssignments**](StoragePolicyAssignments.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_policy_assignments_id

> crate::models::StoragePolicyAssignment get_storage_policy_assignments_id(storage_policy_assignment_id)
Get storage policy assignment

Fetches a specific storage policy assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_policy_assignment_id** | **String** | The ID of the storage policy assignment. | [required] |

### Return type

[**crate::models::StoragePolicyAssignment**](StoragePolicyAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_storage_policy_assignments

> crate::models::StoragePolicyAssignment post_storage_policy_assignments(post_storage_policy_assignments_request)
Assign storage policy

Creates a storage policy assignment for an enterprise or user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_storage_policy_assignments_request** | Option<[**PostStoragePolicyAssignmentsRequest**](PostStoragePolicyAssignmentsRequest.md)> |  |  |

### Return type

[**crate::models::StoragePolicyAssignment**](StoragePolicyAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_storage_policy_assignments_id

> crate::models::StoragePolicyAssignment put_storage_policy_assignments_id(storage_policy_assignment_id, put_storage_policy_assignments_id_request)
Update storage policy assignment

Updates a specific storage policy assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_policy_assignment_id** | **String** | The ID of the storage policy assignment. | [required] |
**put_storage_policy_assignments_id_request** | Option<[**PutStoragePolicyAssignmentsIdRequest**](PutStoragePolicyAssignmentsIdRequest.md)> |  |  |

### Return type

[**crate::models::StoragePolicyAssignment**](StoragePolicyAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

