# \EventsApi

All URIs are relative to *https://api.box.com/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_events**](EventsApi.md#get_events) | **GET** /events | List user and enterprise events
[**options_events**](EventsApi.md#options_events) | **OPTIONS** /events | Get events long poll endpoint



## get_events

> crate::models::Events get_events(stream_type, stream_position, limit, event_type, created_after, created_before)
List user and enterprise events

Returns up to a year of past events for a given user or for the entire enterprise.  By default this returns events for the authenticated user. To retrieve events for the entire enterprise, set the `stream_type` to `admin_logs_streaming` for live monitoring of new events, or `admin_logs` for querying across historical events. The user making the API call will need to have admin privileges, and the application will need to have the scope `manage enterprise properties` checked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_type** | Option<**String**> | Defines the type of events that are returned  * `all` returns everything for a user and is the default * `changes` returns events that may cause file tree changes   such as file updates or collaborations. * `sync` is similar to `changes` but only applies to synced folders * `admin_logs` returns all events for an entire enterprise and   requires the user making the API call to have admin permissions. This   stream type is for programmatically pulling from a 1 year history of   events across all users within the enterprise and within a   `created_after` and `created_before` time frame. The complete history   of events will be returned in chronological order based on the event   time, but latency will be much higher than `admin_logs_streaming`. * `admin_logs_streaming` returns all events for an entire enterprise and   requires the user making the API call to have admin permissions. This   stream type is for polling for recent events across all users within   the enterprise. Latency will be much lower than `admin_logs`, but   events will not be returned in chronological order and may   contain duplicates. |  |[default to all]
**stream_position** | Option<**String**> | The location in the event stream to start receiving events from.  * `now` will return an empty list events and the latest stream position for initialization. * `0` or `null` will return all events. |  |
**limit** | Option<**i64**> | Limits the number of events returned  Note: Sometimes, the events less than the limit requested can be returned even when there may be more events remaining. This is primarily done in the case where a number of events have already been retrieved and these retrieved events are returned rather than delaying for an unknown amount of time to see if there are any more results. |  |[default to 100]
**event_type** | Option<[**Vec<String>**](String.md)> | A comma-separated list of events to filter by. This can only be used when requesting the events with a `stream_type` of `admin_logs` or `adming_logs_streaming`. For any other `stream_type` this value will be ignored. |  |
**created_after** | Option<**String**> | The lower bound date and time to return events for. This can only be used when requesting the events with a `stream_type` of `admin_logs`. For any other `stream_type` this value will be ignored. |  |
**created_before** | Option<**String**> | The upper bound date and time to return events for. This can only be used when requesting the events with a `stream_type` of `admin_logs`. For any other `stream_type` this value will be ignored. |  |

### Return type

[**crate::models::Events**](Events.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_events

> crate::models::RealtimeServers options_events()
Get events long poll endpoint

Returns a list of real-time servers that can be used for long-polling updates to the [event stream](#get-events).  Long polling is the concept where a HTTP request is kept open until the server sends a response, then repeating the process over and over to receive updated responses.  Long polling the event stream can only be used for user events, not for enterprise events.  To use long polling, first use this endpoint to retrieve a list of long poll URLs. Next, make a long poll request to any of the provided URLs.  When an event occurs in monitored account a response with the value `new_change` will be sent. The response contains no other details as it only serves as a prompt to take further action such as sending a request to the [events endpoint](#get-events) with the last known `stream_position`.  After the server sends this response it closes the connection. You must now repeat the long poll process to begin listening for events again.  If no events occur for a while and the connection times out you will receive a response with the value `reconnect`. When you receive this response you’ll make another call to this endpoint to restart the process.  If you receive no events in `retry_timeout` seconds then you will need to make another request to the real-time server (one of the URLs in the response for this endpoint). This might be necessary due to network errors.  Finally, if you receive a `max_retries` error when making a request to the real-time server, you should start over by making a call to this endpoint first.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RealtimeServers**](RealtimeServers.md)

### Authorization

[OAuth2Security](../README.md#OAuth2Security)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

