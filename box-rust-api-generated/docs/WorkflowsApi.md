# \WorkflowsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_workflows**](WorkflowsApi.md#get_workflows) | **GET** /workflows | List workflows
[**post_workflows_id_start**](WorkflowsApi.md#post_workflows_id_start) | **POST** /workflows/{workflow_id}/start | Starts workflow based on request body



## get_workflows

> crate::models::Workflows get_workflows(folder_id, trigger_type, limit, marker)
List workflows

Returns list of workflows that act on a given `folder ID`, and have a flow with a trigger type of `WORKFLOW_MANUAL_START`.  You application must be authorized to use the `Manage Box Relay` application scope within the developer console in to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**trigger_type** | Option<**String**> | Type of trigger to search for. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |

### Return type

[**crate::models::Workflows**](Workflows.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workflows_id_start

> post_workflows_id_start(workflow_id, post_workflows_id_start_request)
Starts workflow based on request body

Initiates a flow with a trigger type of `WORKFLOW_MANUAL_START`.  You application must be authorized to use the `Manage Box Relay` application scope within the developer console.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** | The ID of the workflow. | [required] |
**post_workflows_id_start_request** | Option<[**PostWorkflowsIdStartRequest**](PostWorkflowsIdStartRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

