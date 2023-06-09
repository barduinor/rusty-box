# SignRequestSigner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Email address of the signer | 
**role** | Option<**String**> | Defines the role of the signer in the sign request. A `signer` must sign the document and an `approver` must approve the document. A `final_copy_reader` only receives the final signed document and signing log. | [optional][default to Signer]
**is_in_person** | Option<**bool**> | Used in combination with an embed URL for a sender. After the sender signs, they are redirected to the next `in_person` signer. | [optional]
**order** | Option<**i32**> | Order of the signer | [optional]
**embed_url_external_user_id** | Option<**String**> | User ID for the signer in an external application responsible for authentication when accessing the embed URL. | [optional]
**redirect_url** | Option<**String**> | The URL that a signer will be redirected to after signing a document. Defining this URL overrides default or global redirect URL settings for a specific signer. If no declined redirect URL is specified, this URL will be used for decline actions as well. | [optional]
**declined_redirect_url** | Option<**String**> | The URL that a signer will be redirect to after declining to sign a document. Defining this URL overrides default or global declined redirect URL settings for a specific signer. | [optional]
**login_required** | Option<**bool**> | If set to true, signer will need to login to a Box account before signing the request. If the signer does not have an existing account, they will have an option to create a free Box account. | [optional]
**verification_phone_number** | Option<**String**> | If set, this phone number is be used to verify the signer via two factor authentication before they are able to sign the document. | [optional]
**password** | Option<**String**> | If set, the signer is required to enter the password before they are able to sign a document. This field is write only. | [optional]
**has_viewed_document** | Option<**bool**> | Set to `true` if the signer views the document | [optional][readonly]
**signer_decision** | Option<[**crate::models::SignRequestSignerAllOfSignerDecision**](SignRequestSigner_allOf_signer_decision.md)> |  | [optional]
**inputs** | Option<[**Vec<crate::models::SignRequestSignerInput>**](SignRequestSignerInput.md)> |  | [optional][readonly]
**embed_url** | Option<**String**> | URL to direct a signer to for signing | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


