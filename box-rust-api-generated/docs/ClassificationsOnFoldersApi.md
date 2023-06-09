# \ClassificationsOnFoldersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo**](ClassificationsOnFoldersApi.md#delete_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo) | **DELETE** /folders/{folder_id}/metadata/enterprise/securityClassification-6VMVochwUWo | Remove classification from folder
[**get_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo**](ClassificationsOnFoldersApi.md#get_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo) | **GET** /folders/{folder_id}/metadata/enterprise/securityClassification-6VMVochwUWo | Get classification on folder
[**post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo**](ClassificationsOnFoldersApi.md#post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo) | **POST** /folders/{folder_id}/metadata/enterprise/securityClassification-6VMVochwUWo | Add classification to folder
[**put_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo**](ClassificationsOnFoldersApi.md#put_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo) | **PUT** /folders/{folder_id}/metadata/enterprise/securityClassification-6VMVochwUWo | Update classification on folder



## delete_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo

> delete_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo(folder_id)
Remove classification from folder

Removes any classifications from a folder.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/folders/:id//enterprise_12345/securityClassification-6VMVochwUWo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo

> crate::models::Classification get_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo(folder_id)
Get classification on folder

Retrieves the classification metadata instance that has been applied to a folder.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/folders/:id//enterprise_12345/securityClassification-6VMVochwUWo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |

### Return type

[**crate::models::Classification**](Classification.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo

> crate::models::Classification post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo(folder_id, post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo_request)
Add classification to folder

Adds a classification to a folder by specifying the label of the classification to add.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/folders/:id//enterprise_12345/securityClassification-6VMVochwUWo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo_request** | Option<[**PostFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequest**](PostFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequest.md)> |  |  |

### Return type

[**crate::models::Classification**](Classification.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo

> crate::models::Classification put_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo(folder_id, put_folders_id_metadata_enterprise_security_classification_6_vm_vochw_uwo_request_inner)
Update classification on folder

Updates a classification on a folder.  The classification can only be updated if a classification has already been applied to the folder before. When editing classifications, only values are defined for the enterprise will be accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting this folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folder/123` the `folder_id` is `123`.  The root folder of a Box account is always represented by the ID `0`. | [required] |
**put_folders_id_metadata_enterprise_security_classification_6_vm_vochw_uwo_request_inner** | Option<[**Vec<crate::models::PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequestInner>**](put_folders_id_metadata_enterprise_securityClassification_6VMVochwUWo_request_inner.md)> |  |  |

### Return type

[**crate::models::Classification**](Classification.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

