# TimelineSkillCard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The optional date and time this card was created at. | [optional]
**r#type** | **String** | `skill_card` | 
**skill_card_type** | **String** | `timeline` | 
**skill_card_title** | Option<[**crate::models::TimelineSkillCardSkillCardTitle**](TimelineSkillCard_skill_card_title.md)> |  | [optional]
**skill** | [**crate::models::KeywordSkillCardSkill**](KeywordSkillCard_skill.md) |  | 
**invocation** | [**crate::models::KeywordSkillCardInvocation**](KeywordSkillCard_invocation.md) |  | 
**duration** | Option<**i32**> | An total duration in seconds of the timeline. | [optional]
**entries** | [**Vec<crate::models::TimelineSkillCardEntriesInner>**](TimelineSkillCard_entries_inner.md) | A list of entries on the timeline. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


