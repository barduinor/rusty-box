# Lock

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for this lock | [optional]
**r#type** | Option<**String**> | `lock` | [optional]
**created_by** | Option<[**crate::models::LockCreatedBy**](Lock_created_by.md)> |  | [optional]
**created_at** | Option<**String**> | The time this lock was created at. | [optional]
**expired_at** | Option<**String**> | The time this lock is to expire at, which might be in the past. | [optional]
**is_download_prevented** | Option<**bool**> | Whether or not the file can be downloaded while locked. | [optional]
**app_type** | Option<**String**> | If the lock is managed by an application rather than a user, this field identifies the type of the application that holds the lock. This is an open enum and may be extended with additional values in the future. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


