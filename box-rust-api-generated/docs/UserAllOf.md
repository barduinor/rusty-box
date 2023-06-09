# UserAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | When the user object was created | [optional]
**modified_at** | Option<**String**> | When the user object was last modified | [optional]
**language** | Option<**String**> | The language of the user, formatted in modified version of the [ISO 639-1](/guides/api-calls/language-codes) format. | [optional]
**timezone** | Option<**String**> | The user's timezone | [optional]
**space_amount** | Option<**i64**> | The user’s total available space amount in bytes | [optional]
**space_used** | Option<**i64**> | The amount of space in use by the user | [optional]
**max_upload_size** | Option<**i64**> | The maximum individual file size in bytes the user can have | [optional]
**status** | Option<**String**> | The user's account status | [optional]
**job_title** | Option<**String**> | The user’s job title | [optional]
**phone** | Option<**String**> | The user’s phone number | [optional]
**address** | Option<**String**> | The user’s address | [optional]
**avatar_url** | Option<**String**> | URL of the user’s avatar image | [optional]
**notification_email** | Option<[**crate::models::UserAllOfNotificationEmail**](User_allOf_notification_email.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


