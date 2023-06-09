# \RetentionPolicyAssignmentsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_retention_policy_assignments_id**](RetentionPolicyAssignmentsApi.md#delete_retention_policy_assignments_id) | **DELETE** /retention_policy_assignments/{retention_policy_assignment_id} | Remove retention policy assignment
[**get_retention_policies_id_assignments**](RetentionPolicyAssignmentsApi.md#get_retention_policies_id_assignments) | **GET** /retention_policies/{retention_policy_id}/assignments | List retention policy assignments
[**get_retention_policy_assignments_id**](RetentionPolicyAssignmentsApi.md#get_retention_policy_assignments_id) | **GET** /retention_policy_assignments/{retention_policy_assignment_id} | Get retention policy assignment
[**get_retention_policy_assignments_id_file_versions_under_retention**](RetentionPolicyAssignmentsApi.md#get_retention_policy_assignments_id_file_versions_under_retention) | **GET** /retention_policy_assignments/{retention_policy_assignment_id}/file_versions_under_retention | Get file versions under retention
[**get_retention_policy_assignments_id_files_under_retention**](RetentionPolicyAssignmentsApi.md#get_retention_policy_assignments_id_files_under_retention) | **GET** /retention_policy_assignments/{retention_policy_assignment_id}/files_under_retention | Get files under retention
[**post_retention_policy_assignments**](RetentionPolicyAssignmentsApi.md#post_retention_policy_assignments) | **POST** /retention_policy_assignments | Assign retention policy



## delete_retention_policy_assignments_id

> delete_retention_policy_assignments_id(retention_policy_assignment_id)
Remove retention policy assignment

Removes a retention policy assignment applied to content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retention_policy_assignment_id** | **String** | The ID of the retention policy assignment. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention_policies_id_assignments

> crate::models::RetentionPolicyAssignments get_retention_policies_id_assignments(retention_policy_id, r#type, fields, marker, limit)
List retention policy assignments

Returns a list of all retention policy assignments associated with a specified retention policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retention_policy_id** | **String** | The ID of the retention policy. | [required] |
**r#type** | Option<**String**> | The type of the retention policy assignment to retrieve. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::RetentionPolicyAssignments**](RetentionPolicyAssignments.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention_policy_assignments_id

> crate::models::RetentionPolicyAssignment get_retention_policy_assignments_id(retention_policy_assignment_id, fields)
Get retention policy assignment

Retrieves a retention policy assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retention_policy_assignment_id** | **String** | The ID of the retention policy assignment. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::RetentionPolicyAssignment**](RetentionPolicyAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention_policy_assignments_id_file_versions_under_retention

> crate::models::FilesUnderRetention get_retention_policy_assignments_id_file_versions_under_retention(retention_policy_assignment_id, marker, limit)
Get file versions under retention

Returns a list of file versions under retention for a retention policy assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retention_policy_assignment_id** | **String** | The ID of the retention policy assignment. | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::FilesUnderRetention**](FilesUnderRetention.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention_policy_assignments_id_files_under_retention

> crate::models::FilesUnderRetention get_retention_policy_assignments_id_files_under_retention(retention_policy_assignment_id, marker, limit)
Get files under retention

Returns a list of files under retention for a retention policy assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retention_policy_assignment_id** | **String** | The ID of the retention policy assignment. | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::FilesUnderRetention**](FilesUnderRetention.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_retention_policy_assignments

> crate::models::RetentionPolicyAssignment post_retention_policy_assignments(post_retention_policy_assignments_request)
Assign retention policy

Assigns a retention policy to an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_retention_policy_assignments_request** | Option<[**PostRetentionPolicyAssignmentsRequest**](PostRetentionPolicyAssignmentsRequest.md)> |  |  |

### Return type

[**crate::models::RetentionPolicyAssignment**](RetentionPolicyAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

