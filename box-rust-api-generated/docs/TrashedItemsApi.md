# \TrashedItemsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_folders_trash_items**](TrashedItemsApi.md#get_folders_trash_items) | **GET** /folders/trash/items | List trashed items



## get_folders_trash_items

> crate::models::Items get_folders_trash_items(fields, limit, offset, usemarker, marker, direction, sort)
List trashed items

Retrieves the files and folders that have been moved to the trash.  Any attribute in the full files or folders objects can be passed in with the `fields` parameter to retrieve those specific attributes that are not returned by default.  This endpoint defaults to use offset-based pagination, yet also supports marker-based pagination using the `marker` parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]
**usemarker** | Option<**bool**> | Specifies whether to use marker-based pagination instead of offset-based pagination. Only one pagination method can be used at a time.  By setting this value to true, the API will return a `marker` field that can be passed as a parameter to this endpoint to get the next page of the response. |  |
**marker** | Option<**String**> | Defines the position marker at which to begin returning results. This is used when paginating using marker-based pagination.  This requires `usemarker` to be set to `true`. |  |
**direction** | Option<**String**> | The direction to sort results in. This can be either in alphabetical ascending (`ASC`) or descending (`DESC`) order. |  |
**sort** | Option<**String**> | Defines the **second** attribute by which items are sorted.  Items are always sorted by their `type` first, with folders listed before files, and files listed before web links.  This parameter is not supported when using marker-based pagination. |  |

### Return type

[**crate::models::Items**](Items.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

