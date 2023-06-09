# AMetadataInstanceUpdateOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**op** | Option<**String**> | The type of change to perform on the template. Some of these are hazardous as they will change existing templates. | [optional]
**path** | Option<**String**> | The location in the metadata JSON object to apply the changes to, in the format of a [JSON-Pointer](https://tools.ietf.org/html/rfc6901).  The path must always be prefixed with a `/` to represent the root of the template. The characters `~` and `/` are reserved characters and must be escaped in the key. | [optional]
**value** | Option<**String**> | The value to be set or tested.  Required for `add`, `replace`, and `test` operations. For `add`, if the value exists already the previous value will be overwritten by the new value. For `replace`, the value must exist before replacing.  For `test`, the existing value at the `path` location must match the specified value. | [optional]
**from** | Option<**String**> | The location in the metadata JSON object to move or copy a value from. Required for `move` or `copy` operations and must be in the format of a [JSON-Pointer](https://tools.ietf.org/html/rfc6901). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


