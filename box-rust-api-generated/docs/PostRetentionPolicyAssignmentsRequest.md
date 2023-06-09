# PostRetentionPolicyAssignmentsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the retention policy to assign | 
**assign_to** | [**crate::models::PostRetentionPolicyAssignmentsRequestAssignTo**](post_retention_policy_assignments_request_assign_to.md) |  | 
**filter_fields** | Option<[**Vec<crate::models::PostRetentionPolicyAssignmentsRequestFilterFieldsInner>**](post_retention_policy_assignments_request_filter_fields_inner.md)> | If the `assign_to` type is `metadata_template`, then optionally add the `filter_fields` parameter which will require an array of objects with a field entry and a value entry. Currently only one object of `field` and `value` is supported. | [optional]
**start_date_field** | Option<**String**> | The date the retention policy assignment begins.  If the `assigned_to` type is `metadata_template`, this field can be a date field's metadata attribute key id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


