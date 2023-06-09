/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// FileFullAllOfClassificationAllOf : The classification applied to an item



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FileFullAllOfClassificationAllOf {
    /// The name of the classification
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An explanation of the meaning of this classification.
    #[serde(rename = "definition", skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// The color that is used to display the classification label in a user-interface. Colors are defined by the admin or co-admin who created the classification in the Box web app.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl FileFullAllOfClassificationAllOf {
    /// The classification applied to an item
    pub fn new() -> FileFullAllOfClassificationAllOf {
        FileFullAllOfClassificationAllOf {
            name: None,
            definition: None,
            color: None,
        }
    }
}


