# \GroupsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_groups_id**](GroupsApi.md#delete_groups_id) | **DELETE** /groups/{group_id} | Remove group
[**get_groups**](GroupsApi.md#get_groups) | **GET** /groups | List groups for enterprise
[**get_groups_id**](GroupsApi.md#get_groups_id) | **GET** /groups/{group_id} | Get group
[**post_groups**](GroupsApi.md#post_groups) | **POST** /groups | Create group
[**post_groups_terminate_sessions**](GroupsApi.md#post_groups_terminate_sessions) | **POST** /groups/terminate_sessions | Create jobs to terminate user group session
[**put_groups_id**](GroupsApi.md#put_groups_id) | **PUT** /groups/{group_id} | Update group



## delete_groups_id

> delete_groups_id(group_id)
Remove group

Permanently deletes a group. Only users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups

> crate::models::Groups get_groups(filter_term, fields, limit, offset)
List groups for enterprise

Retrieves all of the groups for a given enterprise. The user must have admin permissions to inspect enterprise's groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_term** | Option<**String**> | Limits the results to only groups whose `name` starts with the search term. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]

### Return type

[**crate::models::Groups**](Groups.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_id

> crate::models::GroupFull get_groups_id(group_id, fields)
Get group

Retrieves information about a group. Only members of this group or users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::GroupFull**](Group--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_groups

> crate::models::Group post_groups(fields, post_groups_request)
Create group

Creates a new group of users in an enterprise. Only users with admin permissions can create new groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_groups_request** | Option<[**PostGroupsRequest**](PostGroupsRequest.md)> |  |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_groups_terminate_sessions

> crate::models::SessionTerminationMessage post_groups_terminate_sessions(post_groups_terminate_sessions_request)
Create jobs to terminate user group session

Validates the roles and permissions of the group, and creates asynchronous jobs to terminate the group's sessions. Returns the status for the POST request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_groups_terminate_sessions_request** | Option<[**PostGroupsTerminateSessionsRequest**](PostGroupsTerminateSessionsRequest.md)> |  |  |

### Return type

[**crate::models::SessionTerminationMessage**](SessionTerminationMessage.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_groups_id

> crate::models::GroupFull put_groups_id(group_id, fields, put_groups_id_request)
Update group

Updates a specific group. Only admins of this group or users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**put_groups_id_request** | Option<[**PutGroupsIdRequest**](PutGroupsIdRequest.md)> |  |  |

### Return type

[**crate::models::GroupFull**](Group--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

