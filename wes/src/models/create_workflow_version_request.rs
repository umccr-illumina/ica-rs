/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateWorkflowVersionRequest : Create a new workflow version under an existing workflow root

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWorkflowVersionRequest {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<crate::models::WorkflowLanguage>>,
    #[serde(rename = "definition", skip_serializing_if = "Option::is_none")]
    pub definition: Option<serde_json::Value>,
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
    /// Published Status of the workflow version
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl CreateWorkflowVersionRequest {
    /// Create a new workflow version under an existing workflow root
    pub fn new(version: String) -> CreateWorkflowVersionRequest {
        CreateWorkflowVersionRequest {
            version,
            description: None,
            language: None,
            definition: None,
            acl: None,
            status: None,
        }
    }
}

/// Published Status of the workflow version
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "released")]
    Released,
    #[serde(rename = "obsolete")]
    Obsolete,
}

impl Default for Status {
    fn default() -> Status {
        Self::Draft
    }
}
