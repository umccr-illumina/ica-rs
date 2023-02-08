/*
 * ICA Rest API
 *
 * This API can be used to interact with Illumina Connected Analytics.<br> <p> Authentication to the  API can be done in multiple ways:<br> <ul><li>For the entire API, except for the POST /tokens endpoint: API-key + JWT</li> <li>Only for the POST /tokens endpoint: API-key + Basic Authentication</li></ul> </p> <p> <b>API-key</b><br> API keys are managed within the Illumina portal where you can manage your profile after you have logged on. The API-key has to be provided in the X-API-Key header parameter when executing API calls to ICA. In the background, a JWT will be requested at the IDP of Illumina to create a session. A good practice is to not use the API-key for every API call, but to first generate a JWT and to use that for authentication in subsequent calls.<br> </p> <p> <b>JWT</b><br> To avoid using an API-key for each call, we recommend to request a JWT via the POST /tokens endpoint  using this API-key. The JWT will expire after a pre-configured period specified by a tenant administrator through the IAM console in the Illumina portal. The JWT is the preferred way for authentication.<br>A not yet expired, still valid JWT could be refreshed using the POST /tokens:refresh endpoint.<br> </p> <p> <b>Basic Authentication</b><br> Basic authentication is only supported by the POST /tokens endpoint for generating a JWT. Use \"Basic base64encoded(emailaddress:password)\" in the \"Authorization\" header parameter for this authentication method. In case having access to multiple tenants using the same email-address, also provide the \"tenant\" request parameter to indicate what tenant you would like to request a JWT for. </p> 
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DataDetails : The details of this data. This object is optional because it is possible that these details are deleted.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataDetails {
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "timeModified")]
    pub time_modified: String,
    #[serde(rename = "creatorId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    #[serde(rename = "tenantName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<Option<String>>,
    #[serde(rename = "owningProjectId")]
    pub owning_project_id: uuid::Uuid,
    #[serde(rename = "owningProjectName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owning_project_name: Option<Option<String>>,
    /// The name of the file/folder as it was uploaded.
    #[serde(rename = "name")]
    pub name: String,
    /// The user friendly path of the parent of this data.
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    /// The size of the file in bytes. Folders do not have a size.
    #[serde(rename = "fileSizeInBytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub file_size_in_bytes: Option<Option<i64>>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "tags")]
    pub tags: Box<crate::models::DataTag>,
    #[serde(rename = "format", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub format: Option<Option<Box<crate::models::DataFormat>>>,
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    /// The file's ETag, as received from the cloud provider. Not to be confused with the ETag reponse header of this API.
    #[serde(rename = "objectETag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub object_e_tag: Option<Option<String>>,
    /// Specifies when the data object was stored for the first time
    #[serde(rename = "storedForTheFirstTimeAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stored_for_the_first_time_at: Option<Option<String>>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Box<crate::models::Region>>,
    /// Specifies when the data object will be archived.
    #[serde(rename = "willBeArchivedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub will_be_archived_at: Option<Option<String>>,
    /// Specifies when the data object will be deleted.
    #[serde(rename = "willBeDeletedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub will_be_deleted_at: Option<Option<String>>,
    #[serde(rename = "sequencingRun", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sequencing_run: Option<Option<Box<crate::models::SequencingRun>>>,
}

impl DataDetails {
    /// The details of this data. This object is optional because it is possible that these details are deleted.
    pub fn new(time_created: String, time_modified: String, tenant_id: uuid::Uuid, owning_project_id: uuid::Uuid, name: String, status: Status, tags: crate::models::DataTag, data_type: DataType) -> DataDetails {
        DataDetails {
            time_created,
            time_modified,
            creator_id: None,
            tenant_id,
            tenant_name: None,
            owning_project_id,
            owning_project_name: None,
            name,
            path: None,
            file_size_in_bytes: None,
            status,
            tags: Box::new(tags),
            format: None,
            data_type,
            object_e_tag: None,
            stored_for_the_first_time_at: None,
            region: None,
            will_be_archived_at: None,
            will_be_deleted_at: None,
            sequencing_run: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "PARTIAL")]
    Partial,
    #[serde(rename = "AVAILABLE")]
    Available,
    #[serde(rename = "ARCHIVING")]
    Archiving,
    #[serde(rename = "ARCHIVED")]
    Archived,
    #[serde(rename = "UNARCHIVING")]
    Unarchiving,
    #[serde(rename = "DELETING")]
    Deleting,
}

impl Default for Status {
    fn default() -> Status {
        Self::Partial
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "FOLDER")]
    Folder,
}

impl Default for DataType {
    fn default() -> DataType {
        Self::File
    }
}

