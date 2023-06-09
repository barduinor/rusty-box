# MetadataFieldWrite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The type of field. The basic fields are a `string` field for text, a `float` field for numbers, and a `date` fields to present the user with a date-time picker.  Additionally, metadata templates support an `enum` field for a basic list of items, and ` multiSelect` field for a similar list of items where the user can select more than one value. | 
**key** | **String** | A unique identifier for the field. The identifier must be unique within the template to which it belongs. | 
**display_name** | **String** | The display name of the field as it is shown to the user in the web and mobile apps. | 
**description** | Option<**String**> | A description of the field. This is not shown to the user. | [optional]
**hidden** | Option<**bool**> | Whether this field is hidden in the UI for the user and can only be set through the API instead. | [optional]
**options** | Option<[**Vec<crate::models::MetadataOptionWrite>**](Metadata_Option__Write_.md)> | A list of options for this field. This is used in combination with the `enum` and `multiSelect` field types. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


