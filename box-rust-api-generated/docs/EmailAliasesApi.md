# \EmailAliasesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_users_id_email_aliases_id**](EmailAliasesApi.md#delete_users_id_email_aliases_id) | **DELETE** /users/{user_id}/email_aliases/{email_alias_id} | Remove email alias
[**get_users_id_email_aliases**](EmailAliasesApi.md#get_users_id_email_aliases) | **GET** /users/{user_id}/email_aliases | List user's email aliases
[**post_users_id_email_aliases**](EmailAliasesApi.md#post_users_id_email_aliases) | **POST** /users/{user_id}/email_aliases | Create email alias



## delete_users_id_email_aliases_id

> delete_users_id_email_aliases_id(user_id, email_alias_id)
Remove email alias

Removes an email alias from a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**email_alias_id** | **String** | The ID of the email alias. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_id_email_aliases

> crate::models::EmailAliases get_users_id_email_aliases(user_id)
List user's email aliases

Retrieves all email aliases for a user. The collection does not include the primary login for the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |

### Return type

[**crate::models::EmailAliases**](EmailAliases.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users_id_email_aliases

> crate::models::EmailAlias post_users_id_email_aliases(user_id, post_users_id_email_aliases_request)
Create email alias

Adds a new email alias to a user account..

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**post_users_id_email_aliases_request** | Option<[**PostUsersIdEmailAliasesRequest**](PostUsersIdEmailAliasesRequest.md)> |  |  |

### Return type

[**crate::models::EmailAlias**](EmailAlias.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

