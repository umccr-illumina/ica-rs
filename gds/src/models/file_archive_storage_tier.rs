/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileArchiveStorageTier {
    #[serde(rename = "Archive")]
    Archive,
    #[serde(rename = "DeepArchive")]
    DeepArchive,
}

impl ToString for FileArchiveStorageTier {
    fn to_string(&self) -> String {
        match self {
            Self::Archive => String::from("Archive"),
            Self::DeepArchive => String::from("DeepArchive"),
        }
    }
}

impl Default for FileArchiveStorageTier {
    fn default() -> FileArchiveStorageTier {
        Self::Archive
    }
}
