# PostCommentsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **String** | The text of the comment.  To mention a user, use the `tagged_message` parameter instead. | 
**tagged_message** | Option<**String**> | The text of the comment, including `@[user_id:name]` somewhere in the message to mention another user, which will send them an email notification, letting them know they have been mentioned.  The `user_id` is the target user's ID, where the `name` can be any custom phrase. In the Box UI this name will link to the user's profile.  If you are not mentioning another user, use `message` instead. | [optional]
**item** | Option<[**crate::models::PostCommentsRequestItem**](post_comments_request_item.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


