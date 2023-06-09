# MetadataFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_parent** | Option<**String**> | The identifier of the item that this metadata instance has been attached to. This combines the `type` and the `id` of the parent in the form `{type}_{id}`. | [optional]
**dollar_template** | Option<**String**> | The name of the template | [optional]
**dollar_scope** | Option<**String**> | An ID for the scope in which this template has been applied. This will be `enterprise_{enterprise_id}` for templates defined for use in this enterprise, and `global` for general templates that are available to all enterprises using Box. | [optional]
**dollar_version** | Option<**i32**> | The version of the metadata instance. This version starts at 0 and increases every time a user-defined property is modified. | [optional]
**dollar_can_edit** | Option<**bool**> | Whether the user can edit this metadata instance. | [optional]
**dollar_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A UUID to identify the metadata instance. | [optional]
**dollar_type** | Option<**String**> | A unique identifier for the \"type\" of this instance. This is an internal system property and should not be used by a client application. | [optional]
**dollar_type_version** | Option<**i32**> | The last-known version of the template of the object. This is an internal system property and should not be used by a client application. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


