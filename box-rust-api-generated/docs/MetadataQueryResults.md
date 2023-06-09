# MetadataQueryResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entries** | Option<[**Vec<crate::models::SkillInvocationSourceAllOf>**](SkillInvocation_source_allOf.md)> | The mini representation of the files and folders that match the search terms.  By default, this endpoint returns only the most basic info about the items. To get additional fields for each item, including any of the metadata, use the `fields` attribute in the query. | [optional]
**limit** | Option<**i64**> | The limit that was used for this search. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed. | [optional][default to 100]
**next_marker** | Option<**String**> | The marker for the start of the next page of results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


