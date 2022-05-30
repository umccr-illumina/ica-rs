/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JobProgressStatus : The valid Job Status values for folders in GDS.

/// The valid Job Status values for folders in GDS.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobProgressStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Failed")]
    Failed,

}

impl ToString for JobProgressStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("Pending"),
            Self::InProgress => String::from("InProgress"),
            Self::Completed => String::from("Completed"),
            Self::Failed => String::from("Failed"),
        }
    }
}

impl Default for JobProgressStatus {
    fn default() -> JobProgressStatus {
        Self::Pending
    }
}




