# TaskAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this task assignment | [optional]
**r#type** | Option<**String**> | `task_assignment` | [optional]
**item** | Option<[**crate::models::TaskAssignmentItem**](TaskAssignment_item.md)> |  | [optional]
**assigned_to** | Option<[**crate::models::TaskAssignmentAssignedTo**](TaskAssignment_assigned_to.md)> |  | [optional]
**message** | Option<**String**> | A message that will is included with the task assignment. This is visible to the assigned user in the web and mobile UI. | [optional]
**completed_at** | Option<**String**> | The date at which this task assignment was completed. This will be `null` if the task is not completed yet. | [optional]
**assigned_at** | Option<**String**> | The date at which this task was assigned to the user. | [optional]
**reminded_at** | Option<**String**> | The date at which the assigned user was reminded of this task assignment. | [optional]
**resolution_state** | Option<**String**> | The current state of the assignment. The available states depend on the `action` value of the task object. | [optional]
**assigned_by** | Option<[**crate::models::TaskAssignmentAssignedBy**](TaskAssignment_assigned_by.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


