# \WebLinksApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_web_links_id**](WebLinksApi.md#delete_web_links_id) | **DELETE** /web_links/{web_link_id} | Remove web link
[**get_web_links_id**](WebLinksApi.md#get_web_links_id) | **GET** /web_links/{web_link_id} | Get web link
[**post_web_links**](WebLinksApi.md#post_web_links) | **POST** /web_links | Create web link
[**put_web_links_id**](WebLinksApi.md#put_web_links_id) | **PUT** /web_links/{web_link_id} | Update web link



## delete_web_links_id

> delete_web_links_id(web_link_id)
Remove web link

Deletes a web link.

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


## get_web_links_id

> crate::models::WebLink get_web_links_id(web_link_id, boxapi)
Get web link

Retrieve information about a web link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |
**boxapi** | Option<**String**> | The URL, and optional password, for the shared link of this item.  This header can be used to access items that have not been explicitly shared with a user.  Use the format `shared_link=[link]` or if a password is required then use `shared_link=[link]&shared_link_password=[password]`.  This header can be used on the file or folder shared, as well as on any files or folders nested within the item. |  |

### Return type

[**crate::models::WebLink**](WebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_web_links

> crate::models::WebLink post_web_links(post_web_links_request)
Create web link

Creates a web link object within a folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_web_links_request** | Option<[**PostWebLinksRequest**](PostWebLinksRequest.md)> |  |  |

### Return type

[**crate::models::WebLink**](WebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_web_links_id

> crate::models::WebLink put_web_links_id(web_link_id, put_web_links_id_request)
Update web link

Updates a web link object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**web_link_id** | **String** | The ID of the web link. | [required] |
**put_web_links_id_request** | Option<[**PutWebLinksIdRequest**](PutWebLinksIdRequest.md)> |  |  |

### Return type

[**crate::models::WebLink**](WebLink.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

