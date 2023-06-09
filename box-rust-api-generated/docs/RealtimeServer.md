# RealtimeServer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | `realtime_server` | [optional]
**url** | Option<**String**> | The URL for the server. | [optional]
**ttl** | Option<**i32**> | The time in minutes for which this server is available | [optional]
**max_retries** | Option<**i32**> | The maximum number of retries this server will allow before a new long poll should be started by getting a [new list of server](#options-events). | [optional]
**retry_timeout** | Option<**i32**> | The maximum number of seconds without a response after which you should retry the long poll connection.  This helps to overcome network issues where the long poll looks to be working but no packages are coming through. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


