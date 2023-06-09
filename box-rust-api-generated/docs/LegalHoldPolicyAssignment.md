# LegalHoldPolicyAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this legal hold assignment | [optional]
**r#type** | Option<**String**> | `legal_hold_policy_assignment` | [optional]
**legal_hold_policy** | Option<[**crate::models::LegalHoldPolicyAssignmentAllOfLegalHoldPolicy**](LegalHoldPolicyAssignment_allOf_legal_hold_policy.md)> |  | [optional]
**assigned_to** | Option<[**crate::models::LegalHoldPolicyAssignmentAllOfAssignedTo**](LegalHoldPolicyAssignment_allOf_assigned_to.md)> |  | [optional]
**assigned_by** | Option<[**crate::models::LegalHoldPolicyAssignmentAllOfAssignedBy**](LegalHoldPolicyAssignment_allOf_assigned_by.md)> |  | [optional]
**assigned_at** | Option<**String**> | When the legal hold policy assignment object was created | [optional]
**deleted_at** | Option<**String**> | When the assignment release request was sent. (Because it can take time for an assignment to fully delete, this isn't quite the same time that the assignment is fully deleted). If null, Assignment was not deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


