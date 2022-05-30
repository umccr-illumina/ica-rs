/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JobStatus : This enum was originally created to store the type of job on a folder or file entity.   The Job entity is now used to track this information for copy operations, with the Illumina.Gds.Api.Models.Enums.JobOperationType   and Illumina.Gds.Api.Models.Enums.JobProgressStatus used to record the type and progress status of the job.

/// This enum was originally created to store the type of job on a folder or file entity.   The Job entity is now used to track this information for copy operations, with the Illumina.Gds.Api.Models.Enums.JobOperationType   and Illumina.Gds.Api.Models.Enums.JobProgressStatus used to record the type and progress status of the job.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobStatus {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Archiving")]
    Archiving,
    #[serde(rename = "Unarchiving")]
    Unarchiving,
    #[serde(rename = "Deleting")]
    Deleting,

}

impl ToString for JobStatus {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("None"),
            Self::Archiving => String::from("Archiving"),
            Self::Unarchiving => String::from("Unarchiving"),
            Self::Deleting => String::from("Deleting"),
        }
    }
}

impl Default for JobStatus {
    fn default() -> JobStatus {
        Self::None
    }
}




