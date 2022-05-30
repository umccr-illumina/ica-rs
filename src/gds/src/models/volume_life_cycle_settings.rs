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
pub struct VolumeLifeCycleSettings {
    /// Number of days before the files associated to this volume expires
    #[serde(rename = "gracePeriodDays", skip_serializing_if = "Option::is_none")]
    pub grace_period_days: Option<i32>,
    #[serde(rename = "gracePeriodEndAction", skip_serializing_if = "Option::is_none")]
    pub grace_period_end_action: Option<crate::models::GracePeriodEndAction>,
}

impl VolumeLifeCycleSettings {
    pub fn new() -> VolumeLifeCycleSettings {
        VolumeLifeCycleSettings {
            grace_period_days: None,
            grace_period_end_action: None,
        }
    }
}


