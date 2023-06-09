# \IntegrationMappingsSlackApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_integration_mappings_slack_id**](IntegrationMappingsSlackApi.md#delete_integration_mappings_slack_id) | **DELETE** /integration_mappings/slack/{integration_mapping_id} | Delete Slack integration mapping
[**get_integration_mappings_slack**](IntegrationMappingsSlackApi.md#get_integration_mappings_slack) | **GET** /integration_mappings/slack | List Slack integration mappings
[**post_integration_mappings_slack**](IntegrationMappingsSlackApi.md#post_integration_mappings_slack) | **POST** /integration_mappings/slack | Create Slack integration mapping
[**put_integration_mappings_slack_id**](IntegrationMappingsSlackApi.md#put_integration_mappings_slack_id) | **PUT** /integration_mappings/slack/{integration_mapping_id} | Update Slack integration mapping



## delete_integration_mappings_slack_id

> delete_integration_mappings_slack_id(integration_mapping_id)
Delete Slack integration mapping

Deletes a [Slack integration mapping](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack).   You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_id** | **String** | An ID of an integration mapping | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integration_mappings_slack

> crate::models::IntegrationMappings get_integration_mappings_slack(marker, limit, partner_item_type, partner_item_id, box_item_id, box_item_type, is_manually_created)
List Slack integration mappings

Lists [Slack integration mappings](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack) in a users' enterprise.  You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**partner_item_type** | Option<**String**> | Mapped item type, for which the mapping should be returned |  |
**partner_item_id** | Option<**String**> | ID of the mapped item, for which the mapping should be returned |  |
**box_item_id** | Option<**String**> | Box item ID, for which the mappings should be returned |  |
**box_item_type** | Option<**String**> | Box item type, for which the mappings should be returned |  |
**is_manually_created** | Option<**bool**> | Whether the mapping has been manually created |  |

### Return type

[**crate::models::IntegrationMappings**](IntegrationMappings.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integration_mappings_slack

> crate::models::IntegrationMapping post_integration_mappings_slack(integration_mapping_slack_create_request)
Create Slack integration mapping

Creates a [Slack integration mapping](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack) by mapping a Slack channel to a Box item.  You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_slack_create_request** | Option<[**IntegrationMappingSlackCreateRequest**](IntegrationMappingSlackCreateRequest.md)> |  |  |

### Return type

[**crate::models::IntegrationMapping**](IntegrationMapping.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_integration_mappings_slack_id

> crate::models::IntegrationMapping put_integration_mappings_slack_id(integration_mapping_id, put_integration_mappings_slack_id_request)
Update Slack integration mapping

Updates a [Slack integration mapping](https://support.box.com/hc/en-us/articles/4415585987859-Box-as-the-Content-Layer-for-Slack). Supports updating the Box folder ID and options.  You need Admin or Co-Admin role to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_mapping_id** | **String** | An ID of an integration mapping | [required] |
**put_integration_mappings_slack_id_request** | Option<[**PutIntegrationMappingsSlackIdRequest**](PutIntegrationMappingsSlackIdRequest.md)> | At least one of `box_item` and `options` must be provided. |  |

### Return type

[**crate::models::IntegrationMapping**](IntegrationMapping.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

