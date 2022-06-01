/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowSignalCompact : Compact details of a signal

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowSignalCompact {
    /// Unique resource ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URN of the resource
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    /// HREF to the resource
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// HREF to send a heartbeat to a workflow signal
    #[serde(rename = "sendHeartbeatHref", skip_serializing_if = "Option::is_none")]
    pub send_heartbeat_href: Option<String>,
    /// HREF to succeed a workflow signal
    #[serde(
        rename = "sendSuccessResponseHref",
        skip_serializing_if = "Option::is_none"
    )]
    pub send_success_response_href: Option<String>,
    /// HREF to fail a workflow signal
    #[serde(
        rename = "sendFailureResponseHref",
        skip_serializing_if = "Option::is_none"
    )]
    pub send_failure_response_href: Option<String>,
    /// Unique name of the signal
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Current status of the signal
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// User-defined type associated with the signal
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Description of the signal
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Inputs defined by the originating WaitForSignal state, in JSON.
    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[serde(rename = "workflowRun", skip_serializing_if = "Option::is_none")]
    pub workflow_run: Option<Box<crate::models::WorkflowRunCompact>>,
    /// Signal timeout in seconds. The Signal will timeout if a heartbeat, succeed or fail is not received in this time interval.
    #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// The result of a successful signalling action in JSON.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// The error of a failed signal.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The error cause of a failed signal.
    #[serde(rename = "errorCause", skip_serializing_if = "Option::is_none")]
    pub error_cause: Option<String>,
    /// Client ID of the Origin Request
    #[serde(rename = "createdByClientId", skip_serializing_if = "Option::is_none")]
    pub created_by_client_id: Option<String>,
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

impl WorkflowSignalCompact {
    /// Compact details of a signal
    pub fn new() -> WorkflowSignalCompact {
        WorkflowSignalCompact {
            id: None,
            urn: None,
            href: None,
            send_heartbeat_href: None,
            send_success_response_href: None,
            send_failure_response_href: None,
            name: None,
            status: None,
            _type: None,
            description: None,
            inputs: None,
            workflow_run: None,
            timeout_seconds: None,
            result: None,
            error: None,
            error_cause: None,
            created_by_client_id: None,
            time_created: None,
            time_modified: None,
            created_by: None,
            modified_by: None,
            tenant_id: None,
            acl: None,
        }
    }
}
