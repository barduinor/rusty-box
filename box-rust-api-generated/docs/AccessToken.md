# AccessToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | Option<**String**> | The requested access token. | [optional]
**expires_in** | Option<**i64**> | The time in seconds in seconds by which this token will expire. | [optional]
**token_type** | Option<**String**> | The type of access token returned. | [optional]
**restricted_to** | Option<[**Vec<crate::models::FileScope>**](FileScope.md)> | The permissions that this access token permits, providing a list of resources (files, folders, etc) and the scopes permitted for each of those resources. | [optional]
**refresh_token** | Option<**String**> | The refresh token for this access token, which can be used to request a new access token when the current one expires. | [optional]
**issued_token_type** | Option<**String**> | The type of downscoped access token returned. This is only returned if an access token has been downscoped. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


