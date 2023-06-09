# IntegrationMappingMini

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique identifier of a folder mapping (part of a composite key together with `integration_type`) | [optional]
**integration_type** | Option<**String**> | Identifies the Box partner app, with which the mapping is associated. Currently only supports Slack. (part of the composite key together with `id`) | [optional]
**partner_item_id** | Option<**String**> | ID of the mapped partner item | [optional]
**partner_item_type** | Option<**String**> | Domain-specific type of the mapped partner item | [optional]
**box_item_id** | Option<**String**> | ID of the Box item mapped to the object referenced in `partner_item_id` | [optional]
**box_item_type** | Option<**String**> | Type of the Box object referenced in `box_item_id` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


