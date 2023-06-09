# TemplateSigner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inputs** | Option<[**Vec<crate::models::TemplateSignerInput>**](TemplateSignerInput.md)> |  | [optional][readonly]
**email** | Option<**String**> | Email address of the signer | [optional]
**role** | Option<**String**> | Defines the role of the signer in the signature request. A role of `signer` needs to sign the document, a role `approver` approves the document and a `final_copy_reader` role only receives the final signed document and signing log. | [optional][default to Signer]
**is_in_person** | Option<**bool**> | Used in combination with an embed URL for a sender. After the sender signs, they will be redirected to the next `in_person` signer. | [optional]
**order** | Option<**i32**> | Order of the signer | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


