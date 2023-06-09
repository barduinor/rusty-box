# LegalHoldPolicies

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The limit that was used for these entries. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed. The maximum value varies by API. | [optional]
**next_marker** | Option<**i64**> | The marker for the start of the next page of results. | [optional]
**prev_marker** | Option<**i64**> | The marker for the start of the previous page of results. | [optional]
**entries** | Option<[**Vec<crate::models::LegalHoldPolicy>**](LegalHoldPolicy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


