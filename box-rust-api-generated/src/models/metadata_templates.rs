/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// MetadataTemplates : A list of metadata templates



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataTemplates {
    /// The limit that was used for these entries. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed. The maximum value varies by API.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The marker for the start of the next page of results.
    #[serde(rename = "next_marker", skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<i64>,
    /// The marker for the start of the previous page of results.
    #[serde(rename = "prev_marker", skip_serializing_if = "Option::is_none")]
    pub prev_marker: Option<i64>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::MetadataTemplate>>,
}

impl MetadataTemplates {
    /// A list of metadata templates
    pub fn new() -> MetadataTemplates {
        MetadataTemplates {
            limit: None,
            next_marker: None,
            prev_marker: None,
            entries: None,
        }
    }
}


