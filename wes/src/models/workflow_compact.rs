/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowCompact : Compact details of a workflow

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowCompact {
    /// Unique resource ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URN of the resource
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    /// HREF to the resource
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name of the workflow
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Organization associated with the workflow
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Description of the workflow
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Workflow type
    #[serde(rename = "toolClass", skip_serializing_if = "Option::is_none")]
    pub tool_class: Option<String>,
    /// Categories of the workflow (Limit Max Size : 10)
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
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

impl WorkflowCompact {
    /// Compact details of a workflow
    pub fn new() -> WorkflowCompact {
        WorkflowCompact {
            id: None,
            urn: None,
            href: None,
            name: None,
            organization: None,
            description: None,
            tool_class: None,
            categories: None,
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
