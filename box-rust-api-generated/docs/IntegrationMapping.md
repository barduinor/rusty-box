# IntegrationMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique identifier of a folder mapping (part of a composite key together with `integration_type`) | [optional]
**integration_type** | Option<**String**> | Identifies the Box partner app, with which the mapping is associated. Currently only supports Slack. (part of the composite key together with `id`) | [optional]
**r#type** | **String** | Mapping type | 
**partner_item** | [**crate::models::IntegrationMappingAllOfPartnerItem**](IntegrationMapping_allOf_partner_item.md) |  | 
**box_item** | [**crate::models::IntegrationMappingAllOfBoxItem**](IntegrationMapping_allOf_box_item.md) |  | 
**is_manually_created** | Option<**bool**> | Identifies whether the mapping has been manually set (as opposed to being automatically created) | [optional]
**options** | Option<[**crate::models::IntegrationMappingAllOfOptions**](IntegrationMapping_allOf_options.md)> |  | [optional]
**created_by** | Option<[**crate::models::IntegrationMappingAllOfCreatedBy**](IntegrationMapping_allOf_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::IntegrationMappingAllOfModifiedBy**](IntegrationMapping_allOf_modified_by.md)> |  | [optional]
**created_at** | Option<**String**> | When the integration mapping object was created | [optional]
**modified_at** | Option<**String**> | When the integration mapping object was last modified | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


