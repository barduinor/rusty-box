# Classification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**box__security__classification__key** | Option<**String**> | The name of the classification applied to the item. | [optional]
**dollar_parent** | Option<**String**> | The identifier of the item that this metadata instance has been attached to. This combines the `type` and the `id` of the parent in the form `{type}_{id}`. | [optional]
**dollar_template** | Option<**String**> | `securityClassification-6VMVochwUWo` | [optional]
**dollar_scope** | Option<**String**> | The scope of the enterprise that this classification has been applied for.  This will be in the format `enterprise_{enterprise_id}`. | [optional]
**dollar_version** | Option<**i32**> | The version of the metadata instance. This version starts at 0 and increases every time a classification is updated. | [optional]
**dollar_type** | Option<**String**> | The unique ID of this classification instance. This will be include the name of the classification template and a unique ID. | [optional]
**dollar_type_version** | Option<**f32**> | The version of the metadata template. This version starts at 0 and increases every time the template is updated. This is mostly for internal use. | [optional]
**dollar_can_edit** | Option<**bool**> | Whether an end user can change the classification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


