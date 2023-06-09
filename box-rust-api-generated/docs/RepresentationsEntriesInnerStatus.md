# RepresentationsEntriesInnerStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The status of the representation.  * `success` defines the representation as ready to be viewed. * `viewable` defines a video to be ready for viewing. * `pending` defines the representation as to be generated. Retry   this endpoint to re-check the status. * `none` defines that the representation will be created when   requested. Request the URL defined in the `info` object to   trigger this generation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


