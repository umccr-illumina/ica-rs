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
pub struct UpdateVolumeRequest {
    /// Metadata about this volume and its contents
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "lifeCycle", skip_serializing_if = "Option::is_none")]
    pub life_cycle: Option<Box<crate::models::VolumeLifeCycleSettings>>,
}

impl UpdateVolumeRequest {
    pub fn new() -> UpdateVolumeRequest {
        UpdateVolumeRequest {
            metadata: None,
            life_cycle: None,
        }
    }
}
