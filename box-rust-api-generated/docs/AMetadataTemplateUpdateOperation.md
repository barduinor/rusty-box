# AMetadataTemplateUpdateOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**op** | **String** | The type of change to perform on the template. Some of these are hazardous as they will change existing templates. | 
**data** | Option<**::std::collections::HashMap<String, String>**> | The data for the operation. This will vary depending on the operation being performed. | [optional]
**field_key** | Option<**String**> | For operations that affect a single field this defines the key of the field that is affected. | [optional]
**field_keys** | Option<**Vec<String>**> | For operations that affect multiple fields this defines the keys of the fields that are affected. | [optional]
**enum_option_key** | Option<**String**> | For operations that affect a single `enum` option this defines the key of the option that is affected. | [optional]
**enum_option_keys** | Option<**Vec<String>**> | For operations that affect multiple `enum` options this defines the keys of the options that are affected. | [optional]
**multi_select_option_key** | Option<**String**> | For operations that affect a single multi select option this defines the key of the option that is affected. | [optional]
**multi_select_option_keys** | Option<**Vec<String>**> | For operations that affect multiple multi select options this defines the keys of the options that are affected. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


