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
pub struct BulkUpdateFailedItem {
    /// Id of resource that failed to be updated
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "errorResponse", skip_serializing_if = "Option::is_none")]
    pub error_response: Option<Box<crate::models::ErrorResponse>>,
}

impl BulkUpdateFailedItem {
    pub fn new() -> BulkUpdateFailedItem {
        BulkUpdateFailedItem {
            id: None,
            error_response: None,
        }
    }
}
