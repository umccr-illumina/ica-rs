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
pub struct VolumeConfigurationListResponse {
    /// Items in paged list
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::VolumeConfigurationResponse>>,
    /// Number of items included in the page
    #[serde(rename = "itemCount", skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    /// PageToken for first paged list
    #[serde(rename = "firstPageToken", skip_serializing_if = "Option::is_none")]
    pub first_page_token: Option<String>,
    /// PageToken for the next paged list
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// PageToken for the previous paged list
    #[serde(rename = "prevPageToken", skip_serializing_if = "Option::is_none")]
    pub prev_page_token: Option<String>,
    /// PageToken for the last paged list. Only included when totalItemCount is listed
    #[serde(rename = "lastPageToken", skip_serializing_if = "Option::is_none")]
    pub last_page_token: Option<String>,
    /// Total number of items in all pages. Only included when requested
    #[serde(rename = "totalItemCount", skip_serializing_if = "Option::is_none")]
    pub total_item_count: Option<i64>,
    /// Total number of pages. Only included when totalItemCount is listed
    #[serde(rename = "totalPageCount", skip_serializing_if = "Option::is_none")]
    pub total_page_count: Option<i64>,
}

impl VolumeConfigurationListResponse {
    pub fn new() -> VolumeConfigurationListResponse {
        VolumeConfigurationListResponse {
            items: None,
            item_count: None,
            first_page_token: None,
            next_page_token: None,
            prev_page_token: None,
            last_page_token: None,
            total_item_count: None,
            total_page_count: None,
        }
    }
}
