/*
 * Task Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskSummaryPagedItems {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::TaskSummary>>,
    #[serde(rename = "itemCount", skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "firstPageToken", skip_serializing_if = "Option::is_none")]
    pub first_page_token: Option<String>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "prevPageToken", skip_serializing_if = "Option::is_none")]
    pub prev_page_token: Option<String>,
    #[serde(rename = "lastPageToken", skip_serializing_if = "Option::is_none")]
    pub last_page_token: Option<String>,
    #[serde(rename = "totalItemCount", skip_serializing_if = "Option::is_none")]
    pub total_item_count: Option<i64>,
    #[serde(rename = "totalPageCount", skip_serializing_if = "Option::is_none")]
    pub total_page_count: Option<i64>,
}

impl TaskSummaryPagedItems {
    pub fn new() -> TaskSummaryPagedItems {
        TaskSummaryPagedItems {
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


