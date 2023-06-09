# ExpiringEmbedLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | Option<**String**> | The requested access token. | [optional]
**expires_in** | Option<**i64**> | The time in seconds in seconds by which this token will expire. | [optional]
**token_type** | Option<**String**> | The type of access token returned. | [optional]
**restricted_to** | Option<[**Vec<crate::models::FileScope>**](FileScope.md)> | The permissions that this access token permits, providing a list of resources (files, folders, etc) and the scopes permitted for each of those resources. | [optional]
**url** | Option<**String**> | The actual expiring embed URL for this file, constructed from the file ID and access tokens specified in this object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


