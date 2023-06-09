# \DevicePinnersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_device_pinners_id**](DevicePinnersApi.md#delete_device_pinners_id) | **DELETE** /device_pinners/{device_pinner_id} | Remove device pin
[**get_device_pinners_id**](DevicePinnersApi.md#get_device_pinners_id) | **GET** /device_pinners/{device_pinner_id} | Get device pin
[**get_enterprises_id_device_pinners**](DevicePinnersApi.md#get_enterprises_id_device_pinners) | **GET** /enterprises/{enterprise_id}/device_pinners | List enterprise device pins



## delete_device_pinners_id

> delete_device_pinners_id(device_pinner_id)
Remove device pin

Deletes an individual device pin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_pinner_id** | **String** | The ID of the device pin | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device_pinners_id

> crate::models::DevicePinner get_device_pinners_id(device_pinner_id)
Get device pin

Retrieves information about an individual device pin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_pinner_id** | **String** | The ID of the device pin | [required] |

### Return type

[**crate::models::DevicePinner**](DevicePinner.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enterprises_id_device_pinners

> crate::models::DevicePinners get_enterprises_id_device_pinners(enterprise_id, marker, limit, direction)
List enterprise device pins

Retrieves all the device pins within an enterprise.  The user must have admin privileges, and the application needs the \"manage enterprise\" scope to make this call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise_id** | **String** | The ID of the enterprise | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**direction** | Option<**String**> | The direction to sort results in. This can be either in alphabetical ascending (`ASC`) or descending (`DESC`) order. |  |

### Return type

[**crate::models::DevicePinners**](DevicePinners.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

