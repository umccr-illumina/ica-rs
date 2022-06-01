/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateWorkflowRequest : Create a workflow and optional version

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWorkflowRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "workflowVersion", skip_serializing_if = "Option::is_none")]
    pub workflow_version: Option<Box<crate::models::CreateWorkflowVersionRequest>>,
    #[serde(rename = "toolClass", skip_serializing_if = "Option::is_none")]
    pub tool_class: Option<ToolClass>,
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
}

impl CreateWorkflowRequest {
    /// Create a workflow and optional version
    pub fn new(name: String) -> CreateWorkflowRequest {
        CreateWorkflowRequest {
            name,
            description: None,
            organization: None,
            workflow_version: None,
            tool_class: None,
            acl: None,
            categories: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ToolClass {
    #[serde(rename = "workflow")]
    Workflow,
    #[serde(rename = "commandLineTool")]
    CommandLineTool,
}

impl Default for ToolClass {
    fn default() -> ToolClass {
        Self::Workflow
    }
}
