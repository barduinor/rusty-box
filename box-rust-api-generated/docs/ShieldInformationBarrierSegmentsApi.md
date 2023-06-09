# \ShieldInformationBarrierSegmentsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_shield_information_barrier_segments_id**](ShieldInformationBarrierSegmentsApi.md#delete_shield_information_barrier_segments_id) | **DELETE** /shield_information_barrier_segments/{shield_information_barrier_segment_id} | Delete shield information barrier segment
[**get_shield_information_barrier_segments**](ShieldInformationBarrierSegmentsApi.md#get_shield_information_barrier_segments) | **GET** /shield_information_barrier_segments | List shield information barrier segments
[**get_shield_information_barrier_segments_id**](ShieldInformationBarrierSegmentsApi.md#get_shield_information_barrier_segments_id) | **GET** /shield_information_barrier_segments/{shield_information_barrier_segment_id} | Get shield information barrier segment with specified ID
[**post_shield_information_barrier_segments**](ShieldInformationBarrierSegmentsApi.md#post_shield_information_barrier_segments) | **POST** /shield_information_barrier_segments | Create shield information barrier segment
[**put_shield_information_barrier_segments_id**](ShieldInformationBarrierSegmentsApi.md#put_shield_information_barrier_segments_id) | **PUT** /shield_information_barrier_segments/{shield_information_barrier_segment_id} | Update shield information barrier segment with specified ID



## delete_shield_information_barrier_segments_id

> delete_shield_information_barrier_segments_id(shield_information_barrier_segment_id)
Delete shield information barrier segment

Deletes the shield information barrier segment based on provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_id** | **String** | The ID of the shield information barrier segment. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shield_information_barrier_segments

> crate::models::GetShieldInformationBarrierSegments200Response get_shield_information_barrier_segments(shield_information_barrier_id, marker, limit)
List shield information barrier segments

Retrieves a list of shield information barrier segment objects for the specified Information Barrier ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_id** | **String** | The ID of the shield information barrier. | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::GetShieldInformationBarrierSegments200Response**](get_shield_information_barrier_segments_200_response.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shield_information_barrier_segments_id

> crate::models::ShieldInformationBarrierSegment get_shield_information_barrier_segments_id(shield_information_barrier_segment_id)
Get shield information barrier segment with specified ID

Retrieves shield information barrier segment based on provided ID..

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_id** | **String** | The ID of the shield information barrier segment. | [required] |

### Return type

[**crate::models::ShieldInformationBarrierSegment**](ShieldInformationBarrierSegment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_shield_information_barrier_segments

> crate::models::ShieldInformationBarrierSegment post_shield_information_barrier_segments(post_shield_information_barrier_segments_request)
Create shield information barrier segment

Creates a shield information barrier segment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_shield_information_barrier_segments_request** | Option<[**PostShieldInformationBarrierSegmentsRequest**](PostShieldInformationBarrierSegmentsRequest.md)> |  |  |

### Return type

[**crate::models::ShieldInformationBarrierSegment**](ShieldInformationBarrierSegment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_shield_information_barrier_segments_id

> crate::models::ShieldInformationBarrierSegment put_shield_information_barrier_segments_id(shield_information_barrier_segment_id, put_shield_information_barrier_segments_id_request)
Update shield information barrier segment with specified ID

Updates the shield information barrier segment based on provided ID..

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_id** | **String** | The ID of the shield information barrier segment. | [required] |
**put_shield_information_barrier_segments_id_request** | Option<[**PutShieldInformationBarrierSegmentsIdRequest**](PutShieldInformationBarrierSegmentsIdRequest.md)> |  |  |

### Return type

[**crate::models::ShieldInformationBarrierSegment**](ShieldInformationBarrierSegment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

