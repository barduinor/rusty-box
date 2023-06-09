# \LegalHoldPolicyAssignmentsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_legal_hold_policy_assignments_id**](LegalHoldPolicyAssignmentsApi.md#delete_legal_hold_policy_assignments_id) | **DELETE** /legal_hold_policy_assignments/{legal_hold_policy_assignment_id} | Unassign legal hold policy
[**get_legal_hold_policy_assignments**](LegalHoldPolicyAssignmentsApi.md#get_legal_hold_policy_assignments) | **GET** /legal_hold_policy_assignments | List legal hold policy assignments
[**get_legal_hold_policy_assignments_id**](LegalHoldPolicyAssignmentsApi.md#get_legal_hold_policy_assignments_id) | **GET** /legal_hold_policy_assignments/{legal_hold_policy_assignment_id} | Get legal hold policy assignment
[**get_legal_hold_policy_assignments_id_file_versions_on_hold**](LegalHoldPolicyAssignmentsApi.md#get_legal_hold_policy_assignments_id_file_versions_on_hold) | **GET** /legal_hold_policy_assignments/{legal_hold_policy_assignment_id}/file_versions_on_hold | List previous file versions for legal hold policy assignment
[**get_legal_hold_policy_assignments_id_files_on_hold**](LegalHoldPolicyAssignmentsApi.md#get_legal_hold_policy_assignments_id_files_on_hold) | **GET** /legal_hold_policy_assignments/{legal_hold_policy_assignment_id}/files_on_hold | List current file versions for legal hold policy assignment
[**post_legal_hold_policy_assignments**](LegalHoldPolicyAssignmentsApi.md#post_legal_hold_policy_assignments) | **POST** /legal_hold_policy_assignments | Assign legal hold policy



## delete_legal_hold_policy_assignments_id

> delete_legal_hold_policy_assignments_id(legal_hold_policy_assignment_id)
Unassign legal hold policy

Remove a legal hold from an item.  This is an asynchronous process. The policy will not be fully removed yet when the response returns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_hold_policy_assignment_id** | **String** | The ID of the legal hold policy assignment | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_legal_hold_policy_assignments

> crate::models::LegalHoldPolicyAssignments get_legal_hold_policy_assignments(policy_id, assign_to_type, assign_to_id, marker, limit, fields)
List legal hold policy assignments

Retrieves a list of items a legal hold policy has been assigned to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the legal hold policy | [required] |
**assign_to_type** | Option<**String**> | Filters the results by the type of item the policy was applied to. |  |
**assign_to_id** | Option<**String**> | Filters the results by the ID of item the policy was applied to. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::LegalHoldPolicyAssignments**](LegalHoldPolicyAssignments.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_legal_hold_policy_assignments_id

> crate::models::LegalHoldPolicyAssignment get_legal_hold_policy_assignments_id(legal_hold_policy_assignment_id)
Get legal hold policy assignment

Retrieve a legal hold policy assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_hold_policy_assignment_id** | **String** | The ID of the legal hold policy assignment | [required] |

### Return type

[**crate::models::LegalHoldPolicyAssignment**](LegalHoldPolicyAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_legal_hold_policy_assignments_id_file_versions_on_hold

> crate::models::FileVersionLegalHolds get_legal_hold_policy_assignments_id_file_versions_on_hold(legal_hold_policy_assignment_id, marker, limit, fields)
List previous file versions for legal hold policy assignment

Get a list of previous file versions for a legal hold assignment.  In some cases you may only need the latest file versions instead. In these cases, use the `GET  /legal_hold_policy_assignments/:id/files_on_hold` API instead to return any current (latest) versions of a file for this legal hold policy assignment.  Due to ongoing re-architecture efforts this API might not return all files held for this policy ID. Instead, this API will only return past file versions held in the newly developed architecture. The `GET /file_version_legal_holds` API can be used to fetch current and past versions of files held within the legacy architecture.  The `GET /legal_hold_policy_assignments?policy_id={id}` API can be used to find a list of policy assignments for a given policy ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_hold_policy_assignment_id** | **String** | The ID of the legal hold policy assignment | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::FileVersionLegalHolds**](FileVersionLegalHolds.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_legal_hold_policy_assignments_id_files_on_hold

> crate::models::FileVersionLegalHolds get_legal_hold_policy_assignments_id_files_on_hold(legal_hold_policy_assignment_id, marker, limit, fields)
List current file versions for legal hold policy assignment

Get a list of current file versions for a legal hold assignment.  In some cases you may want to get previous file versions instead. In these cases, use the `GET  /legal_hold_policy_assignments/:id/file_versions_on_hold` API instead to return any previous versions of a file for this legal hold policy assignment.  Due to ongoing re-architecture efforts this API might not return all file versions held for this policy ID. Instead, this API will only return the latest file version held in the newly developed architecture. The `GET /file_version_legal_holds` API can be used to fetch current and past versions of files held within the legacy architecture.  The `GET /legal_hold_policy_assignments?policy_id={id}` API can be used to find a list of policy assignments for a given policy ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_hold_policy_assignment_id** | **String** | The ID of the legal hold policy assignment | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::FileVersionLegalHolds**](FileVersionLegalHolds.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_legal_hold_policy_assignments

> crate::models::LegalHoldPolicyAssignment post_legal_hold_policy_assignments(post_legal_hold_policy_assignments_request)
Assign legal hold policy

Assign a legal hold to a file, file version, folder, or user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_legal_hold_policy_assignments_request** | Option<[**PostLegalHoldPolicyAssignmentsRequest**](PostLegalHoldPolicyAssignmentsRequest.md)> |  |  |

### Return type

[**crate::models::LegalHoldPolicyAssignment**](LegalHoldPolicyAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

