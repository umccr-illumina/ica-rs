/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateWorkflowRequest : Update an existing workflow

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateWorkflowRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
}

impl UpdateWorkflowRequest {
    /// Update an existing workflow
    pub fn new() -> UpdateWorkflowRequest {
        UpdateWorkflowRequest {
            name: None,
            description: None,
            organization: None,
            acl: None,
            categories: None,
        }
    }
}
