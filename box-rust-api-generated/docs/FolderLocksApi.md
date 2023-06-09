# \FolderLocksApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_folder_locks_id**](FolderLocksApi.md#delete_folder_locks_id) | **DELETE** /folder_locks/{folder_lock_id} | Delete folder lock
[**get_folder_locks**](FolderLocksApi.md#get_folder_locks) | **GET** /folder_locks | List folder locks
[**post_folder_locks**](FolderLocksApi.md#post_folder_locks) | **POST** /folder_locks | Create folder lock



## delete_folder_locks_id

> delete_folder_locks_id(folder_lock_id)
Delete folder lock

Deletes a folder lock on a given folder.  You must be authenticated as the owner or co-owner of the folder to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_lock_id** | **String** | The ID of the folder lock. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folder_locks

> crate::models::FolderLocks get_folder_locks(folder_id)
List folder locks

Retrieves folder lock details for a given folder.  You must be authenticated as the owner or co-owner of the folder to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |

### Return type

[**crate::models::FolderLocks**](FolderLocks.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_folder_locks

> crate::models::FolderLock post_folder_locks(post_folder_locks_request)
Create folder lock

Creates a folder lock on a folder, preventing it from being moved and/or deleted.  You must be authenticated as the owner or co-owner of the folder to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_folder_locks_request** | Option<[**PostFolderLocksRequest**](PostFolderLocksRequest.md)> |  |  |

### Return type

[**crate::models::FolderLock**](FolderLock.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

