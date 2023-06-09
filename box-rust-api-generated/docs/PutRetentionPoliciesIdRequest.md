# PutRetentionPoliciesIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy_name** | Option<**String**> | The name for the retention policy | [optional]
**description** | Option<**String**> | The additional text description of the retention policy. | [optional]
**disposition_action** | Option<**String**> | The disposition action of the retention policy. `permanently_delete` deletes the content retained by the policy permanently. `remove_retention` lifts retention policy from the content, allowing it to be deleted by users once the retention policy has expired. | [optional]
**retention_type** | Option<**String**> | Specifies the retention type:  * `modifiable`: You can modify the retention policy. For example, you can add or remove folders, shorten or lengthen the policy duration, or delete the assignment. Use this type if your retention policy is not related to any regulatory purposes. * `non-modifiable`: You can modify the retention policy only in a limited way: add a folder, lengthen the duration, retire the policy, change the disposition action or notification settings. You cannot perform other actions, such as deleting the assignment or shortening the policy duration. Use this type to ensure compliance with regulatory retention policies.  When updating a retention policy, you can use `non-modifiable` type only. You can convert a `modifiable` policy to `non-modifiable`, but not the other way around. | [optional]
**retention_length** | Option<**String**> | The length of the retention policy. This value specifies the duration in days that the retention policy will be active for after being assigned to content.  If the policy has a `policy_type` of `indefinite`, the `retention_length` will also be `indefinite`. | [optional]
**status** | Option<**String**> | Used to retire a retention policy.  If not retiring a policy, do not include this parameter or set it to `null`. | [optional]
**can_owner_extend_retention** | Option<**bool**> | Determines if the owner of items under the policy can extend the retention when the original retention duration is about to end. | [optional]
**are_owners_notified** | Option<**bool**> | Determines if owners and co-owners of items under the policy are notified when the retention duration is about to end. | [optional]
**custom_notification_recipients** | Option<[**Vec<crate::models::UserMini>**](User--Mini.md)> | A list of users notified when the retention duration is about to end. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


