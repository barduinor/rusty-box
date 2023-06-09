# \ClassificationsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema**](ClassificationsApi.md#delete_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema) | **DELETE** /metadata_templates/enterprise/securityClassification-6VMVochwUWo/schema | Delete all classifications
[**get_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema**](ClassificationsApi.md#get_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema) | **GET** /metadata_templates/enterprise/securityClassification-6VMVochwUWo/schema | List all classifications
[**post_metadata_templates_schema_classifications**](ClassificationsApi.md#post_metadata_templates_schema_classifications) | **POST** /metadata_templates/schema#classifications | Add initial classifications
[**put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_add**](ClassificationsApi.md#put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_add) | **PUT** /metadata_templates/enterprise/securityClassification-6VMVochwUWo/schema#add | Add classification
[**put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_delete**](ClassificationsApi.md#put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_delete) | **PUT** /metadata_templates/enterprise/securityClassification-6VMVochwUWo/schema#delete | Delete classification
[**put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_update**](ClassificationsApi.md#put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_update) | **PUT** /metadata_templates/enterprise/securityClassification-6VMVochwUWo/schema#update | Update classification



## delete_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema

> delete_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema()
Delete all classifications

Delete all classifications by deleting the classification metadata template.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema

> crate::models::ClassificationTemplate get_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema()
List all classifications

Retrieves the classification metadata template and lists all the classifications available to this enterprise.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/metadata_templates/enterprise_12345/securityClassification-6VMVochwUWo/schema`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClassificationTemplate**](ClassificationTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_metadata_templates_schema_classifications

> crate::models::ClassificationTemplate post_metadata_templates_schema_classifications(post_metadata_templates_schema_classifications_request)
Add initial classifications

When an enterprise does not yet have any classifications, this API call initializes the classification template with an initial set of classifications.  If an enterprise already has a classification, the template will already exist and instead an API call should be made to add additional classifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_metadata_templates_schema_classifications_request** | Option<[**PostMetadataTemplatesSchemaClassificationsRequest**](PostMetadataTemplatesSchemaClassificationsRequest.md)> |  |  |

### Return type

[**crate::models::ClassificationTemplate**](ClassificationTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_add

> crate::models::ClassificationTemplate put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_add(put_metadata_templates_enterprise_security_classification_6_vm_vochw_uwo_schema_add_request_inner)
Add classification

Adds one or more new classifications to the list of classifications available to the enterprise.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/metadata_templates/enterprise_12345/securityClassification-6VMVochwUWo/schema`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_metadata_templates_enterprise_security_classification_6_vm_vochw_uwo_schema_add_request_inner** | Option<[**Vec<crate::models::PutMetadataTemplatesEnterpriseSecurityClassification6VmVochwUwoSchemaAddRequestInner>**](put_metadata_templates_enterprise_securityClassification_6VMVochwUWo_schema_add_request_inner.md)> |  |  |

### Return type

[**crate::models::ClassificationTemplate**](ClassificationTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_delete

> crate::models::ClassificationTemplate put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_delete(put_metadata_templates_enterprise_security_classification_6_vm_vochw_uwo_schema_delete_request_inner)
Delete classification

Removes a classification from the list of classifications available to the enterprise.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/metadata_templates/enterprise_12345/securityClassification-6VMVochwUWo/schema`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_metadata_templates_enterprise_security_classification_6_vm_vochw_uwo_schema_delete_request_inner** | Option<[**Vec<crate::models::PutMetadataTemplatesEnterpriseSecurityClassification6VmVochwUwoSchemaDeleteRequestInner>**](put_metadata_templates_enterprise_securityClassification_6VMVochwUWo_schema_delete_request_inner.md)> |  |  |

### Return type

[**crate::models::ClassificationTemplate**](ClassificationTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_update

> crate::models::ClassificationTemplate put_metadata_templates_enterprise_security_classification6_vm_vochw_uwo_schema_update(put_metadata_templates_enterprise_security_classification_6_vm_vochw_uwo_schema_update_request_inner)
Update classification

Updates the labels and descriptions of one or more classifications available to the enterprise.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/metadata_templates/enterprise_12345/securityClassification-6VMVochwUWo/schema`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_metadata_templates_enterprise_security_classification_6_vm_vochw_uwo_schema_update_request_inner** | Option<[**Vec<crate::models::PutMetadataTemplatesEnterpriseSecurityClassification6VmVochwUwoSchemaUpdateRequestInner>**](put_metadata_templates_enterprise_securityClassification_6VMVochwUWo_schema_update_request_inner.md)> |  |  |

### Return type

[**crate::models::ClassificationTemplate**](ClassificationTemplate.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

