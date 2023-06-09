# PostGroupsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the new group to be created. This name must be unique within the enterprise. | 
**provenance** | Option<**String**> | Keeps track of which external source this group is coming, for example `Active Directory`, or `Okta`.  Setting this will also prevent Box admins from editing the group name and its members directly via the Box web application.  This is desirable for one-way syncing of groups. | [optional]
**external_sync_identifier** | Option<**String**> | An arbitrary identifier that can be used by external group sync tools to link this Box Group to an external group.  Example values of this field could be an **Active Directory Object ID** or a **Google Group ID**.  We recommend you use of this field in order to avoid issues when group names are updated in either Box or external systems. | [optional]
**description** | Option<**String**> | A human readable description of the group. | [optional]
**invitability_level** | Option<**String**> | Specifies who can invite the group to collaborate on folders.  When set to `admins_only` the enterprise admin, co-admins, and the group's admin can invite the group.  When set to `admins_and_members` all the admins listed above and group members can invite the group.  When set to `all_managed_users` all managed users in the enterprise can invite the group. | [optional]
**member_viewability_level** | Option<**String**> | Specifies who can see the members of the group.  * `admins_only` - the enterprise admin, co-admins, group's   group admin * `admins_and_members` - all admins and group members * `all_managed_users` - all managed users in the   enterprise | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


