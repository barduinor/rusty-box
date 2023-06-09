# PostMetadataTemplatesSchemaClassificationsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | **String** | The scope in which to create the classifications. This should be `enterprise` or `enterprise_{id}` where `id` is the unique ID of the enterprise. | 
**template_key** | Option<**String**> | `securityClassification-6VMVochwUWo` | [optional]
**display_name** | **String** | `Classification` | 
**hidden** | Option<**bool**> | `false` | [optional]
**copy_instance_on_item_copy** | Option<**bool**> | `false` | [optional]
**fields** | Option<[**Vec<crate::models::PostMetadataTemplatesSchemaClassificationsRequestFieldsInner>**](post_metadata_templates_schema_classifications_request_fields_inner.md)> | The classification template holds one field, which holds all the valid classification values. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


