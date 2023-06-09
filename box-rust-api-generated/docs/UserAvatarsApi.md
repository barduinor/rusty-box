# \UserAvatarsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_users_id_avatar**](UserAvatarsApi.md#delete_users_id_avatar) | **DELETE** /users/{user_id}/avatar | Delete user avatar
[**get_users_id_avatar**](UserAvatarsApi.md#get_users_id_avatar) | **GET** /users/{user_id}/avatar | Get user avatar
[**post_users_id_avatar**](UserAvatarsApi.md#post_users_id_avatar) | **POST** /users/{user_id}/avatar | Add or update user avatar



## delete_users_id_avatar

> delete_users_id_avatar(user_id)
Delete user avatar

Removes an existing user avatar. You cannot reverse this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_id_avatar

> std::path::PathBuf get_users_id_avatar(user_id)
Get user avatar

Retrieves an image of a the user's avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpg, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users_id_avatar

> crate::models::UserAvatar post_users_id_avatar(user_id, pic)
Add or update user avatar

Adds or updates a user avatar.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**pic** | **std::path::PathBuf** | The image file to be uploaded to Box. Accepted file extensions are `.jpg` or `.png`. The maximum file size is 1MB. | [required] |

### Return type

[**crate::models::UserAvatar**](UserAvatar.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

