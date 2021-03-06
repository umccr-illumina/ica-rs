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
pub struct LaunchTaskRequest {
    /// User-defined name for the task run, if not specified version string of task version will be used
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined description for the task run, if not specified task version description will be used
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Arguments to launch a task run
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<::std::collections::HashMap<String, String>>,
    /// Access Control List
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
}

impl LaunchTaskRequest {
    pub fn new() -> LaunchTaskRequest {
        LaunchTaskRequest {
            name: None,
            description: None,
            arguments: None,
            acl: None,
        }
    }
}
