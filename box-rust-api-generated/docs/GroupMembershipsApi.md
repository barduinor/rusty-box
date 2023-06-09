# \GroupMembershipsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_group_memberships_id**](GroupMembershipsApi.md#delete_group_memberships_id) | **DELETE** /group_memberships/{group_membership_id} | Remove user from group
[**get_group_memberships_id**](GroupMembershipsApi.md#get_group_memberships_id) | **GET** /group_memberships/{group_membership_id} | Get group membership
[**get_groups_id_memberships**](GroupMembershipsApi.md#get_groups_id_memberships) | **GET** /groups/{group_id}/memberships | List members of group
[**get_users_id_memberships**](GroupMembershipsApi.md#get_users_id_memberships) | **GET** /users/{user_id}/memberships | List user's groups
[**post_group_memberships**](GroupMembershipsApi.md#post_group_memberships) | **POST** /group_memberships | Add user to group
[**put_group_memberships_id**](GroupMembershipsApi.md#put_group_memberships_id) | **PUT** /group_memberships/{group_membership_id} | Update group membership



## delete_group_memberships_id

> delete_group_memberships_id(group_membership_id)
Remove user from group

Deletes a specific group membership. Only admins of this group or users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_membership_id** | **String** | The ID of the group membership. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_memberships_id

> crate::models::GroupMembership get_group_memberships_id(group_membership_id, fields)
Get group membership

Retrieves a specific group membership. Only admins of this group or users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_membership_id** | **String** | The ID of the group membership. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::GroupMembership**](GroupMembership.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_id_memberships

> crate::models::GroupMemberships get_groups_id_memberships(group_id, limit, offset)
List members of group

Retrieves all the members for a group. Only members of this group or users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group. | [required] |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]

### Return type

[**crate::models::GroupMemberships**](GroupMemberships.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_id_memberships

> crate::models::GroupMemberships get_users_id_memberships(user_id, limit, offset)
List user's groups

Retrieves all the groups for a user. Only members of this group or users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]

### Return type

[**crate::models::GroupMemberships**](GroupMemberships.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_group_memberships

> crate::models::GroupMembership post_group_memberships(fields, post_group_memberships_request)
Add user to group

Creates a group membership. Only users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_group_memberships_request** | Option<[**PostGroupMembershipsRequest**](PostGroupMembershipsRequest.md)> |  |  |

### Return type

[**crate::models::GroupMembership**](GroupMembership.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_group_memberships_id

> crate::models::GroupMembership put_group_memberships_id(group_membership_id, fields, put_group_memberships_id_request)
Update group membership

Updates a user's group membership. Only admins of this group or users with admin-level permissions will be able to use this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_membership_id** | **String** | The ID of the group membership. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**put_group_memberships_id_request** | Option<[**PutGroupMembershipsIdRequest**](PutGroupMembershipsIdRequest.md)> |  |  |

### Return type

[**crate::models::GroupMembership**](GroupMembership.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

