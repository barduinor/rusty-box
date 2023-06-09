# \TasksApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tasks_id**](TasksApi.md#delete_tasks_id) | **DELETE** /tasks/{task_id} | Remove task
[**get_files_id_tasks**](TasksApi.md#get_files_id_tasks) | **GET** /files/{file_id}/tasks | List tasks on file
[**get_tasks_id**](TasksApi.md#get_tasks_id) | **GET** /tasks/{task_id} | Get task
[**post_tasks**](TasksApi.md#post_tasks) | **POST** /tasks | Create task
[**put_tasks_id**](TasksApi.md#put_tasks_id) | **PUT** /tasks/{task_id} | Update task



## delete_tasks_id

> delete_tasks_id(task_id)
Remove task

Removes a task from a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the task. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_tasks

> crate::models::Tasks get_files_id_tasks(file_id)
List tasks on file

Retrieves a list of all the tasks for a file. This endpoint does not support pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |

### Return type

[**crate::models::Tasks**](Tasks.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tasks_id

> crate::models::Task get_tasks_id(task_id)
Get task

Retrieves information about a specific task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the task. | [required] |

### Return type

[**crate::models::Task**](Task.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tasks

> crate::models::Task post_tasks(post_tasks_request)
Create task

Creates a single task on a file. This task is not assigned to any user and will need to be assigned separately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_tasks_request** | Option<[**PostTasksRequest**](PostTasksRequest.md)> |  |  |

### Return type

[**crate::models::Task**](Task.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_tasks_id

> crate::models::Task put_tasks_id(task_id, put_tasks_id_request)
Update task

Updates a task. This can be used to update a task's configuration, or to update its completion state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The ID of the task. | [required] |
**put_tasks_id_request** | Option<[**PutTasksIdRequest**](PutTasksIdRequest.md)> |  |  |

### Return type

[**crate::models::Task**](Task.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

