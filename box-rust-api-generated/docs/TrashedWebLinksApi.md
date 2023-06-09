# \TrashedWebLinksApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_web_links_id_trash**](TrashedWebLinksApi.md#delete_web_links_id_trash) | **DELETE** /web_links/{web_link_id}/trash | Permanently remove web link
[**get_web_links_id_trash**](TrashedWebLinksApi.md#get_web_links_id_trash) | **GET** /web_links/{web_link_id}/trash | Get trashed web link
[**post_web_links_id**](TrashedWebLinksApi.md#post_web_links_id) | **POST** /web_links/{web_link_id} | Restore web link



## delete_web_links_id_trash

> delete_web_links_id_trash(web_link_id)
Permanently remove web link

Permanently deletes a web link that is in the trash. This action cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_web_links_id_trash

> crate::models::TrashWebLink get_web_links_id_trash(web_link_id, fields)
Get trashed web link

Retrieves a web link that has been moved to the trash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::TrashWebLink**](TrashWebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_web_links_id

> crate::models::TrashWebLinkRestored post_web_links_id(web_link_id, fields, post_web_links_id_request)
Restore web link

Restores a web link that has been moved to the trash.  An optional new parent ID can be provided to restore the  web link to in case the original folder has been deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_web_links_id_request** | Option<[**PostWebLinksIdRequest**](PostWebLinksIdRequest.md)> |  |  |

### Return type

[**crate::models::TrashWebLinkRestored**](TrashWebLinkRestored.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

