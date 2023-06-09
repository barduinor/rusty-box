# FileVersionRetention

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this file version retention. | [optional]
**r#type** | Option<**String**> | `file_version_retention` | [optional]
**file_version** | Option<[**crate::models::FileVersionRetentionFileVersion**](FileVersionRetention_file_version.md)> |  | [optional]
**file** | Option<[**crate::models::FileVersionRetentionFile**](FileVersionRetention_file.md)> |  | [optional]
**applied_at** | Option<**String**> | When this file version retention object was created | [optional]
**disposition_at** | Option<**String**> | When the retention expires on this file version retention | [optional]
**winning_retention_policy** | Option<[**crate::models::FileVersionRetentionWinningRetentionPolicy**](FileVersionRetention_winning_retention_policy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


