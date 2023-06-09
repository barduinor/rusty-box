# TranscriptSkillCard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The optional date and time this card was created at. | [optional]
**r#type** | **String** | `skill_card` | 
**skill_card_type** | **String** | `transcript` | 
**skill_card_title** | Option<[**crate::models::TranscriptSkillCardSkillCardTitle**](TranscriptSkillCard_skill_card_title.md)> |  | [optional]
**skill** | [**crate::models::TranscriptSkillCardSkill**](TranscriptSkillCard_skill.md) |  | 
**invocation** | [**crate::models::TranscriptSkillCardInvocation**](TranscriptSkillCard_invocation.md) |  | 
**duration** | Option<**i32**> | An optional total duration in seconds.  Used with a `skill_card_type` of `transcript` or `timeline`. | [optional]
**entries** | [**Vec<crate::models::TranscriptSkillCardEntriesInner>**](TranscriptSkillCard_entries_inner.md) | An list of entries for the card. This represents the individual entries of the transcription. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


