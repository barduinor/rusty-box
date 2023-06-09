# RetentionPolicyAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for a retention policy assignment. | [optional]
**r#type** | Option<**String**> | `retention_policy_assignment` | [optional]
**retention_policy** | Option<[**crate::models::RetentionPolicyAssignmentRetentionPolicy**](RetentionPolicyAssignment_retention_policy.md)> |  | [optional]
**assigned_to** | Option<[**crate::models::RetentionPolicyAssignmentAssignedTo**](RetentionPolicyAssignment_assigned_to.md)> |  | [optional]
**filter_fields** | Option<[**Vec<crate::models::RetentionPolicyAssignmentFilterFieldsInner>**](RetentionPolicyAssignment_filter_fields_inner.md)> | An array of field objects. Values are only returned if the `assigned_to` type is `metadata_template`. Otherwise, the array is blank. | [optional]
**assigned_by** | Option<[**crate::models::RetentionPolicyAssignmentAssignedBy**](RetentionPolicyAssignment_assigned_by.md)> |  | [optional]
**assigned_at** | Option<**String**> | When the retention policy assignment object was created. | [optional]
**start_date_field** | Option<**String**> | The date the retention policy assignment begins. If the `assigned_to` type is `metadata_template`, this field can be a date field's metadata attribute key id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


