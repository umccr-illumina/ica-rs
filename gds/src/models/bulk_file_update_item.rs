/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkFileUpdateItem {
    /// Id of this file
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Metadata about this file and its contents
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// The File's Format
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The File's Edam Format
    #[serde(rename = "formatEdam", skip_serializing_if = "Option::is_none")]
    pub format_edam: Option<String>,
    #[serde(rename = "lifeCycle", skip_serializing_if = "Option::is_none")]
    pub life_cycle: Option<Box<crate::models::FileLifeCycleSettings>>,
}

impl BulkFileUpdateItem {
    pub fn new() -> BulkFileUpdateItem {
        BulkFileUpdateItem {
            id: None,
            metadata: None,
            format: None,
            format_edam: None,
            life_cycle: None,
        }
    }
}
