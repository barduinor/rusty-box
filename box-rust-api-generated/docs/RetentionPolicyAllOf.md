# RetentionPolicyAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The additional text description of the retention policy. | [optional]
**policy_type** | Option<**String**> | The type of the retention policy. A retention policy type can either be `finite`, where a specific amount of time to retain the content is known upfront, or `indefinite`, where the amount of time to retain the content is still unknown. | [optional]
**retention_type** | Option<**String**> | Specifies the retention type:  * `modifiable`: You can modify the retention policy. For example,  you can add or remove folders, shorten or lengthen  the policy duration, or delete the assignment.  Use this type if your retention policy  is not related to any regulatory purposes.  * `non-modifiable`: You can modify the retention policy  only in a limited way: add a folder, lengthen the duration,  retire the policy, change the disposition action  or notification settings. You cannot perform other actions,  such as deleting the assignment or shortening the  policy duration. Use this type to ensure  compliance with regulatory retention policies. | [optional]
**status** | Option<**String**> | The status of the retention policy. The status of a policy will be `active`, unless explicitly retired by an administrator, in which case the status will be `retired`. Once a policy has been retired, it cannot become active again. | [optional]
**created_by** | Option<[**crate::models::RetentionPolicyAllOfCreatedBy**](RetentionPolicy_allOf_created_by.md)> |  | [optional]
**created_at** | Option<**String**> | When the retention policy object was created. | [optional]
**modified_at** | Option<**String**> | When the retention policy object was last modified. | [optional]
**can_owner_extend_retention** | Option<**bool**> | Determines if the owner of items under the policy can extend the retention when the original retention duration is about to end. | [optional]
**are_owners_notified** | Option<**bool**> | Determines if owners and co-owners of items under the policy are notified when the retention duration is about to end. | [optional]
**custom_notification_recipients** | Option<[**Vec<crate::models::UserMini>**](User--Mini.md)> | A list of users notified when the retention policy duration is about to end. | [optional]
**assignment_counts** | Option<[**crate::models::RetentionPolicyAllOfAssignmentCounts**](RetentionPolicy_allOf_assignment_counts.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


