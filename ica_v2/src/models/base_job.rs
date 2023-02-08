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
pub struct BaseJob {
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
    #[serde(rename = "tenantName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<Option<String>>,
    /// A short description of the base job
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "table", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub table: Option<Option<Box<crate::models::ProjectBaseTable>>>,
    /// The type of the job
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The status of the job
    #[serde(rename = "status")]
    pub status: Status,
    /// The duration of the job expressed in milliseconds
    #[serde(rename = "overallDuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overall_duration: Option<Option<i64>>,
    /// Detailed description of the job
    #[serde(rename = "details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub details: Option<Option<String>>,
    /// Bytes billed
    #[serde(rename = "bytesBilled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bytes_billed: Option<Option<i64>>,
}

impl BaseJob {
    pub fn new(id: uuid::Uuid, time_created: String, time_modified: String, owner_id: uuid::Uuid, tenant_id: uuid::Uuid, r#type: RHashType, status: Status) -> BaseJob {
        BaseJob {
            id,
            time_created,
            time_modified,
            owner_id,
            tenant_id,
            tenant_name: None,
            description: None,
            table: None,
            r#type,
            status,
            overall_duration: None,
            details: None,
            bytes_billed: None,
        }
    }
}

/// The type of the job
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "COPYTABLE")]
    Copytable,
    #[serde(rename = "EXPORTTABLE")]
    Exporttable,
    #[serde(rename = "CREATETABLE")]
    Createtable,
    #[serde(rename = "EXECUTEQUERY")]
    Executequery,
    #[serde(rename = "LOADDATA")]
    Loaddata,
    #[serde(rename = "PREPAREDATA")]
    Preparedata,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Copytable
    }
}
/// The status of the job
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "INPROGRESS")]
    Inprogress,
    #[serde(rename = "ABORTED")]
    Aborted,
}

impl Default for Status {
    fn default() -> Status {
        Self::Created
    }
}

