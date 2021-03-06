/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowRun : Details of a workflow run

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowRun {
    /// Unique resource ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URN of the resource
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    /// HREF to the resource
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name of the workflow run
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The time (in UTC) the workflow run started
    #[serde(rename = "timeStarted", skip_serializing_if = "Option::is_none")]
    pub time_started: Option<String>,
    /// The time (in UTC) the Workflow Run stopped
    #[serde(rename = "timeStopped", skip_serializing_if = "Option::is_none")]
    pub time_stopped: Option<String>,
    /// Workflow run status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "idempotencyKey", skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// Workflow run status summary
    #[serde(rename = "statusSummary", skip_serializing_if = "Option::is_none")]
    pub status_summary: Option<String>,
    /// Error for a failed workflow run
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Error cause for a failed workflow run
    #[serde(rename = "errorCause", skip_serializing_if = "Option::is_none")]
    pub error_cause: Option<String>,
    #[serde(rename = "workflowVersion", skip_serializing_if = "Option::is_none")]
    pub workflow_version: Option<Box<crate::models::WorkflowVersionCompact>>,
    /// Client ID of the Origin Request
    #[serde(rename = "createdByClientId", skip_serializing_if = "Option::is_none")]
    pub created_by_client_id: Option<String>,
    /// Input to workflow run, as JSON
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    /// Output from workflow run, as JSON
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    /// Definition of the workflow version
    #[serde(rename = "definition", skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// Workflow Engine Parameters
    #[serde(rename = "engineParameters", skip_serializing_if = "Option::is_none")]
    pub engine_parameters: Option<String>,
    /// Time (in UTC) the resource was created
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    /// Time (in UTC) the resource was modified
    #[serde(rename = "timeModified", skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    /// User that created the resource
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// User that modified the resource
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// Tenant ID the resource belongs to
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Access control list of the resource
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
}

impl WorkflowRun {
    /// Details of a workflow run
    pub fn new() -> WorkflowRun {
        WorkflowRun {
            id: None,
            urn: None,
            href: None,
            name: None,
            time_started: None,
            time_stopped: None,
            status: None,
            idempotency_key: None,
            status_summary: None,
            error: None,
            error_cause: None,
            workflow_version: None,
            created_by_client_id: None,
            input: None,
            output: None,
            definition: None,
            engine_parameters: None,
            time_created: None,
            time_modified: None,
            created_by: None,
            modified_by: None,
            tenant_id: None,
            acl: None,
        }
    }
}
