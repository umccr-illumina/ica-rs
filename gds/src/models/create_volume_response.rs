/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateVolumeResponse {
    /// Unique identifier of the import Session for this Volume. This only applies to Volumes created from custom  Volume configurations.
    #[serde(rename = "importSessionId", skip_serializing_if = "Option::is_none")]
    pub import_session_id: Option<String>,
    #[serde(rename = "objectStoreAccess", skip_serializing_if = "Option::is_none")]
    pub object_store_access: Option<Box<crate::models::ObjectStoreAccess>>,
    /// A unique identifier for this Volume
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of this Volume
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique identifier for this Volume's Tenant
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// The unique identifier for this Volume's Sub Tenant
    #[serde(rename = "subTenantId", skip_serializing_if = "Option::is_none")]
    pub sub_tenant_id: Option<String>,
    /// The Universal Resource Name, unique to this Volume
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    /// The unique identifier for the root Folder of this Volume
    #[serde(rename = "rootFolderId", skip_serializing_if = "Option::is_none")]
    pub root_folder_id: Option<String>,
    /// The base bucket location for Volumes associated with custom VolumeConfigurations otherwise this field is not set.
    #[serde(rename = "rootKeyPrefix", skip_serializing_if = "Option::is_none")]
    pub root_key_prefix: Option<String>,
    /// Unique name of the Volume configuration for this Volume.  This field will only be set if a custom Volume configuration is associated.
    #[serde(
        rename = "volumeConfigurationName",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_configuration_name: Option<String>,
    /// The inherited list of Id(s) that have access to this Volume
    #[serde(rename = "inheritedAcl", skip_serializing_if = "Option::is_none")]
    pub inherited_acl: Option<Vec<String>>,
    /// The date & time this Volume was created, in GDS
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    /// The creator of this Volume
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The date & time this Volume was updated, in GDS
    #[serde(rename = "timeModified", skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    /// The updator of this Volume
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "jobStatus", skip_serializing_if = "Option::is_none")]
    pub job_status: Option<crate::models::JobStatus>,
    /// Metadata about this Volume
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "lifeCycle", skip_serializing_if = "Option::is_none")]
    pub life_cycle: Option<Box<crate::models::VolumeLifeCycleSettings>>,
}

impl CreateVolumeResponse {
    pub fn new() -> CreateVolumeResponse {
        CreateVolumeResponse {
            import_session_id: None,
            object_store_access: None,
            id: None,
            name: None,
            tenant_id: None,
            sub_tenant_id: None,
            urn: None,
            root_folder_id: None,
            root_key_prefix: None,
            volume_configuration_name: None,
            inherited_acl: None,
            time_created: None,
            created_by: None,
            time_modified: None,
            modified_by: None,
            job_status: None,
            metadata: None,
            life_cycle: None,
        }
    }
}
