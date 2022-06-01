/*
 * Developer Console Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProductUsage {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "iCredit", skip_serializing_if = "Option::is_none")]
    pub i_credit: Option<f64>,
}

impl ProductUsage {
    pub fn new(_type: String, amount: f64, unit: String) -> ProductUsage {
        ProductUsage {
            _type,
            amount,
            unit,
            i_credit: None,
        }
    }
}
