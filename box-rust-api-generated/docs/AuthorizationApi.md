# \AuthorizationApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_authorize**](AuthorizationApi.md#get_authorize) | **GET** /authorize | Authorize user
[**post_oauth2_revoke**](AuthorizationApi.md#post_oauth2_revoke) | **POST** /oauth2/revoke | Revoke access token
[**post_oauth2_token**](AuthorizationApi.md#post_oauth2_token) | **POST** /oauth2/token | Request access token
[**post_oauth2_token_refresh**](AuthorizationApi.md#post_oauth2_token_refresh) | **POST** /oauth2/token#refresh | Refresh access token



## get_authorize

> String get_authorize(response_type, client_id, redirect_uri, state, scope)
Authorize user

Authorize a user by sending them through the [Box](https://box.com) website and request their permission to act on their behalf.  This is the first step when authenticating a user using OAuth 2.0. To request a user's authorization to use the Box APIs on their behalf you will need to send a user to the URL with this format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_type** | **String** | The type of response we'd like to receive. | [required] |
**client_id** | **String** | The Client ID of the application that is requesting to authenticate the user. To get the Client ID for your application, log in to your Box developer console and click the **Edit Application** link for the application you're working with. In the OAuth 2.0 Parameters section of the configuration page, find the item labelled `client_id`. The text of that item is your application's Client ID. | [required] |
**redirect_uri** | Option<**String**> | The URI to which Box redirects the browser after the user has granted or denied the application permission. This URI match one of the redirect URIs in the configuration of your application. It must be a valid HTTPS URI and it needs to be able to handle the redirection to complete the next step in the OAuth 2.0 flow. Although this parameter is optional, it must be a part of the authorization URL if you configured multiple redirect URIs for the application in the developer console. A missing parameter causes a `redirect_uri_missing` error after the user grants application access. |  |
**state** | Option<**String**> | A custom string of your choice. Box will pass the same string to the redirect URL when authentication is complete. This parameter can be used to identify a user on redirect, as well as protect against hijacked sessions and other exploits. |  |
**scope** | Option<**String**> | A comma-separated list of application scopes you'd like to authenticate the user for. This defaults to all the scopes configured for the application in its configuration page. |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_oauth2_revoke

> post_oauth2_revoke(client_id, client_secret, token)
Revoke access token

Revoke an active Access Token, effectively logging a user out that has been previously authenticated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | Option<**String**> | The Client ID of the application requesting to revoke the access token. |  |
**client_secret** | Option<**String**> | The client secret of the application requesting to revoke an access token. |  |
**token** | Option<**String**> | The access token to revoke. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_oauth2_token

> crate::models::AccessToken post_oauth2_token(grant_type, client_id, client_secret, code, refresh_token, assertion, subject_token, subject_token_type, actor_token, actor_token_type, scope, resource, box_subject_type, box_subject_id, box_shared_link)
Request access token

Request an Access Token using either a client-side obtained OAuth 2.0 authorization code or a server-side JWT assertion.  An Access Token is a string that enables Box to verify that a request belongs to an authorized session. In the normal order of operations you will begin by requesting authentication from the [authorize](#get-authorize) endpoint and Box will send you an authorization code.  You will then send this code to this endpoint to exchange it for an Access Token. The returned Access Token can then be used to to make Box API calls.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | The type of request being made, either using a client-side obtained authorization code, a refresh token, a JWT assertion, client credentials grant or another access token for the purpose of downscoping a token. | [required] |
**client_id** | Option<**String**> | The Client ID of the application requesting an access token.  Used in combination with `authorization_code`, `client_credentials`, or `urn:ietf:params:oauth:grant-type:jwt-bearer` as the `grant_type`. |  |
**client_secret** | Option<**String**> | The client secret of the application requesting an access token.  Used in combination with `authorization_code`, `client_credentials`, or `urn:ietf:params:oauth:grant-type:jwt-bearer` as the `grant_type`. |  |
**code** | Option<**String**> | The client-side authorization code passed to your application by Box in the browser redirect after the user has successfully granted your application permission to make API calls on their behalf.  Used in combination with `authorization_code` as the `grant_type`. |  |
**refresh_token** | Option<**String**> | A refresh token used to get a new access token with.  Used in combination with `refresh_token` as the `grant_type`. |  |
**assertion** | Option<**String**> | A JWT assertion for which to request a new access token.  Used in combination with `urn:ietf:params:oauth:grant-type:jwt-bearer` as the `grant_type`. |  |
**subject_token** | Option<**String**> | The token to exchange for a downscoped token. This can be a regular access token, a JWT assertion, or an app token.  Used in combination with `urn:ietf:params:oauth:grant-type:token-exchange` as the `grant_type`. |  |
**subject_token_type** | Option<**String**> | The type of `subject_token` passed in.  Used in combination with `urn:ietf:params:oauth:grant-type:token-exchange` as the `grant_type`. |  |
**actor_token** | Option<**String**> | The token used to create an annotator token. This is a JWT assertion.  Used in combination with `urn:ietf:params:oauth:grant-type:token-exchange` as the `grant_type`. |  |
**actor_token_type** | Option<**String**> | The type of `actor_token` passed in.  Used in combination with `urn:ietf:params:oauth:grant-type:token-exchange` as the `grant_type`. |  |
**scope** | Option<**String**> | The space-delimited list of scopes that you want apply to the new access token.  The `subject_token` will need to have all of these scopes or the call will error with **401 Unauthorized**. |  |
**resource** | Option<**String**> | Full URL for the file that the token should be generated for. |  |
**box_subject_type** | Option<**String**> | Used in combination with `client_credentials` as the `grant_type`. |  |
**box_subject_id** | Option<**String**> | Used in combination with `client_credentials` as the `grant_type`. Value is determined by `box_subject_type`. If `user` use user ID and if `enterprise` use enterprise ID. |  |
**box_shared_link** | Option<**String**> | Full URL of the shared link on the file or folder that the token should be generated for. |  |

### Return type

[**crate::models::AccessToken**](AccessToken.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_oauth2_token_refresh

> crate::models::AccessToken post_oauth2_token_refresh(grant_type, client_id, client_secret, refresh_token)
Refresh access token

Refresh an Access Token using its client ID, secret, and refresh token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | The type of request being made, in this case a refresh request. | [required] |
**client_id** | **String** | The client ID of the application requesting to refresh the token. | [required] |
**client_secret** | **String** | The client secret of the application requesting to refresh the token. | [required] |
**refresh_token** | **String** | The refresh token to refresh. | [required] |

### Return type

[**crate::models::AccessToken**](AccessToken.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

