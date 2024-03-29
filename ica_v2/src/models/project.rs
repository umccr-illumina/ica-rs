/*
 * ICA Rest API
 *
 * This API can be used to interact with Illumina Connected Analytics.<br> <p> Authentication to the  API can be done in multiple ways:<br> <ul><li>For the entire API, except for the POST /tokens endpoint: API-key + JWT</li> <li>Only for the POST /tokens endpoint: API-key + Basic Authentication</li></ul> </p> <p> <b>API-key</b><br> API keys are managed within the Illumina portal where you can manage your profile after you have logged on. The API-key has to be provided in the X-API-Key header parameter when executing API calls to ICA. In the background, a JWT will be requested at the IDP of Illumina to create a session. A good practice is to not use the API-key for every API call, but to first generate a JWT and to use that for authentication in subsequent calls.<br> </p> <p> <b>JWT</b><br> To avoid using an API-key for each call, we recommend to request a JWT via the POST /tokens endpoint  using this API-key. The JWT will expire after a pre-configured period specified by a tenant administrator through the IAM console in the Illumina portal. The JWT is the preferred way for authentication.<br>A not yet expired, still valid JWT could be refreshed using the POST /tokens:refresh endpoint.<br> </p> <p> <b>Basic Authentication</b><br> Basic authentication is only supported by the POST /tokens endpoint for generating a JWT. Use \"Basic base64encoded(emailaddress:password)\" in the \"Authorization\" header parameter for this authentication method. In case having access to multiple tenants using the same email-address, also provide the \"tenant\" request parameter to indicate what tenant you would like to request a JWT for. </p>
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "timeModified")]
    pub time_modified: String,
    #[serde(rename = "ownerId")]
    pub owner_id: uuid::Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    #[serde(
        rename = "tenantName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tenant_name: Option<Option<String>>,
    /// The URN of the project. The format is urn:ilmn:ica:\\<type of the object\\>:\\<ID of the object\\>#\\<optional human readable hint representing the object\\>. The hint can be omitted, in that case the hashtag (#) must also be omitted.
    #[serde(
        rename = "urn",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub urn: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates whether the project is active or hidden.
    #[serde(rename = "active")]
    pub active: bool,
    /// Indicates whether the project is base enabled.
    #[serde(
        rename = "baseEnabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_enabled: Option<Option<bool>>,
    #[serde(
        rename = "shortDescription",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_description: Option<Option<String>>,
    /// Information about the project. Note that the value of this field can be arbitrary large.
    #[serde(
        rename = "information",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub information: Option<Option<String>>,
    #[serde(rename = "region")]
    pub region: Box<crate::models::Region>,
    /// The billing mode of the project. It determines who pays for the costs linked to the project.
    #[serde(rename = "billingMode")]
    pub billing_mode: BillingMode,
    /// Indicates whether the Data and Samples created in this Project can be linked to other Projects.
    #[serde(
        rename = "dataSharingEnabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_sharing_enabled: Option<Option<bool>>,
    #[serde(rename = "tags")]
    pub tags: Box<crate::models::ProjectTag>,
    #[serde(rename = "storageBundle", skip_serializing_if = "Option::is_none")]
    pub storage_bundle: Option<Box<crate::models::StorageBundle>>,
    #[serde(
        rename = "selfManagedStorageConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_managed_storage_configuration: Option<Box<crate::models::StorageConfiguration>>,
    /// Indicates the priority given to a project and its analyses within a single tenant. Note that for a PUT call, when not providing a value for this attribute (null value or absent attribute), the persisted value will not change.
    #[serde(
        rename = "analysisPriority",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub analysis_priority: Option<Option<AnalysisPriority>>,
    #[serde(
        rename = "metadataModel",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_model: Option<Option<Box<crate::models::MetadataModel>>>,
    #[serde(
        rename = "application",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub application: Option<Option<Box<crate::models::Application>>>,
}

impl Project {
    pub fn new(
        id: uuid::Uuid,
        time_created: String,
        time_modified: String,
        owner_id: uuid::Uuid,
        tenant_id: uuid::Uuid,
        name: String,
        active: bool,
        region: crate::models::Region,
        billing_mode: BillingMode,
        tags: crate::models::ProjectTag,
    ) -> Project {
        Project {
            id,
            time_created,
            time_modified,
            owner_id,
            tenant_id,
            tenant_name: None,
            urn: None,
            name,
            active,
            base_enabled: None,
            short_description: None,
            information: None,
            region: Box::new(region),
            billing_mode,
            data_sharing_enabled: None,
            tags: Box::new(tags),
            storage_bundle: None,
            self_managed_storage_configuration: None,
            analysis_priority: None,
            metadata_model: None,
            application: None,
        }
    }
}

/// The billing mode of the project. It determines who pays for the costs linked to the project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingMode {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "TENANT")]
    Tenant,
}

impl Default for BillingMode {
    fn default() -> BillingMode {
        Self::Project
    }
}
/// Indicates the priority given to a project and its analyses within a single tenant. Note that for a PUT call, when not providing a value for this attribute (null value or absent attribute), the persisted value will not change.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AnalysisPriority {
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
}

impl Default for AnalysisPriority {
    fn default() -> AnalysisPriority {
        Self::Low
    }
}
