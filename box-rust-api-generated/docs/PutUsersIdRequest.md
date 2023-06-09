# PutUsersIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enterprise** | Option<**String**> | Set this to `null` to roll the user out of the enterprise and make them a free user | [optional]
**notify** | Option<**bool**> | Whether the user should receive an email when they are rolled out of an enterprise | [optional]
**name** | Option<**String**> | The name of the user | [optional]
**login** | Option<**String**> | The email address the user uses to log in  Note: If the target user's email is not confirmed, then the primary login address cannot be changed. | [optional]
**role** | Option<**String**> | The user’s enterprise role | [optional]
**language** | Option<**String**> | The language of the user, formatted in modified version of the [ISO 639-1](/guides/api-calls/language-codes) format. | [optional]
**is_sync_enabled** | Option<**bool**> | Whether the user can use Box Sync | [optional]
**job_title** | Option<**String**> | The user’s job title | [optional]
**phone** | Option<**String**> | The user’s phone number | [optional]
**address** | Option<**String**> | The user’s address | [optional]
**tracking_codes** | Option<[**Vec<crate::models::TrackingCode>**](TrackingCode.md)> | Tracking codes allow an admin to generate reports from the admin console and assign an attribute to a specific group of users. This setting must be enabled for an enterprise before it can be used. | [optional]
**can_see_managed_users** | Option<**bool**> | Whether the user can see other enterprise users in their contact list | [optional]
**timezone** | Option<**String**> | The user's timezone | [optional]
**is_external_collab_restricted** | Option<**bool**> | Whether the user is allowed to collaborate with users outside their enterprise | [optional]
**is_exempt_from_device_limits** | Option<**bool**> | Whether to exempt the user from enterprise device limits | [optional]
**is_exempt_from_login_verification** | Option<**bool**> | Whether the user must use two-factor authentication | [optional]
**is_password_reset_required** | Option<**bool**> | Whether the user is required to reset their password | [optional]
**status** | Option<**String**> | The user's account status | [optional]
**space_amount** | Option<**i64**> | The user’s total available space in bytes. Set this to `-1` to indicate unlimited storage. | [optional]
**notification_email** | Option<[**crate::models::PutUsersIdRequestNotificationEmail**](put_users_id_request_notification_email.md)> |  | [optional]
**external_app_user_id** | Option<**String**> | An external identifier for an app user, which can be used to look up the user. This can be used to tie user IDs from external identity providers to Box users.  Note: In order to update this field, you need to request a token using the application that created the app user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


