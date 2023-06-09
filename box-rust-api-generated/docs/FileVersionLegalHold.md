# FileVersionLegalHold

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this file version legal hold | [optional]
**r#type** | Option<**String**> | `file_version_legal_hold` | [optional]
**file_version** | Option<[**crate::models::FileVersionLegalHoldFileVersion**](FileVersionLegalHold_file_version.md)> |  | [optional]
**file** | Option<[**crate::models::FileVersionLegalHoldFile**](FileVersionLegalHold_file.md)> |  | [optional]
**legal_hold_policy_assignments** | Option<[**Vec<crate::models::LegalHoldPolicyAssignment>**](LegalHoldPolicyAssignment.md)> | List of assignments contributing to this Hold. | [optional]
**deleted_at** | Option<**String**> | Time that this File-Version-Legal-Hold was deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


