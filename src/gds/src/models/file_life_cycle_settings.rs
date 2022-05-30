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
pub struct FileLifeCycleSettings {
    /// If set, date when billing for the file will start
    #[serde(rename = "timeGracePeriodEnds", skip_serializing_if = "Option::is_none")]
    pub time_grace_period_ends: Option<String>,
    /// If set, date when file will be archived
    #[serde(rename = "timeToBeArchived", skip_serializing_if = "Option::is_none")]
    pub time_to_be_archived: Option<String>,
    /// If set, date when file will be deleted
    #[serde(rename = "timeToBeDeleted", skip_serializing_if = "Option::is_none")]
    pub time_to_be_deleted: Option<String>,
    #[serde(rename = "archiveStorageTier", skip_serializing_if = "Option::is_none")]
    pub archive_storage_tier: Option<crate::models::FileArchiveStorageTier>,
}

impl FileLifeCycleSettings {
    pub fn new() -> FileLifeCycleSettings {
        FileLifeCycleSettings {
            time_grace_period_ends: None,
            time_to_be_archived: None,
            time_to_be_deleted: None,
            archive_storage_tier: None,
        }
    }
}


