/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// SignTemplateAllOfAdditionalInfo : Additional information on which fields are required and which fields are not editable.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignTemplateAllOfAdditionalInfo {
    /// Non editable fields.
    #[serde(rename = "non_editable", skip_serializing_if = "Option::is_none")]
    pub non_editable: Option<Vec<NonEditable>>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Box<crate::models::SignTemplateAllOfAdditionalInfoRequired>>,
}

impl SignTemplateAllOfAdditionalInfo {
    /// Additional information on which fields are required and which fields are not editable.
    pub fn new() -> SignTemplateAllOfAdditionalInfo {
        SignTemplateAllOfAdditionalInfo {
            non_editable: None,
            required: None,
        }
    }
}

/// Non editable fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NonEditable {
    #[serde(rename = "email_subject")]
    EmailSubject,
    #[serde(rename = "email_message")]
    EmailMessage,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "days_valid")]
    DaysValid,
    #[serde(rename = "signers")]
    Signers,
    #[serde(rename = "source_files")]
    SourceFiles,
}

impl Default for NonEditable {
    fn default() -> NonEditable {
        Self::EmailSubject
    }
}

