# MetadataBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_parent** | Option<**String**> | The identifier of the item that this metadata instance has been attached to. This combines the `type` and the `id` of the parent in the form `{type}_{id}`. | [optional]
**dollar_template** | Option<**String**> | The name of the template | [optional]
**dollar_scope** | Option<**String**> | An ID for the scope in which this template has been applied. This will be `enterprise_{enterprise_id}` for templates defined for use in this enterprise, and `global` for general templates that are available to all enterprises using Box. | [optional]
**dollar_version** | Option<**i32**> | The version of the metadata instance. This version starts at 0 and increases every time a user-defined property is modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


