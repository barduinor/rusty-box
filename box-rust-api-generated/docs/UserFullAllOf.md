# UserFullAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role** | Option<**String**> | The user’s enterprise role | [optional]
**tracking_codes** | Option<[**Vec<crate::models::TrackingCode>**](TrackingCode.md)> | Tracking codes allow an admin to generate reports from the admin console and assign an attribute to a specific group of users. This setting must be enabled for an enterprise before it can be used. | [optional]
**can_see_managed_users** | Option<**bool**> | Whether the user can see other enterprise users in their contact list | [optional]
**is_sync_enabled** | Option<**bool**> | Whether the user can use Box Sync | [optional]
**is_external_collab_restricted** | Option<**bool**> | Whether the user is allowed to collaborate with users outside their enterprise | [optional]
**is_exempt_from_device_limits** | Option<**bool**> | Whether to exempt the user from Enterprise device limits | [optional]
**is_exempt_from_login_verification** | Option<**bool**> | Whether the user must use two-factor authentication | [optional]
**enterprise** | Option<[**crate::models::UserFullAllOfEnterprise**](User__Full_allOf_enterprise.md)> |  | [optional]
**my_tags** | Option<**Vec<String>**> | Tags for all files and folders owned by the user. Values returned will only contain tags that were set by the requester. | [optional]
**hostname** | Option<**String**> | The root (protocol, subdomain, domain) of any links that need to be generated for the user | [optional]
**is_platform_access_only** | Option<**bool**> | Whether the user is an App User | [optional]
**external_app_user_id** | Option<**String**> | An external identifier for an app user, which can be used to look up the user. This can be used to tie user IDs from external identity providers to Box users. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


