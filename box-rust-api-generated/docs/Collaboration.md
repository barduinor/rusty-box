# Collaboration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this collaboration. | [optional]
**r#type** | Option<**String**> | `collaboration` | [optional]
**item** | Option<[**crate::models::CollaborationItem**](Collaboration_item.md)> |  | [optional]
**accessible_by** | Option<[**crate::models::CollaborationAccessibleBy**](Collaboration_accessible_by.md)> |  | [optional]
**invite_email** | Option<**String**> | The email address used to invite an unregistered collaborator, if they are not a registered user. | [optional]
**role** | Option<**String**> | The level of access granted. | [optional]
**expires_at** | Option<**String**> | When the collaboration will expire, or `null` if no expiration date is set. | [optional]
**status** | Option<**String**> | The status of the collaboration invitation. If the status is `pending`, `login` and `name` return an empty string. | [optional]
**acknowledged_at** | Option<**String**> | When the `status` of the collaboration object changed to `accepted` or `rejected`. | [optional]
**created_by** | Option<[**crate::models::CollaborationCreatedBy**](Collaboration_created_by.md)> |  | [optional]
**created_at** | Option<**String**> | When the collaboration object was created. | [optional]
**modified_at** | Option<**String**> | When the collaboration object was last modified. | [optional]
**acceptance_requirements_status** | Option<[**crate::models::CollaborationAcceptanceRequirementsStatus**](Collaboration_acceptance_requirements_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


