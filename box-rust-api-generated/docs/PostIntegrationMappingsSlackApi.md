# \PostIntegrationMappingsSlackApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_integration_mappings_slack**](PostIntegrationMappingsSlackApi.md#post_integration_mappings_slack) | **POST** /integration_mappings/slack | Create Slack integration mapping



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

