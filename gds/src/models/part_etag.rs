/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartEtag : PartEtag

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartEtag {
    /// Part number
    #[serde(rename = "part", skip_serializing_if = "Option::is_none")]
    pub part: Option<i32>,
    /// Etag response for the part upload
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}

impl PartEtag {
    /// PartEtag
    pub fn new() -> PartEtag {
        PartEtag {
            part: None,
            etag: None,
        }
    }
}
