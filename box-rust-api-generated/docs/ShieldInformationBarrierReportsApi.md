# \ShieldInformationBarrierReportsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_shield_information_barrier_reports**](ShieldInformationBarrierReportsApi.md#get_shield_information_barrier_reports) | **GET** /shield_information_barrier_reports | List shield information barrier reports
[**get_shield_information_barrier_reports_id**](ShieldInformationBarrierReportsApi.md#get_shield_information_barrier_reports_id) | **GET** /shield_information_barrier_reports/{shield_information_barrier_report_id} | Get shield information barrier report by ID
[**post_shield_information_barrier_reports**](ShieldInformationBarrierReportsApi.md#post_shield_information_barrier_reports) | **POST** /shield_information_barrier_reports | Create shield information barrier report



## get_shield_information_barrier_reports

> crate::models::GetShieldInformationBarrierReports200Response get_shield_information_barrier_reports(shield_information_barrier_id, marker, limit)
List shield information barrier reports

Lists shield information barrier reports with specific IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_id** | **String** | The ID of the shield information barrier. | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::GetShieldInformationBarrierReports200Response**](get_shield_information_barrier_reports_200_response.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shield_information_barrier_reports_id

> crate::models::ShieldInformationBarrierReport get_shield_information_barrier_reports_id(shield_information_barrier_report_id)
Get shield information barrier report by ID

Retrieves a shield information barrier report by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_report_id** | **String** | The ID of the shield information barrier Report. | [required] |

### Return type

[**crate::models::ShieldInformationBarrierReport**](ShieldInformationBarrierReport.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_shield_information_barrier_reports

> crate::models::ShieldInformationBarrierReport post_shield_information_barrier_reports(shield_information_barrier_reference)
Create shield information barrier report

Creates a shield information barrier report for a given barrier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shield_information_barrier_reference** | Option<[**ShieldInformationBarrierReference**](ShieldInformationBarrierReference.md)> |  |  |

### Return type

[**crate::models::ShieldInformationBarrierReport**](ShieldInformationBarrierReport.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

