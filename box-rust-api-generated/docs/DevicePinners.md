# DevicePinners

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entries** | Option<[**Vec<crate::models::DevicePinner>**](DevicePinner.md)> |  | [optional]
**limit** | Option<**i64**> | The limit that was used for these entries. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed. | [optional][default to 100]
**next_marker** | Option<**i64**> | The marker for the start of the next page of results. | [optional]
**order** | Option<[**Vec<crate::models::DevicePinnersOrderInner>**](DevicePinners_order_inner.md)> | The order by which items are returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


