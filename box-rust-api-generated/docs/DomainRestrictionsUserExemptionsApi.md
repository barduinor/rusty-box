# \DomainRestrictionsUserExemptionsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_collaboration_whitelist_exempt_targets_id**](DomainRestrictionsUserExemptionsApi.md#delete_collaboration_whitelist_exempt_targets_id) | **DELETE** /collaboration_whitelist_exempt_targets/{collaboration_whitelist_exempt_target_id} | Remove user from list of users exempt from domain restrictions
[**get_collaboration_whitelist_exempt_targets**](DomainRestrictionsUserExemptionsApi.md#get_collaboration_whitelist_exempt_targets) | **GET** /collaboration_whitelist_exempt_targets | List users exempt from collaboration domain restrictions
[**get_collaboration_whitelist_exempt_targets_id**](DomainRestrictionsUserExemptionsApi.md#get_collaboration_whitelist_exempt_targets_id) | **GET** /collaboration_whitelist_exempt_targets/{collaboration_whitelist_exempt_target_id} | Get user exempt from collaboration domain restrictions
[**post_collaboration_whitelist_exempt_targets**](DomainRestrictionsUserExemptionsApi.md#post_collaboration_whitelist_exempt_targets) | **POST** /collaboration_whitelist_exempt_targets | Create user exemption from collaboration domain restrictions



## delete_collaboration_whitelist_exempt_targets_id

> delete_collaboration_whitelist_exempt_targets_id(collaboration_whitelist_exempt_target_id)
Remove user from list of users exempt from domain restrictions

Removes a user's exemption from the restrictions set out by the allowed list of domains for collaborations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collaboration_whitelist_exempt_target_id** | **String** | The ID of the exemption to the list. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collaboration_whitelist_exempt_targets

> crate::models::CollaborationAllowlistExemptTargets get_collaboration_whitelist_exempt_targets(marker, limit)
List users exempt from collaboration domain restrictions

Returns a list of users who have been exempt from the collaboration domain restrictions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::CollaborationAllowlistExemptTargets**](CollaborationAllowlistExemptTargets.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collaboration_whitelist_exempt_targets_id

> crate::models::CollaborationAllowlistExemptTarget get_collaboration_whitelist_exempt_targets_id(collaboration_whitelist_exempt_target_id)
Get user exempt from collaboration domain restrictions

Returns a users who has been exempt from the collaboration domain restrictions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collaboration_whitelist_exempt_target_id** | **String** | The ID of the exemption to the list. | [required] |

### Return type

[**crate::models::CollaborationAllowlistExemptTarget**](CollaborationAllowlistExemptTarget.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_collaboration_whitelist_exempt_targets

> crate::models::CollaborationAllowlistExemptTarget post_collaboration_whitelist_exempt_targets(post_collaboration_whitelist_exempt_targets_request)
Create user exemption from collaboration domain restrictions

Exempts a user from the restrictions set out by the allowed list of domains for collaborations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_collaboration_whitelist_exempt_targets_request** | Option<[**PostCollaborationWhitelistExemptTargetsRequest**](PostCollaborationWhitelistExemptTargetsRequest.md)> |  |  |

### Return type

[**crate::models::CollaborationAllowlistExemptTarget**](CollaborationAllowlistExemptTarget.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

