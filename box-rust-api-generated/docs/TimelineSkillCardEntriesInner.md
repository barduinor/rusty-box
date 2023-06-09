# TimelineSkillCardEntriesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | Option<**String**> | The text of the entry. This would be the display name for an item being placed on the timeline, for example the name of the person who was detected in a video. | [optional]
**appears** | Option<[**Vec<crate::models::TimelineSkillCardEntriesInnerAppearsInner>**](TimelineSkillCard_entries_inner_appears_inner.md)> | Defines a list of timestamps for when this item should appear on the timeline. | [optional]
**image_url** | Option<**String**> | The image to show on a for an entry that appears on a timeline. This image URL is required for every entry.  The image will be shown in a list of items (for example faces), and clicking the image will show the user where that entry appears during the duration of this entry. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


