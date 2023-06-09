# PutFilesIdMetadataGlobalBoxSkillsCardsRequestInnerValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The optional date and time this card was created at. | [optional]
**r#type** | **String** | `skill_card` | 
**skill_card_type** | **String** | `status` | 
**skill_card_title** | Option<[**crate::models::StatusSkillCardSkillCardTitle**](StatusSkillCard_skill_card_title.md)> |  | [optional]
**skill** | [**crate::models::KeywordSkillCardSkill**](KeywordSkillCard_skill.md) |  | 
**invocation** | [**crate::models::KeywordSkillCardInvocation**](KeywordSkillCard_invocation.md) |  | 
**entries** | [**Vec<crate::models::TranscriptSkillCardEntriesInner>**](TranscriptSkillCard_entries_inner.md) | An list of entries for the card. This represents the individual entries of the transcription. | 
**duration** | Option<**i32**> | An optional total duration in seconds.  Used with a `skill_card_type` of `transcript` or `timeline`. | [optional]
**status** | [**crate::models::StatusSkillCardStatus**](StatusSkillCard_status.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


