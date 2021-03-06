/*
 * Task Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Image {
    /// Name of url for Docker Image
    #[serde(rename = "name")]
    pub name: String,
    /// Version of image as defined in repository
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Version of image as defined in repository
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Box<crate::models::Credentials>>,
}

impl Image {
    pub fn new(name: String) -> Image {
        Image {
            name,
            tag: None,
            digest: None,
            credentials: None,
        }
    }
}
