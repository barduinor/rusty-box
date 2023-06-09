# Task

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this task | [optional]
**r#type** | Option<**String**> | `task` | [optional]
**item** | Option<[**crate::models::TaskItem**](Task_item.md)> |  | [optional]
**due_at** | Option<**String**> | When the task is due | [optional]
**action** | Option<**String**> | The type of task the task assignee will be prompted to perform. | [optional]
**message** | Option<**String**> | A message that will be included with the task | [optional]
**task_assignment_collection** | Option<[**crate::models::TaskTaskAssignmentCollection**](Task_task_assignment_collection.md)> |  | [optional]
**is_completed** | Option<**bool**> | Whether the task has been completed | [optional]
**created_by** | Option<[**crate::models::TaskCreatedBy**](Task_created_by.md)> |  | [optional]
**created_at** | Option<**String**> | When the task object was created | [optional]
**completion_rule** | Option<**String**> | Defines which assignees need to complete this task before the task is considered completed.  * `all_assignees` requires all assignees to review or approve the the task in order for it to be considered completed. * `any_assignee` accepts any one assignee to review or approve the the task in order for it to be considered completed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


