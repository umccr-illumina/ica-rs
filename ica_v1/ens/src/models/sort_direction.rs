/*
 * Event Notification Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortDirection {
    #[serde(rename = "Asc")]
    Asc,
    #[serde(rename = "Desc")]
    Desc,
    #[serde(rename = "Ascending")]
    Ascending,
    #[serde(rename = "Descending")]
    Descending,
}

impl ToString for SortDirection {
    fn to_string(&self) -> String {
        match self {
            Self::Asc => String::from("Asc"),
            Self::Desc => String::from("Desc"),
            Self::Ascending => String::from("Ascending"),
            Self::Descending => String::from("Descending"),
        }
    }
}

impl Default for SortDirection {
    fn default() -> SortDirection {
        Self::Asc
    }
}