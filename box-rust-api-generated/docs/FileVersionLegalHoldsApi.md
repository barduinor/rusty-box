# \FileVersionLegalHoldsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_file_version_legal_holds**](FileVersionLegalHoldsApi.md#get_file_version_legal_holds) | **GET** /file_version_legal_holds | List file version legal holds
[**get_file_version_legal_holds_id**](FileVersionLegalHoldsApi.md#get_file_version_legal_holds_id) | **GET** /file_version_legal_holds/{file_version_legal_hold_id} | Get file version legal hold



## get_file_version_legal_holds

> crate::models::FileVersionLegalHolds get_file_version_legal_holds(policy_id, marker, limit)
List file version legal holds

Get a list of file versions on legal hold for a legal hold assignment.  Due to ongoing re-architecture efforts this API might not return all file versions for this policy ID.  Instead, this API will only return file versions held in the legacy architecture. Two new endpoints will available to request any file versions held in the new architecture.  For file versions held in the new architecture, the `GET /legal_hold_policy_assignments/:id/file_versions_on_hold` API can be used to return all past file versions available for this policy assignment, and the `GET /legal_hold_policy_assignments/:id/files_on_hold` API can be used to return any current (latest) versions of a file under legal hold.  The `GET /legal_hold_policy_assignments?policy_id={id}` API can be used to find a list of policy assignments for a given policy ID.  Once the re-architecture is completed this API will be deprecated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the legal hold policy to get the file version legal holds for. | [required] |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |

### Return type

[**crate::models::FileVersionLegalHolds**](FileVersionLegalHolds.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_version_legal_holds_id

> crate::models::FileVersionLegalHold get_file_version_legal_holds_id(file_version_legal_hold_id)
Get file version legal hold

Retrieves information about the legal hold policies assigned to a file version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_version_legal_hold_id** | **String** | The ID of the file version legal hold | [required] |

### Return type

[**crate::models::FileVersionLegalHold**](FileVersionLegalHold.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

