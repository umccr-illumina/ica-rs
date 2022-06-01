/*
 * Developer Console Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "timeModified", skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            id: None,
            urn: None,
            name: None,
            owner: None,
            time_created: None,
            time_modified: None,
        }
    }
}
