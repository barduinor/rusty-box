# \SharedLinksWebLinksApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_shared_items_web_links**](SharedLinksWebLinksApi.md#get_shared_items_web_links) | **GET** /shared_items#web_links | Find web link for shared link
[**get_web_links_id_get_shared_link**](SharedLinksWebLinksApi.md#get_web_links_id_get_shared_link) | **GET** /web_links/{web_link_id}#get_shared_link | Get shared link for web link
[**put_web_links_id_add_shared_link**](SharedLinksWebLinksApi.md#put_web_links_id_add_shared_link) | **PUT** /web_links/{web_link_id}#add_shared_link | Add shared link to web link
[**put_web_links_id_remove_shared_link**](SharedLinksWebLinksApi.md#put_web_links_id_remove_shared_link) | **PUT** /web_links/{web_link_id}#remove_shared_link | Remove shared link from web link
[**put_web_links_id_update_shared_link**](SharedLinksWebLinksApi.md#put_web_links_id_update_shared_link) | **PUT** /web_links/{web_link_id}#update_shared_link | Update shared link on web link



## get_shared_items_web_links

> crate::models::WebLink get_shared_items_web_links(boxapi, if_none_match, fields)
Find web link for shared link

Returns the web link represented by a shared link.  A shared web link can be represented by a shared link, which can originate within the current enterprise or within another.  This endpoint allows an application to retrieve information about a shared web link when only given a shared link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**boxapi** | **String** | A header containing the shared link and optional password for the shared link.  The format for this header is as follows.  `shared_link=[link]&shared_link_password=[password]` | [required] |
**if_none_match** | Option<**String**> | Ensures an item is only returned if it has changed.  Pass in the item's last observed `etag` value into this header and the endpoint will fail with a `304 Not Modified` if the item has not changed since. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::WebLink**](WebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_web_links_id_get_shared_link

> crate::models::WebLink get_web_links_id_get_shared_link(web_link_id, fields)
Get shared link for web link

Gets the information for a shared link on a web link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |

### Return type

[**crate::models::WebLink**](WebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_web_links_id_add_shared_link

> crate::models::WebLink put_web_links_id_add_shared_link(web_link_id, fields, put_web_links_id_add_shared_link_request)
Add shared link to web link

Adds a shared link to a web link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_web_links_id_add_shared_link_request** | Option<[**PutWebLinksIdAddSharedLinkRequest**](PutWebLinksIdAddSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::WebLink**](WebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_web_links_id_remove_shared_link

> crate::models::WebLink put_web_links_id_remove_shared_link(web_link_id, fields, put_web_links_id_remove_shared_link_request)
Remove shared link from web link

Removes a shared link from a web link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_web_links_id_remove_shared_link_request** | Option<[**PutWebLinksIdRemoveSharedLinkRequest**](PutWebLinksIdRemoveSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::WebLink**](WebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_web_links_id_update_shared_link

> crate::models::WebLink put_web_links_id_update_shared_link(web_link_id, fields, put_web_links_id_update_shared_link_request)
Update shared link on web link

Updates a shared link on a web link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |
**fields** | **String** | Explicitly request the `shared_link` fields to be returned for this item. | [required] |
**put_web_links_id_update_shared_link_request** | Option<[**PutWebLinksIdUpdateSharedLinkRequest**](PutWebLinksIdUpdateSharedLinkRequest.md)> |  |  |

### Return type

[**crate::models::WebLink**](WebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

