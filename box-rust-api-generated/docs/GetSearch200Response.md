# GetSearch200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_count** | Option<**i64**> | One greater than the offset of the last entry in the search results. The total number of entries in the collection may be less than `total_count`. | [optional]
**limit** | Option<**i64**> | The limit that was used for this search. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed. | [optional]
**offset** | Option<**i64**> | The 0-based offset of the first entry in this set. This will be the same as the `offset` query parameter used. | [optional]
**entries** | Option<[**Vec<crate::models::SearchResultWithSharedLink>**](SearchResultWithSharedLink.md)> | The search results for the query provided, including the additional information about any shared links through which the item has been shared with the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


