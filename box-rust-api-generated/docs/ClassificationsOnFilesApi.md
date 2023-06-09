# \ClassificationsOnFilesApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo**](ClassificationsOnFilesApi.md#delete_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo) | **DELETE** /files/{file_id}/metadata/enterprise/securityClassification-6VMVochwUWo | Remove classification from file
[**get_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo**](ClassificationsOnFilesApi.md#get_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo) | **GET** /files/{file_id}/metadata/enterprise/securityClassification-6VMVochwUWo | Get classification on file
[**post_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo**](ClassificationsOnFilesApi.md#post_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo) | **POST** /files/{file_id}/metadata/enterprise/securityClassification-6VMVochwUWo | Add classification to file
[**put_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo**](ClassificationsOnFilesApi.md#put_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo) | **PUT** /files/{file_id}/metadata/enterprise/securityClassification-6VMVochwUWo | Update classification on file



## delete_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo

> delete_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo(file_id)
Remove classification from file

Removes any classifications from a file.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/files/:id//enterprise_12345/securityClassification-6VMVochwUWo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo

> crate::models::Classification get_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo(file_id)
Get classification on file

Retrieves the classification metadata instance that has been applied to a file.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/files/:id//enterprise_12345/securityClassification-6VMVochwUWo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |

### Return type

[**crate::models::Classification**](Classification.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo

> crate::models::Classification post_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo(file_id, post_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo_request)
Add classification to file

Adds a classification to a file by specifying the label of the classification to add.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/files/:id//enterprise_12345/securityClassification-6VMVochwUWo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**post_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo_request** | Option<[**PostFilesIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequest**](PostFilesIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequest.md)> |  |  |

### Return type

[**crate::models::Classification**](Classification.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo

> crate::models::Classification put_files_id_metadata_enterprise_security_classification6_vm_vochw_uwo(file_id, put_files_id_metadata_enterprise_security_classification_6_vm_vochw_uwo_request_inner)
Update classification on file

Updates a classification on a file.  The classification can only be updated if a classification has already been applied to the file before. When editing classifications, only values are defined for the enterprise will be accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**put_files_id_metadata_enterprise_security_classification_6_vm_vochw_uwo_request_inner** | Option<[**Vec<crate::models::PutFilesIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequestInner>**](put_files_id_metadata_enterprise_securityClassification_6VMVochwUWo_request_inner.md)> |  |  |

### Return type

[**crate::models::Classification**](Classification.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

