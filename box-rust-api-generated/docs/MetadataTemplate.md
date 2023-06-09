# MetadataTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the metadata template. | [optional]
**r#type** | **String** | `metadata_template` | 
**scope** | Option<**String**> | The scope of the metadata template can either be `global` or `enterprise_*`. The `global` scope is used for templates that are available to any Box enterprise. The `enterprise_*` scope represents templates that have been created within a specific enterprise, where `*` will be the ID of that enterprise. | [optional]
**template_key** | Option<**String**> | A unique identifier for the template. This identifier is unique across the `scope` of the enterprise to which the metadata template is being applied, yet is not necessarily unique across different enterprises. | [optional]
**display_name** | Option<**String**> | The display name of the template. This can be seen in the Box web app and mobile apps. | [optional]
**hidden** | Option<**bool**> | Defines if this template is visible in the Box web app UI, or if it is purely intended for usage through the API. | [optional]
**fields** | Option<[**Vec<crate::models::MetadataTemplateFieldsInner>**](MetadataTemplate_fields_inner.md)> | An ordered list of template fields which are part of the template. Each field can be a regular text field, date field, number field, as well as a single or multi-select list. | [optional]
**copy_instance_on_item_copy** | Option<**bool**> | Whether or not to include the metadata when a file or folder is copied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


