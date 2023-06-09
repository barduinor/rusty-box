# \DomainRestrictionsForCollaborationsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_collaboration_whitelist_entries_id**](DomainRestrictionsForCollaborationsApi.md#delete_collaboration_whitelist_entries_id) | **DELETE** /collaboration_whitelist_entries/{collaboration_whitelist_entry_id} | Remove domain from list of allowed collaboration domains
[**get_collaboration_whitelist_entries**](DomainRestrictionsForCollaborationsApi.md#get_collaboration_whitelist_entries) | **GET** /collaboration_whitelist_entries | List allowed collaboration domains
[**get_collaboration_whitelist_entries_id**](DomainRestrictionsForCollaborationsApi.md#get_collaboration_whitelist_entries_id) | **GET** /collaboration_whitelist_entries/{collaboration_whitelist_entry_id} | Get allowed collaboration domain
[**post_collaboration_whitelist_entries**](DomainRestrictionsForCollaborationsApi.md#post_collaboration_whitelist_entries) | **POST** /collaboration_whitelist_entries | Add domain to list of allowed collaboration domains



## delete_collaboration_whitelist_entries_id

> delete_collaboration_whitelist_entries_id(collaboration_whitelist_entry_id)
Remove domain from list of allowed collaboration domains

Removes a domain from the list of domains that have been deemed safe to create collaborations for within the current enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collaboration_whitelist_entry_id** | **String** | The ID of the entry in the list. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collaboration_whitelist_entries

> crate::models::CollaborationAllowlistEntries get_collaboration_whitelist_entries(marker, limit)
List allowed collaboration domains

Returns the list domains that have been deemed safe to create collaborations for within the current enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::CollaborationAllowlistEntries**](CollaborationAllowlistEntries.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collaboration_whitelist_entries_id

> crate::models::CollaborationAllowlistEntry get_collaboration_whitelist_entries_id(collaboration_whitelist_entry_id)
Get allowed collaboration domain

Returns a domain that has been deemed safe to create collaborations for within the current enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collaboration_whitelist_entry_id** | **String** | The ID of the entry in the list. | [required] |

### Return type

[**crate::models::CollaborationAllowlistEntry**](CollaborationAllowlistEntry.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_collaboration_whitelist_entries

> crate::models::CollaborationAllowlistEntry post_collaboration_whitelist_entries(post_collaboration_whitelist_entries_request)
Add domain to list of allowed collaboration domains

Creates a new entry in the list of allowed domains to allow collaboration for.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_collaboration_whitelist_entries_request** | Option<[**PostCollaborationWhitelistEntriesRequest**](PostCollaborationWhitelistEntriesRequest.md)> |  |  |

### Return type

[**crate::models::CollaborationAllowlistEntry**](CollaborationAllowlistEntry.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

