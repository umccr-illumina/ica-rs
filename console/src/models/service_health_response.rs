/*
 * Developer Console Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// ServiceHealthResponse : Describes the individual health of all services in the platform

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceHealthResponse {
    /// Name of the service
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::HealthCheckStatuses>,
    /// Current version for the deployed service
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ServiceHealthResponse {
    /// Describes the individual health of all services in the platform
    pub fn new() -> ServiceHealthResponse {
        ServiceHealthResponse {
            name: None,
            status: None,
            version: None,
        }
    }
}
