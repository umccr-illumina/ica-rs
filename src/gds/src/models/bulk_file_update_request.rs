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
pub struct BulkFileUpdateRequest {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::BulkFileUpdateItem>>,
    #[serde(rename = "timeOutInSeconds", skip_serializing_if = "Option::is_none")]
    pub time_out_in_seconds: Option<i32>,
}

impl BulkFileUpdateRequest {
    pub fn new() -> BulkFileUpdateRequest {
        BulkFileUpdateRequest {
            items: None,
            time_out_in_seconds: None,
        }
    }
}


