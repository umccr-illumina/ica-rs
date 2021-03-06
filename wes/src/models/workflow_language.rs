/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowLanguage : Language details about a workflow version

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowLanguage {
    /// The name of the workflow language
    #[serde(rename = "name")]
    pub name: String,
    /// The version of the workflow language, if any
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl WorkflowLanguage {
    /// Language details about a workflow version
    pub fn new(name: String) -> WorkflowLanguage {
        WorkflowLanguage {
            name,
            version: None,
        }
    }
}
