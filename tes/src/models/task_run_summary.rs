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
pub struct TaskRunSummary {
    /// Global identifier for object
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Href of the object
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// URN of the resource
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusDetails", skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "typeOfResource", skip_serializing_if = "Option::is_none")]
    pub type_of_resource: Option<String>,
    #[serde(rename = "sizeOfResource", skip_serializing_if = "Option::is_none")]
    pub size_of_resource: Option<String>,
    #[serde(rename = "tierOfResource", skip_serializing_if = "Option::is_none")]
    pub tier_of_resource: Option<String>,
    #[serde(rename = "taskVersionSummary", skip_serializing_if = "Option::is_none")]
    pub task_version_summary: Option<Box<crate::models::TaskVersionSummary>>,
    /// Access Control List
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "subTenantId", skip_serializing_if = "Option::is_none")]
    pub sub_tenant_id: Option<String>,
    /// User who created the object
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Date and Time (in UTC) when object was created in TES
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    /// User who updated the object
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// Date and Time (in UTC) when object was modified in TES
    #[serde(rename = "timeModified", skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
}

impl TaskRunSummary {
    pub fn new() -> TaskRunSummary {
        TaskRunSummary {
            id: None,
            href: None,
            urn: None,
            name: None,
            description: None,
            status: None,
            status_details: None,
            type_of_resource: None,
            size_of_resource: None,
            tier_of_resource: None,
            task_version_summary: None,
            acl: None,
            tenant_id: None,
            sub_tenant_id: None,
            created_by: None,
            time_created: None,
            modified_by: None,
            time_modified: None,
        }
    }
}
