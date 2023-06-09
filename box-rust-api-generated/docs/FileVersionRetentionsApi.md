# \FileVersionRetentionsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_file_version_retentions**](FileVersionRetentionsApi.md#get_file_version_retentions) | **GET** /file_version_retentions | List file version retentions
[**get_file_version_retentions_id**](FileVersionRetentionsApi.md#get_file_version_retentions_id) | **GET** /file_version_retentions/{file_version_retention_id} | Get retention on file



## get_file_version_retentions

> crate::models::FileVersionRetentions get_file_version_retentions(file_id, file_version_id, policy_id, disposition_action, disposition_before, disposition_after, limit, marker)
List file version retentions

Retrieves all file version retentions for the given enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | Option<**String**> | Filters results by files with this ID. |  |
**file_version_id** | Option<**String**> | Filters results by file versions with this ID. |  |
**policy_id** | Option<**String**> | Filters results by the retention policy with this ID. |  |
**disposition_action** | Option<**String**> | Filters results by the retention policy with this disposition action. |  |
**disposition_before** | Option<**String**> | Filters results by files that will have their disposition come into effect before this date. |  |
**disposition_after** | Option<**String**> | Filters results by files that will have their disposition come into effect after this date. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |

### Return type

[**crate::models::FileVersionRetentions**](FileVersionRetentions.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_version_retentions_id

> crate::models::FileVersionRetention get_file_version_retentions_id(file_version_retention_id)
Get retention on file

Returns information about a file version retention.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_version_retention_id** | **String** | The ID of the file version retention | [required] |

### Return type

[**crate::models::FileVersionRetention**](FileVersionRetention.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

