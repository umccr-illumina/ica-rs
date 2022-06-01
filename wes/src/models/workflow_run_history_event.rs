/*
 * Workflow Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowRunHistoryEvent : Information about a specific event in the workflow run history



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowRunHistoryEvent {
    /// Name of the event, such as the name of the step/task for state-level events and run name for run-level events
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier for the history event, if any
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    /// Identifier for any previous history event (if available)
    #[serde(rename = "previousEventId", skip_serializing_if = "Option::is_none")]
    pub previous_event_id: Option<i64>,
    /// Type of history event. The associated details entry will be populated based on the type of event.
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// Timestamp for the history event
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Details for history event
    #[serde(rename = "eventDetails", skip_serializing_if = "Option::is_none")]
    pub event_details: Option<serde_json::Value>,
}

impl WorkflowRunHistoryEvent {
    /// Information about a specific event in the workflow run history
    pub fn new() -> WorkflowRunHistoryEvent {
        WorkflowRunHistoryEvent {
            name: None,
            event_id: None,
            previous_event_id: None,
            event_type: None,
            timestamp: None,
            event_details: None,
        }
    }
}

