# PostMetadataTemplatesSchemaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | **String** | The scope of the metadata template to create. Applications can only create templates for use within the authenticated user's enterprise.  This value needs to be set to `enterprise`, as `global` scopes can not be created by applications. | 
**template_key** | Option<**String**> | A unique identifier for the template. This identifier needs to be unique across the enterprise for which the metadata template is being created.  When not provided, the API will create a unique `templateKey` based on the value of the `displayName`. | [optional]
**display_name** | **String** | The display name of the template. | 
**hidden** | Option<**bool**> | Defines if this template is visible in the Box web app UI, or if it is purely intended for usage through the API. | [optional][default to false]
**fields** | Option<[**Vec<crate::models::MetadataFieldWrite>**](Metadata_Field__Write_.md)> | An ordered list of template fields which are part of the template. Each field can be a regular text field, date field, number field, as well as a single or multi-select list. | [optional]
**copy_instance_on_item_copy** | Option<**bool**> | Whether or not to copy any metadata attached to a file or folder when it is copied. By default, metadata is not copied along with a file or folder when it is copied. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


