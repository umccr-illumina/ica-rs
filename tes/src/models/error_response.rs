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
pub struct ErrorResponse {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<serde_json::Value>>,
}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ErrorResponse {
            code: None,
            message: None,
            details: None,
        }
    }
}

