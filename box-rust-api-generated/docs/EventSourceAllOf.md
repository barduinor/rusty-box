# EventSourceAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier that represent a folder.  The ID for any folder can be determined by visiting a folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folders/123` the `folder_id` is `123`. | 
**r#type** | **String** | `folder` | 
**name** | Option<**String**> | The name of the folder. | [optional]
**login** | Option<**String**> | The primary email address of this user | [optional]
**created_at** | Option<**String**> | The date and time when the folder was created. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**modified_at** | Option<**String**> | The date and time when the folder was last updated. This value may be `null` for some folders such as the root folder or the trash folder. | [optional]
**language** | Option<**String**> | The language of the user, formatted in modified version of the [ISO 639-1](/guides/api-calls/language-codes) format. | [optional]
**timezone** | Option<**String**> | The user's timezone | [optional]
**space_amount** | Option<**i64**> | The user’s total available space amount in bytes | [optional]
**space_used** | Option<**i64**> | The amount of space in use by the user | [optional]
**max_upload_size** | Option<**i64**> | The maximum individual file size in bytes the user can have | [optional]
**status** | Option<**String**> | The user's account status | [optional]
**job_title** | Option<**String**> | The user’s job title | [optional]
**phone** | Option<**String**> | The user’s phone number | [optional]
**address** | Option<**String**> | The user’s address | [optional]
**avatar_url** | Option<**String**> | URL of the user’s avatar image | [optional]
**notification_email** | Option<[**crate::models::UserAllOfNotificationEmail**](User_allOf_notification_email.md)> |  | [optional]
**item_type** | **String** | The type of the item that the event represents. Can be `file` or `folder`.  | 
**item_id** | **String** | The unique identifier that represents the item.  | 
**item_name** | **String** | The name of the item.  | 
**classification** | Option<[**crate::models::EventSourceClassification**](EventSource_classification.md)> |  | [optional]
**parent** | Option<[**crate::models::FolderAllOfParent**](Folder_allOf_parent.md)> |  | [optional]
**owned_by** | Option<[**crate::models::FolderAllOfOwnedBy**](Folder_allOf_owned_by.md)> |  | [optional]
**etag** | Option<**String**> | The HTTP `etag` of this folder. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the folder if (no) changes have happened. | [optional]
**sequence_id** | Option<**String**> | A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource. | [optional]
**sha1** | Option<**String**> | The SHA1 hash of the file. This can be used to compare the contents of a file on Box with a local file. | [optional]
**file_version** | Option<[**crate::models::FileMiniAllOfFileVersion**](File__Mini_allOf_file_version.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**size** | Option<**i64**> | The folder size in bytes.  Be careful parsing this integer as its value can get very large. | [optional]
**path_collection** | Option<[**crate::models::FolderAllOfPathCollection**](Folder_allOf_path_collection.md)> |  | [optional]
**trashed_at** | Option<**String**> | The time at which this folder was put in the trash. | [optional]
**purged_at** | Option<**String**> | The time at which this folder is expected to be purged from the trash. | [optional]
**content_created_at** | Option<**String**> | The date and time at which this folder was originally created. | [optional]
**content_modified_at** | Option<**String**> | The date and time at which this folder was last updated. | [optional]
**created_by** | Option<[**crate::models::FolderAllOfCreatedBy**](Folder_allOf_created_by.md)> |  | [optional]
**modified_by** | Option<[**crate::models::FolderAllOfModifiedBy**](Folder_allOf_modified_by.md)> |  | [optional]
**shared_link** | Option<[**crate::models::FolderAllOfSharedLink**](Folder_allOf_shared_link.md)> |  | [optional]
**item_status** | Option<**String**> | Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted. | [optional]
**folder_upload_email** | Option<[**crate::models::FolderAllOfFolderUploadEmail**](Folder_allOf_folder_upload_email.md)> |  | [optional]
**item_collection** | Option<[**crate::models::FolderAllOfItemCollection**](Folder_allOf_item_collection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


