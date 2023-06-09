# \WebhooksApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_webhooks_id**](WebhooksApi.md#delete_webhooks_id) | **DELETE** /webhooks/{webhook_id} | Remove webhook
[**get_webhooks**](WebhooksApi.md#get_webhooks) | **GET** /webhooks | List all webhooks
[**get_webhooks_id**](WebhooksApi.md#get_webhooks_id) | **GET** /webhooks/{webhook_id} | Get webhook
[**post_webhooks**](WebhooksApi.md#post_webhooks) | **POST** /webhooks | Create webhook
[**put_webhooks_id**](WebhooksApi.md#put_webhooks_id) | **PUT** /webhooks/{webhook_id} | Update webhook



## delete_webhooks_id

> delete_webhooks_id(webhook_id)
Remove webhook

Deletes a webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> crate::models::Webhooks get_webhooks(marker, limit)
List all webhooks

Returns all defined webhooks for the requesting application.  This API only returns webhooks that are applied to files or folders that are owned by the authenticated user. This means that an admin can not see webhooks created by a service account unless the admin has access to those folders, and vice versa.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::Webhooks**](Webhooks.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks_id

> crate::models::Webhook get_webhooks_id(webhook_id)
Get webhook

Retrieves a specific webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook. | [required] |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webhooks

> crate::models::Webhook post_webhooks(post_webhooks_request)
Create webhook

Creates a webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_webhooks_request** | Option<[**PostWebhooksRequest**](PostWebhooksRequest.md)> |  |  |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_webhooks_id

> crate::models::Webhook put_webhooks_id(webhook_id, put_webhooks_id_request)
Update webhook

Updates a webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook. | [required] |
**put_webhooks_id_request** | Option<[**PutWebhooksIdRequest**](PutWebhooksIdRequest.md)> |  |  |

### Return type

[**crate::models::Webhook**](Webhook.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

