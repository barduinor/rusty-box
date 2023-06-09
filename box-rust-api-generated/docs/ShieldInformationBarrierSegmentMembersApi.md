# \ShieldInformationBarrierSegmentMembersApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_shield_information_barrier_segment_members_id**](ShieldInformationBarrierSegmentMembersApi.md#delete_shield_information_barrier_segment_members_id) | **DELETE** /shield_information_barrier_segment_members/{shield_information_barrier_segment_member_id} | Delete shield information barrier segment member by ID
[**get_shield_information_barrier_segment_members**](ShieldInformationBarrierSegmentMembersApi.md#get_shield_information_barrier_segment_members) | **GET** /shield_information_barrier_segment_members | List shield information barrier segment members
[**get_shield_information_barrier_segment_members_id**](ShieldInformationBarrierSegmentMembersApi.md#get_shield_information_barrier_segment_members_id) | **GET** /shield_information_barrier_segment_members/{shield_information_barrier_segment_member_id} | Get shield information barrier segment member by ID
[**post_shield_information_barrier_segment_members**](ShieldInformationBarrierSegmentMembersApi.md#post_shield_information_barrier_segment_members) | **POST** /shield_information_barrier_segment_members | Create shield information barrier segment member



## delete_shield_information_barrier_segment_members_id

> delete_shield_information_barrier_segment_members_id(shield_information_barrier_segment_member_id)
Delete shield information barrier segment member by ID

Deletes a shield information barrier segment member based on provided ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_member_id** | **String** | The ID of the shield information barrier segment Member. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shield_information_barrier_segment_members

> crate::models::GetShieldInformationBarrierSegmentMembers200Response get_shield_information_barrier_segment_members(shield_information_barrier_segment_id, marker, limit)
List shield information barrier segment members

Lists shield information barrier segment members based on provided segment IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_id** | **String** | The ID of the shield information barrier segment. | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::GetShieldInformationBarrierSegmentMembers200Response**](get_shield_information_barrier_segment_members_200_response.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shield_information_barrier_segment_members_id

> crate::models::ShieldInformationBarrierSegmentMember get_shield_information_barrier_segment_members_id(shield_information_barrier_segment_member_id)
Get shield information barrier segment member by ID

Retrieves a shield information barrier segment member by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_segment_member_id** | **String** | The ID of the shield information barrier segment Member. | [required] |

### Return type

[**crate::models::ShieldInformationBarrierSegmentMember**](ShieldInformationBarrierSegmentMember.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_shield_information_barrier_segment_members

> crate::models::ShieldInformationBarrierSegmentMember post_shield_information_barrier_segment_members(post_shield_information_barrier_segment_members_request)
Create shield information barrier segment member

Creates a new shield information barrier segment member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_shield_information_barrier_segment_members_request** | Option<[**PostShieldInformationBarrierSegmentMembersRequest**](PostShieldInformationBarrierSegmentMembersRequest.md)> |  |  |

### Return type

[**crate::models::ShieldInformationBarrierSegmentMember**](ShieldInformationBarrierSegmentMember.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

