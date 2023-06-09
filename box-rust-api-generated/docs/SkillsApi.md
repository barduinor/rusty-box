# \SkillsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_files_id_metadata_global_box_skills_cards**](SkillsApi.md#delete_files_id_metadata_global_box_skills_cards) | **DELETE** /files/{file_id}/metadata/global/boxSkillsCards | Remove Box Skill cards from file
[**get_files_id_metadata_global_box_skills_cards**](SkillsApi.md#get_files_id_metadata_global_box_skills_cards) | **GET** /files/{file_id}/metadata/global/boxSkillsCards | List Box Skill cards on file
[**post_files_id_metadata_global_box_skills_cards**](SkillsApi.md#post_files_id_metadata_global_box_skills_cards) | **POST** /files/{file_id}/metadata/global/boxSkillsCards | Create Box Skill cards on file
[**put_files_id_metadata_global_box_skills_cards**](SkillsApi.md#put_files_id_metadata_global_box_skills_cards) | **PUT** /files/{file_id}/metadata/global/boxSkillsCards | Update Box Skill cards on file
[**put_skill_invocations_id**](SkillsApi.md#put_skill_invocations_id) | **PUT** /skill_invocations/{skill_id} | Update all Box Skill cards on file



## delete_files_id_metadata_global_box_skills_cards

> delete_files_id_metadata_global_box_skills_cards(file_id)
Remove Box Skill cards from file

Removes any Box Skills cards metadata from a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_id_metadata_global_box_skills_cards

> crate::models::SkillCardsMetadata get_files_id_metadata_global_box_skills_cards(file_id)
List Box Skill cards on file

List the Box Skills metadata cards that are attached to a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |

### Return type

[**crate::models::SkillCardsMetadata**](SkillCardsMetadata.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_files_id_metadata_global_box_skills_cards

> crate::models::SkillCardsMetadata post_files_id_metadata_global_box_skills_cards(file_id, post_files_id_metadata_global_box_skills_cards_request)
Create Box Skill cards on file

Applies one or more Box Skills metadata cards to a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**post_files_id_metadata_global_box_skills_cards_request** | Option<[**PostFilesIdMetadataGlobalBoxSkillsCardsRequest**](PostFilesIdMetadataGlobalBoxSkillsCardsRequest.md)> |  |  |

### Return type

[**crate::models::SkillCardsMetadata**](SkillCardsMetadata.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_files_id_metadata_global_box_skills_cards

> crate::models::SkillCardsMetadata put_files_id_metadata_global_box_skills_cards(file_id, put_files_id_metadata_global_box_skills_cards_request_inner)
Update Box Skill cards on file

Updates one or more Box Skills metadata cards to a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The unique identifier that represents a file.  The ID for any file can be determined by visiting a file in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/files/123` the `file_id` is `123`. | [required] |
**put_files_id_metadata_global_box_skills_cards_request_inner** | Option<[**Vec<crate::models::PutFilesIdMetadataGlobalBoxSkillsCardsRequestInner>**](put_files_id_metadata_global_boxSkillsCards_request_inner.md)> |  |  |

### Return type

[**crate::models::SkillCardsMetadata**](SkillCardsMetadata.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_skill_invocations_id

> put_skill_invocations_id(skill_id, put_skill_invocations_id_request)
Update all Box Skill cards on file

An alternative method that can be used to overwrite and update all Box Skill metadata cards on a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | The ID of the skill to apply this metadata for. | [required] |
**put_skill_invocations_id_request** | Option<[**PutSkillInvocationsIdRequest**](PutSkillInvocationsIdRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

