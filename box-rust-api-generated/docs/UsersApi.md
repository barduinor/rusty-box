# \UsersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_users_id**](UsersApi.md#delete_users_id) | **DELETE** /users/{user_id} | Delete user
[**get_users**](UsersApi.md#get_users) | **GET** /users | List enterprise users
[**get_users_id**](UsersApi.md#get_users_id) | **GET** /users/{user_id} | Get user
[**get_users_me**](UsersApi.md#get_users_me) | **GET** /users/me | Get current user
[**post_users**](UsersApi.md#post_users) | **POST** /users | Create user
[**post_users_terminate_sessions**](UsersApi.md#post_users_terminate_sessions) | **POST** /users/terminate_sessions | Create jobs to terminate users session
[**put_users_id**](UsersApi.md#put_users_id) | **PUT** /users/{user_id} | Update user



## delete_users_id

> delete_users_id(user_id, notify, force)
Delete user

Deletes a user. By default this will fail if the user still owns any content. Move their owned content first before proceeding, or use the `force` field to delete the user and their files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**notify** | Option<**bool**> | Whether the user will receive email notification of the deletion |  |
**force** | Option<**bool**> | Whether the user should be deleted even if this user still own files |  |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> crate::models::Users get_users(filter_term, user_type, external_app_user_id, fields, offset, limit, usemarker, marker)
List enterprise users

Returns a list of all users for the Enterprise along with their `user_id`, `public_name`, and `login`.  The application and the authenticated user need to have the permission to look up users in the entire enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_term** | Option<**String**> | Limits the results to only users who's `name` or `login` start with the search term.  For externally managed users, the search term needs to completely match the in order to find the user, and it will only return one user at a time. |  |
**user_type** | Option<**String**> | Limits the results to the kind of user specified.  * `all` returns every kind of user for whom the   `login` or `name` partially matches the   `filter_term`. It will only return an external user   if the login matches the `filter_term` completely,   and in that case it will only return that user. * `managed` returns all managed and app users for whom   the `login` or `name` partially matches the   `filter_term`. * `external` returns all external users for whom the   `login` matches the `filter_term` exactly. |  |
**external_app_user_id** | Option<**String**> | Limits the results to app users with the given `external_app_user_id` value.  When creating an app user, an `external_app_user_id` value can be set. This value can then be used in this endpoint to find any users that match that `external_app_user_id` value. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**usemarker** | Option<**bool**> | Specifies whether to use marker-based pagination instead of offset-based pagination. Only one pagination method can be used at a time.  By setting this value to true, the API will return a `marker` field that can be passed as a parameter to this endpoint to get the next page of the response. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |

### Return type

[**crate::models::Users**](Users.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_id

> crate::models::UserFull get_users_id(user_id, fields)
Get user

Retrieves information about a user in the enterprise.  The application and the authenticated user need to have the permission to look up users in the entire enterprise.  This endpoint also returns a limited set of information for external users who are collaborated on content owned by the enterprise for authenticated users with the right scopes. In this case, disallowed fields will return null instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::UserFull**](User--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_me

> crate::models::UserFull get_users_me(fields)
Get current user

Retrieves information about the user who is currently authenticated.  In the case of a client-side authenticated OAuth 2.0 application this will be the user who authorized the app.  In the case of a JWT, server-side authenticated application this will be the service account that belongs to the application by default.  Use the `As-User` header to change who this API call is made on behalf of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::UserFull**](User--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users

> crate::models::User post_users(fields, post_users_request)
Create user

Creates a new managed user in an enterprise. This endpoint is only available to users and applications with the right admin permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_users_request** | Option<[**PostUsersRequest**](PostUsersRequest.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users_terminate_sessions

> crate::models::SessionTerminationMessage post_users_terminate_sessions(post_users_terminate_sessions_request)
Create jobs to terminate users session

Validates the roles and permissions of the user, and creates asynchronous jobs to terminate the user's sessions. Returns the status for the POST request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_users_terminate_sessions_request** | Option<[**PostUsersTerminateSessionsRequest**](PostUsersTerminateSessionsRequest.md)> |  |  |

### Return type

[**crate::models::SessionTerminationMessage**](SessionTerminationMessage.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_users_id

> crate::models::UserFull put_users_id(user_id, fields, put_users_id_request)
Update user

Updates a managed or app user in an enterprise. This endpoint is only available to users and applications with the right admin permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**put_users_id_request** | Option<[**PutUsersIdRequest**](PutUsersIdRequest.md)> |  |  |

### Return type

[**crate::models::UserFull**](User--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

