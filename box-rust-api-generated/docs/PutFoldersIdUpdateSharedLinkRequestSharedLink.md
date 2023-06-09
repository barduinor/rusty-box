# PutFoldersIdUpdateSharedLinkRequestSharedLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access** | Option<**String**> | The level of access for the shared link. This can be restricted to anyone with the link (`open`), only people within the company (`company`) and only those who have been invited to the folder (`collaborators`).  If not set, this field defaults to the access level specified by the enterprise admin. To create a shared link with this default setting pass the `shared_link` object with no `access` field, for example `{ \"shared_link\": {} }`.  The `company` access level is only available to paid accounts. | [optional]
**password** | Option<**String**> | The password required to access the shared link. Set the password to `null` to remove it.  A password can only be set when `access` is set to `open`. | [optional]
**vanity_name** | Option<**String**> | Defines a custom vanity name to use in the shared link URL, for example `https://app.box.com/v/my-shared-link`.  Custom URLs should not be used when sharing sensitive content as vanity URLs are a lot easier to guess than regular shared links. | [optional]
**unshared_at** | Option<**String**> | The timestamp at which this shared link will expire. This field can only be set by users with paid accounts. The value must be greater than the current date and time. | [optional]
**permissions** | Option<[**crate::models::PutFoldersIdAddSharedLinkRequestSharedLinkPermissions**](put_folders_id_add_shared_link_request_shared_link_permissions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


