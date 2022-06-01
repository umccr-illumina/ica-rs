/*
 * Event Notification Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeliveryTarget : Configuration for the action to perform for events matching this subscription; only one delivery type may be specified

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeliveryTarget {
    #[serde(rename = "awsSnsTopic", skip_serializing_if = "Option::is_none")]
    pub aws_sns_topic: Option<Box<crate::models::DeliveryTargetAwsSnsTopic>>,
    #[serde(rename = "awsSqsQueue", skip_serializing_if = "Option::is_none")]
    pub aws_sqs_queue: Option<Box<crate::models::DeliveryTargetAwsSqsQueue>>,
    #[serde(rename = "workflowRunLaunch", skip_serializing_if = "Option::is_none")]
    pub workflow_run_launch: Option<Box<crate::models::DeliveryTargetWorkflowRunLaunch>>,
}

impl DeliveryTarget {
    /// Configuration for the action to perform for events matching this subscription; only one delivery type may be specified
    pub fn new() -> DeliveryTarget {
        DeliveryTarget {
            aws_sns_topic: None,
            aws_sqs_queue: None,
            workflow_run_launch: None,
        }
    }
}
