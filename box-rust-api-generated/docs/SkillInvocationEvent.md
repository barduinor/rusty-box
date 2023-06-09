# SkillInvocationEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | `event` | [optional]
**created_at** | Option<**String**> | When the event object was created | [optional]
**recorded_at** | Option<**String**> | When the event object was recorded in database | [optional]
**event_id** | Option<**String**> | The ID of the event object. You can use this to detect duplicate events | [optional]
**created_by** | Option<[**crate::models::EventCreatedBy**](Event_created_by.md)> |  | [optional]
**event_type** | Option<**String**> |  | [optional]
**session_id** | Option<**String**> | The session of the user that performed the action. Not all events will populate this attribute. | [optional]
**source** | Option<[**crate::models::EventSource**](Event_source.md)> |  | [optional]
**additional_details** | Option<[**serde_json::Value**](.md)> | This object provides additional information about the event if available.  This can include how a user performed an event as well as additional information to correlate an event to external KeySafe logs. Not all events have an `additional_details` object.  This object is only available in the Enterprise Events. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


