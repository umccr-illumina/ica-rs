/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// FolderCopyOperationParameters : Operation parameters for folder copy operations

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FolderCopyOperationParameters {
    /// The Urn of the source folder for the copy operation
    #[serde(rename = "sourceFolderUrn", skip_serializing_if = "Option::is_none")]
    pub source_folder_urn: Option<String>,
    /// The Urn of the target folder for the copy operation
    #[serde(rename = "targetFolderUrn", skip_serializing_if = "Option::is_none")]
    pub target_folder_urn: Option<String>,
    /// The folder name for the copied folder
    #[serde(
        rename = "destinationFolderName",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_folder_name: Option<String>,
}

impl FolderCopyOperationParameters {
    /// Operation parameters for folder copy operations
    pub fn new() -> FolderCopyOperationParameters {
        FolderCopyOperationParameters {
            source_folder_urn: None,
            target_folder_urn: None,
            destination_folder_name: None,
        }
    }
}
