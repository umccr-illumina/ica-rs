/*
 * Developer Console Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HealthCheckStatuses : Health status

/// Health status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HealthCheckStatuses {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,

}

impl ToString for HealthCheckStatuses {
    fn to_string(&self) -> String {
        match self {
            Self::_0 => String::from("0"),
            Self::_1 => String::from("1"),
            Self::_2 => String::from("2"),
            Self::_3 => String::from("3"),
        }
    }
}

impl Default for HealthCheckStatuses {
    fn default() -> HealthCheckStatuses {
        Self::_0
    }
}




