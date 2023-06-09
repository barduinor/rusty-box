# SkillInvocationStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The state of this event.  * `invoked` - Triggered the skill with event details to start   applying skill on the file. * `processing` - Currently processing. * `success` - Completed processing with a success. * `transient_failure` - Encountered an issue which can be   retried. * `permanent_failure` -  Encountered a permanent issue and   retry would not help. | [optional]
**message** | Option<**String**> | Status information | [optional]
**error_code** | Option<**String**> | Error code information, if error occurred. | [optional]
**additional_info** | Option<**String**> | Additional status information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


