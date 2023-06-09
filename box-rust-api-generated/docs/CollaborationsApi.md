# \CollaborationsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_collaborations_id**](CollaborationsApi.md#delete_collaborations_id) | **DELETE** /collaborations/{collaboration_id} | Remove collaboration
[**get_collaborations_id**](CollaborationsApi.md#get_collaborations_id) | **GET** /collaborations/{collaboration_id} | Get collaboration
[**post_collaborations**](CollaborationsApi.md#post_collaborations) | **POST** /collaborations | Create collaboration
[**put_collaborations_id**](CollaborationsApi.md#put_collaborations_id) | **PUT** /collaborations/{collaboration_id} | Update collaboration



## delete_collaborations_id

> delete_collaborations_id(collaboration_id)
Remove collaboration

Deletes a single collaboration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collaboration_id** | **String** | The ID of the collaboration | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collaborations_id

> crate::models::Collaboration get_collaborations_id(collaboration_id, fields)
Get collaboration

Retrieves a single collaboration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collaboration_id** | **String** | The ID of the collaboration | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::Collaboration**](Collaboration.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_collaborations

> crate::models::Collaboration post_collaborations(fields, notify, post_collaborations_request)
Create collaboration

Adds a collaboration for a single user or a single group to a file or folder.  Collaborations can be created using email address, user IDs, or a group IDs.  If a collaboration is being created with a group, access to this endpoint is dependent on the group's ability to be invited.  If collaboration is in `pending` status, the following fields are redacted: - `login` and `name` are hidden if a collaboration was created using `user_id`, -  `name` is hidden if a collaboration was created using `login`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**notify** | Option<**bool**> | Determines if users should receive email notification for the action performed. |  |
**post_collaborations_request** | Option<[**PostCollaborationsRequest**](PostCollaborationsRequest.md)> |  |  |

### Return type

[**crate::models::Collaboration**](Collaboration.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_collaborations_id

> crate::models::Collaboration put_collaborations_id(collaboration_id, put_collaborations_id_request)
Update collaboration

Updates a collaboration. Can be used to change the owner of an item, or to accept collaboration invites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collaboration_id** | **String** | The ID of the collaboration | [required] |
**put_collaborations_id_request** | Option<[**PutCollaborationsIdRequest**](PutCollaborationsIdRequest.md)> |  |  |

### Return type

[**crate::models::Collaboration**](Collaboration.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

