/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// FailWorkflowSignalRequest : Fail workflow signal request

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FailWorkflowSignalRequest {
    /// Externally provided Error of a signalling action.
    #[serde(rename = "error")]
    pub error: String,
    /// Externally provided Cause of a failed signalling action.
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

impl FailWorkflowSignalRequest {
    /// Fail workflow signal request
    pub fn new(error: String) -> FailWorkflowSignalRequest {
        FailWorkflowSignalRequest { error, cause: None }
    }
}
