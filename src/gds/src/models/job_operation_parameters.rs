/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JobOperationParameters : Container for XXXOperationParameters objects, which have additional details related to the specific job operation type.  Only one object will be populated.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JobOperationParameters {
    #[serde(rename = "folderCopy", skip_serializing_if = "Option::is_none")]
    pub folder_copy: Option<Box<crate::models::FolderCopyOperationParameters>>,
    #[serde(rename = "folderDelete", skip_serializing_if = "Option::is_none")]
    pub folder_delete: Option<Box<crate::models::FolderDeleteOperationParameters>>,
}

impl JobOperationParameters {
    /// Container for XXXOperationParameters objects, which have additional details related to the specific job operation type.  Only one object will be populated.
    pub fn new() -> JobOperationParameters {
        JobOperationParameters {
            folder_copy: None,
            folder_delete: None,
        }
    }
}


