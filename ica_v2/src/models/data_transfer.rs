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
pub struct DataTransfer {
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
    #[serde(rename = "reference")]
    pub reference: String,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "connector", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connector: Option<Option<Box<crate::models::Connector>>>,
    #[serde(rename = "protocol", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Option<Protocol>>,
    /// The data transferred so far in bytes.
    #[serde(rename = "dataTransferred")]
    pub data_transferred: i64,
    #[serde(rename = "status")]
    pub status: Status,
    /// A message explaining the reason why the transfer is in the current status.
    #[serde(rename = "statusMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<Option<String>>,
    /// The overall duration of of the transfer defined in seconds.
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<i64>>,
    #[serde(rename = "project", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project: Option<Option<Box<crate::models::Project>>>,
    #[serde(rename = "data")]
    pub data: Box<crate::models::Data>,
}

impl DataTransfer {
    pub fn new(id: uuid::Uuid, time_created: String, time_modified: String, owner_id: uuid::Uuid, tenant_id: uuid::Uuid, reference: String, direction: Direction, data_transferred: i64, status: Status, data: crate::models::Data) -> DataTransfer {
        DataTransfer {
            id,
            time_created,
            time_modified,
            owner_id,
            tenant_id,
            tenant_name: None,
            reference,
            direction,
            connector: None,
            protocol: None,
            data_transferred,
            status,
            status_message: None,
            duration: None,
            project: None,
            data: Box::new(data),
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "UPLOAD")]
    Upload,
    #[serde(rename = "DOWNLOAD")]
    Download,
    #[serde(rename = "IMPORT")]
    Import,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Upload
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "HTTPS")]
    Https,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Https
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "ONGOING")]
    Ongoing,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "ABORTED")]
    Aborted,
    #[serde(rename = "ABORTREQUESTED")]
    Abortrequested,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Requested
    }
}

