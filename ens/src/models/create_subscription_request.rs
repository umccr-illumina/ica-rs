/*
 * Event Notification Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    /// Event type which will be subscribed to
    #[serde(rename = "type")]
    pub _type: String,
    /// Actions which will be subscribed to for the event type
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// Name of the subscription
    #[serde(rename = "name")]
    pub name: String,
    /// Optional description for the subscription
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON-structured filter expression for events matching the subscription
    #[serde(rename = "filterExpression", skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "deliveryTarget")]
    pub delivery_target: Box<crate::models::DeliveryTarget>,
}

impl CreateSubscriptionRequest {
    pub fn new(
        _type: String,
        name: String,
        delivery_target: crate::models::DeliveryTarget,
    ) -> CreateSubscriptionRequest {
        CreateSubscriptionRequest {
            _type,
            actions: None,
            name,
            description: None,
            filter_expression: None,
            delivery_target: Box::new(delivery_target),
        }
    }
}
