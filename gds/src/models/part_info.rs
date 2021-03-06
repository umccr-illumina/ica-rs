/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartInfo : PartInfo of multi parts presigned url for upload

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartInfo {
    /// Part number to be applied to {part} in MultipartTemplate
    #[serde(rename = "part", skip_serializing_if = "Option::is_none")]
    pub part: Option<i32>,
    /// Date to be applied to {date} in MultipartTemplate
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// DateTime to be applied to {datetime} in MultipartTemplate
    #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    /// Signature to be applied to {signature} in MultipartTemplate
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl PartInfo {
    /// PartInfo of multi parts presigned url for upload
    pub fn new() -> PartInfo {
        PartInfo {
            part: None,
            date: None,
            date_time: None,
            signature: None,
        }
    }
}
