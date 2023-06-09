# GroupFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this object | [optional]
**r#type** | Option<**String**> | `group` | [optional]
**name** | Option<**String**> | The name of the group | [optional]
**group_type** | Option<**String**> | The type of the group. | [optional]
**created_at** | Option<**String**> | When the group object was created | [optional]
**modified_at** | Option<**String**> | When the group object was last modified | [optional]
**provenance** | Option<**String**> | Keeps track of which external source this group is coming from (e.g. \"Active Directory\", \"Google Groups\", \"Facebook Groups\").  Setting this will also prevent Box users from editing the group name and its members directly via the Box web application. This is desirable for one-way syncing of groups. | [optional]
**external_sync_identifier** | Option<**String**> | An arbitrary identifier that can be used by external group sync tools to link this Box Group to an external group. Example values of this field could be an Active Directory Object ID or a Google Group ID.  We recommend you use of this field in order to avoid issues when group names are updated in either Box or external systems. | [optional]
**description** | Option<**String**> | Human readable description of the group. | [optional]
**invitability_level** | Option<**String**> | Specifies who can invite the group to collaborate on items.  When set to `admins_only` the enterprise admin, co-admins, and the group's admin can invite the group.  When set to `admins_and_members` all the admins listed above and group members can invite the group.  When set to `all_managed_users` all managed users in the enterprise can invite the group. | [optional]
**member_viewability_level** | Option<**String**> | Specifies who can view the members of the group (Get Memberships for Group).  * `admins_only` - the enterprise admin, co-admins, group's   group admin * `admins_and_members` - all admins and group members * `all_managed_users` - all managed users in the   enterprise | [optional]
**permissions** | Option<[**crate::models::GroupFullAllOfPermissions**](Group__Full_allOf_permissions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


