# ClassificationTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the classification template. | [optional]
**r#type** | **String** | `metadata_template` | 
**scope** | Option<**String**> | The scope of the classification template. This is in the format `enterprise_{id}` where the `id` is the enterprise ID. | [optional]
**template_key** | Option<**String**> | `securityClassification-6VMVochwUWo` | [optional]
**display_name** | Option<**String**> | The name of this template as shown in web and mobile interfaces. | [optional]
**hidden** | Option<**bool**> | This template is always available in web and mobile interfaces. | [optional]
**copy_instance_on_item_copy** | Option<**bool**> | Classifications are always copied along when the file or folder is copied. | [optional]
**fields** | Option<[**Vec<crate::models::ClassificationTemplateFieldsInner>**](ClassificationTemplate_fields_inner.md)> | A list of fields for this classification template. This includes only one field, the `Box__Security__Classification__Key`, which defines the different classifications available in this enterprise. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


