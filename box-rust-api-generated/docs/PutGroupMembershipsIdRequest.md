# PutGroupMembershipsIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role** | Option<**String**> | The role of the user in the group. | [optional]
**configurable_permissions** | Option<**::std::collections::HashMap<String, bool>**> | Custom configuration for the permissions an admin if a group will receive. This option has no effect on members with a role of `member`.  Setting these permissions overwrites the default access levels of an admin.  Specifying a value of \"null\" for this object will disable all configurable permissions. Specifying permissions will set them accordingly, omitted permissions will be enabled by default. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


