/*
 * Developer Console Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// SystemHealthResponse : The overall health of the platform

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SystemHealthResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::HealthCheckStatuses>,
    /// Service health details
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::ServiceHealthResponse>>,
}

impl SystemHealthResponse {
    /// The overall health of the platform
    pub fn new() -> SystemHealthResponse {
        SystemHealthResponse {
            status: None,
            details: None,
        }
    }
}