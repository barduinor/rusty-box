# PostUsersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the user | 
**login** | Option<**String**> | The email address the user uses to log in  Required, unless `is_platform_access_only` is set to `true`. | [optional]
**is_platform_access_only** | Option<**bool**> | Specifies that the user is an app user. | [optional]
**role** | Option<**String**> | The user’s enterprise role | [optional]
**language** | Option<**String**> | The language of the user, formatted in modified version of the [ISO 639-1](/guides/api-calls/language-codes) format. | [optional]
**is_sync_enabled** | Option<**bool**> | Whether the user can use Box Sync | [optional]
**job_title** | Option<**String**> | The user’s job title | [optional]
**phone** | Option<**String**> | The user’s phone number | [optional]
**address** | Option<**String**> | The user’s address | [optional]
**space_amount** | Option<**i64**> | The user’s total available space in bytes. Set this to `-1` to indicate unlimited storage. | [optional]
**tracking_codes** | Option<[**Vec<crate::models::TrackingCode>**](TrackingCode.md)> | Tracking codes allow an admin to generate reports from the admin console and assign an attribute to a specific group of users. This setting must be enabled for an enterprise before it can be used. | [optional]
**can_see_managed_users** | Option<**bool**> | Whether the user can see other enterprise users in their contact list | [optional]
**timezone** | Option<**String**> | The user's timezone | [optional]
**is_external_collab_restricted** | Option<**bool**> | Whether the user is allowed to collaborate with users outside their enterprise | [optional]
**is_exempt_from_device_limits** | Option<**bool**> | Whether to exempt the user from enterprise device limits | [optional]
**is_exempt_from_login_verification** | Option<**bool**> | Whether the user must use two-factor authentication | [optional]
**status** | Option<**String**> | The user's account status | [optional]
**external_app_user_id** | Option<**String**> | An external identifier for an app user, which can be used to look up the user. This can be used to tie user IDs from external identity providers to Box users. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


