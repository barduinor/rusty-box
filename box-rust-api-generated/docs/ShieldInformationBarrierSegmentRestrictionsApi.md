# \ShieldInformationBarrierSegmentRestrictionsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_shield_information_barrier_segment_restrictions_id**](ShieldInformationBarrierSegmentRestrictionsApi.md#delete_shield_information_barrier_segment_restrictions_id) | **DELETE** /shield_information_barrier_segment_restrictions/{shield_information_barrier_segment_restriction_id} | Delete shield information barrier segment restriction by ID
[**get_shield_information_barrier_segment_restrictions**](ShieldInformationBarrierSegmentRestrictionsApi.md#get_shield_information_barrier_segment_restrictions) | **GET** /shield_information_barrier_segment_restrictions | List shield information barrier segment restrictions
[**get_shield_information_barrier_segment_restrictions_id**](ShieldInformationBarrierSegmentRestrictionsApi.md#get_shield_information_barrier_segment_restrictions_id) | **GET** /shield_information_barrier_segment_restrictions/{shield_information_barrier_segment_restriction_id} | Get shield information barrier segment restriction by ID
[**post_shield_information_barrier_segment_restrictions**](ShieldInformationBarrierSegmentRestrictionsApi.md#post_shield_information_barrier_segment_restrictions) | **POST** /shield_information_barrier_segment_restrictions | Create shield information barrier segment restriction



## delete_shield_information_barrier_segment_restrictions_id

> delete_shield_information_barrier_segment_restrictions_id(shield_information_barrier_segment_restriction_id)
Delete shield information barrier segment restriction by ID

Delete shield information barrier segment restriction based on provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_restriction_id** | **String** | The ID of the shield information barrier segment Restriction. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shield_information_barrier_segment_restrictions

> crate::models::GetShieldInformationBarrierSegmentRestrictions200Response get_shield_information_barrier_segment_restrictions(shield_information_barrier_segment_id, marker, limit)
List shield information barrier segment restrictions

Lists shield information barrier segment restrictions based on provided segment ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_id** | **String** | The ID of the shield information barrier segment. | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::GetShieldInformationBarrierSegmentRestrictions200Response**](get_shield_information_barrier_segment_restrictions_200_response.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shield_information_barrier_segment_restrictions_id

> crate::models::ShieldInformationBarrierSegmentRestriction get_shield_information_barrier_segment_restrictions_id(shield_information_barrier_segment_restriction_id)
Get shield information barrier segment restriction by ID

Retrieves a shield information barrier segment restriction based on provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_restriction_id** | **String** | The ID of the shield information barrier segment Restriction. | [required] |

### Return type

[**crate::models::ShieldInformationBarrierSegmentRestriction**](ShieldInformationBarrierSegmentRestriction.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_shield_information_barrier_segment_restrictions

> crate::models::ShieldInformationBarrierSegmentRestriction post_shield_information_barrier_segment_restrictions(post_shield_information_barrier_segment_restrictions_request)
Create shield information barrier segment restriction

Creates a shield information barrier segment restriction object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_shield_information_barrier_segment_restrictions_request** | Option<[**PostShieldInformationBarrierSegmentRestrictionsRequest**](PostShieldInformationBarrierSegmentRestrictionsRequest.md)> |  |  |

### Return type

[**crate::models::ShieldInformationBarrierSegmentRestriction**](ShieldInformationBarrierSegmentRestriction.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

