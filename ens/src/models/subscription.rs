/*
 * Event Notification Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// Subscription : Details for an Event Notification Service subscription

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique id of the subscription
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URN of the subscription
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    /// Type of event the subscription matches
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Types of actions the subscription matches for the event type
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// JSON-structured filter expression for events matching the subscription
    #[serde(rename = "filterExpression", skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// Name of the subscription
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional description for the subscription
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "deliveryTarget", skip_serializing_if = "Option::is_none")]
    pub delivery_target: Option<Box<crate::models::DeliveryTarget>>,
    /// ACL Identities for events the subscription matches
    #[serde(rename = "matchIdentities", skip_serializing_if = "Option::is_none")]
    pub match_identities: Option<Vec<String>>,
    /// The list of identities that have access to this subscription
    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<String>>,
    /// Tenant id of the subscription owner
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// User id for the creator of the subscription
    #[serde(rename = "createdByUserId", skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    /// Timestamp when the subscription was created
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    /// Id of the user who deleted the subscription, if applicable
    #[serde(rename = "deletedByUserId", skip_serializing_if = "Option::is_none")]
    pub deleted_by_user_id: Option<String>,
    /// Timestamp when the subscription was deleted, if applicable
    #[serde(rename = "timeDeleted", skip_serializing_if = "Option::is_none")]
    pub time_deleted: Option<String>,
    /// Whether or not the subscription has been deleted
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}

impl Subscription {
    /// Details for an Event Notification Service subscription
    pub fn new() -> Subscription {
        Subscription {
            id: None,
            urn: None,
            _type: None,
            actions: None,
            filter_expression: None,
            name: None,
            description: None,
            delivery_target: None,
            match_identities: None,
            acl: None,
            tenant_id: None,
            created_by_user_id: None,
            time_created: None,
            deleted_by_user_id: None,
            time_deleted: None,
            is_deleted: None,
        }
    }
}
