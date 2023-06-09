# PostTasksRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item** | [**crate::models::PostTasksRequestItem**](post_tasks_request_item.md) |  | 
**action** | Option<**String**> | The action the task assignee will be prompted to do. Must be  * `review` defines an approval task that can be approved or rejected * `complete` defines a general task which can be completed | [optional][default to Review]
**message** | Option<**String**> | An optional message to include with the task. | [optional][default to ]
**due_at** | Option<**String**> | Defines when the task is due. Defaults to `null` if not provided. | [optional]
**completion_rule** | Option<**String**> | Defines which assignees need to complete this task before the task is considered completed.  * `all_assignees` (default) requires all assignees to review or approve the the task in order for it to be considered completed. * `any_assignee` accepts any one assignee to review or approve the the task in order for it to be considered completed. | [optional][default to AllAssignees]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


