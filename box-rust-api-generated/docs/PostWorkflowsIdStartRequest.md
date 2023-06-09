# PostWorkflowsIdStartRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of the parameters object | [optional]
**flow** | [**crate::models::PostWorkflowsIdStartRequestFlow**](post_workflows_id_start_request_flow.md) |  | 
**files** | [**Vec<crate::models::PostWorkflowsIdStartRequestFilesInner>**](post_workflows_id_start_request_files_inner.md) | The array of files for which the workflow should start. All files must be in the workflow's configured folder. | 
**folder** | [**crate::models::PostWorkflowsIdStartRequestFolder**](post_workflows_id_start_request_folder.md) |  | 
**outcomes** | Option<[**Vec<crate::models::PostWorkflowsIdStartRequestOutcomesInner>**](post_workflows_id_start_request_outcomes_inner.md)> | A list of outcomes required to be configured at start time. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


