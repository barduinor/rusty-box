# SkillCardsMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_can_edit** | Option<**bool**> | Whether the user can edit this metadata | [optional]
**dollar_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A UUID to identify the metadata object | [optional]
**dollar_parent** | Option<**String**> | An ID for the parent folder | [optional]
**dollar_scope** | Option<**String**> | An ID for the scope in which this template has been applied | [optional]
**dollar_template** | Option<**String**> | The name of the template | [optional]
**dollar_type** | Option<**String**> | A unique identifier for the \"type\" of this instance. This is an internal system property and should not be used by a client application. | [optional]
**dollar_type_version** | Option<**i32**> | The last-known version of the template of the object. This is an internal system property and should not be used by a client application. | [optional]
**dollar_version** | Option<**i32**> | The version of the metadata object. Starts at 0 and increases every time a user-defined property is modified. | [optional]
**cards** | Option<[**Vec<crate::models::PutFilesIdMetadataGlobalBoxSkillsCardsRequestInnerValueAllOf>**](put_files_id_metadata_global_boxSkillsCards_request_inner_value_allOf.md)> | A list of Box Skill cards that have been applied to this file. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


