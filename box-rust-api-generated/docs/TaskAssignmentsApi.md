# \TaskAssignmentsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_task_assignments_id**](TaskAssignmentsApi.md#delete_task_assignments_id) | **DELETE** /task_assignments/{task_assignment_id} | Unassign task
[**get_task_assignments_id**](TaskAssignmentsApi.md#get_task_assignments_id) | **GET** /task_assignments/{task_assignment_id} | Get task assignment
[**get_tasks_id_assignments**](TaskAssignmentsApi.md#get_tasks_id_assignments) | **GET** /tasks/{task_id}/assignments | List task assignments
[**post_task_assignments**](TaskAssignmentsApi.md#post_task_assignments) | **POST** /task_assignments | Assign task
[**put_task_assignments_id**](TaskAssignmentsApi.md#put_task_assignments_id) | **PUT** /task_assignments/{task_assignment_id} | Update task assignment



## delete_task_assignments_id

> delete_task_assignments_id(task_assignment_id)
Unassign task

Deletes a specific task assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_assignment_id** | **String** | The ID of the task assignment. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task_assignments_id

> crate::models::TaskAssignment get_task_assignments_id(task_assignment_id)
Get task assignment

Retrieves information about a task assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_assignment_id** | **String** | The ID of the task assignment. | [required] |

### Return type

[**crate::models::TaskAssignment**](TaskAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tasks_id_assignments

> crate::models::TaskAssignments get_tasks_id_assignments(task_id)
List task assignments

Lists all of the assignments for a given task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the task. | [required] |

### Return type

[**crate::models::TaskAssignments**](TaskAssignments.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_task_assignments

> crate::models::TaskAssignment post_task_assignments(post_task_assignments_request)
Assign task

Assigns a task to a user.  A task can be assigned to more than one user by creating multiple assignments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_task_assignments_request** | Option<[**PostTaskAssignmentsRequest**](PostTaskAssignmentsRequest.md)> |  |  |

### Return type

[**crate::models::TaskAssignment**](TaskAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_task_assignments_id

> crate::models::TaskAssignment put_task_assignments_id(task_assignment_id, put_task_assignments_id_request)
Update task assignment

Updates a task assignment. This endpoint can be used to update the state of a task assigned to a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_assignment_id** | **String** | The ID of the task assignment. | [required] |
**put_task_assignments_id_request** | Option<[**PutTaskAssignmentsIdRequest**](PutTaskAssignmentsIdRequest.md)> |  |  |

### Return type

[**crate::models::TaskAssignment**](TaskAssignment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

