# TemplateSignerInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_tag_id** | Option<**String**> | This references the ID of a specific tag contained in a file of the sign request. | [optional]
**text_value** | Option<**String**> | Text prefill value | [optional]
**checkbox_value** | Option<**bool**> | Checkbox prefill value | [optional]
**date_value** | Option<**String**> | Date prefill value | [optional]
**r#type** | Option<**String**> | Type of input | [optional]
**content_type** | Option<**String**> | Content type of input | [optional]
**is_required** | Option<**bool**> | Whether or not the input is required. | [optional]
**page_index** | **i32** | Index of page that the input is on. | 
**document_id** | Option<**String**> | Document identifier. | [optional]
**dropdown_choices** | Option<**Vec<String>**> | When the input is of the type `dropdown` this values will be filled with all the dropdown options. | [optional]
**group_id** | Option<**String**> | When the input is of type `radio` they can be grouped to gather with this identifier. | [optional]
**coordinates** | Option<[**crate::models::TemplateSignerInputAllOfCoordinates**](TemplateSignerInput_allOf_coordinates.md)> |  | [optional]
**dimensions** | Option<[**crate::models::TemplateSignerInputAllOfDimensions**](TemplateSignerInput_allOf_dimensions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


