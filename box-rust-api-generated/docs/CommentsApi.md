# \CommentsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_comments_id**](CommentsApi.md#delete_comments_id) | **DELETE** /comments/{comment_id} | Remove comment
[**get_comments_id**](CommentsApi.md#get_comments_id) | **GET** /comments/{comment_id} | Get comment
[**get_files_id_comments**](CommentsApi.md#get_files_id_comments) | **GET** /files/{file_id}/comments | List file comments
[**post_comments**](CommentsApi.md#post_comments) | **POST** /comments | Create comment
[**put_comments_id**](CommentsApi.md#put_comments_id) | **PUT** /comments/{comment_id} | Update comment



## delete_comments_id

> delete_comments_id(comment_id)
Remove comment

Permanently deletes a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_id

> crate::models::CommentFull get_comments_id(comment_id, fields)
Get comment

Retrieves the message and metadata for a specific comment, as well as information on the user who created the comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |

### Return type

[**crate::models::CommentFull**](Comment--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_comments

> crate::models::Comments get_files_id_comments(file_id, fields, limit, offset)
List file comments

Retrieves a list of comments for a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**limit** | Option<**i64**> | The maximum number of items to return per page. |  |
**offset** | Option<**i64**> | The offset of the item at which to begin the response.  Queries with offset parameter value exceeding 10000 will be rejected with a 400 response. |  |[default to 0]

### Return type

[**crate::models::Comments**](Comments.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_comments

> crate::models::Comment post_comments(fields, post_comments_request)
Create comment

Adds a comment by the user to a specific file, or as a reply to an other comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**post_comments_request** | Option<[**PostCommentsRequest**](PostCommentsRequest.md)> |  |  |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_comments_id

> crate::models::CommentFull put_comments_id(comment_id, fields, put_comments_id_request)
Update comment

Update the message of a comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A comma-separated list of attributes to include in the response. This can be used to request fields that are not normally returned in a standard response.  Be aware that specifying this parameter will have the effect that none of the standard fields are returned in the response unless explicitly specified, instead only fields for the mini representation are returned, additional to the fields requested. |  |
**put_comments_id_request** | Option<[**PutCommentsIdRequest**](PutCommentsIdRequest.md)> |  |  |

### Return type

[**crate::models::CommentFull**](Comment--Full.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

