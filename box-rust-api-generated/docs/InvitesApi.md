# \InvitesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_invites_id**](InvitesApi.md#get_invites_id) | **GET** /invites/{invite_id} | Get user invite status
[**post_invites**](InvitesApi.md#post_invites) | **POST** /invites | Create user invite



## get_invites_id

> crate::models::Invite get_invites_id(invite_id, fields)
Get user invite status

Returns the status of a user invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_id** | **String** | The ID of an invite. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::Invite**](Invite.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_invites

> crate::models::Invite post_invites(fields, post_invites_request)
Create user invite

Invites an existing external user to join an enterprise.  The existing user can not be part of another enterprise and must already have a Box account. Once invited, the user will receive an email and are prompted to accept the invitation within the Box web application.  This method requires the \"Manage An Enterprise\" scope enabled for the application, which can be enabled within the developer console.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_invites_request** | Option<[**PostInvitesRequest**](PostInvitesRequest.md)> |  |  |

### Return type

[**crate::models::Invite**](Invite.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

