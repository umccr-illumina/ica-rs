/*
 * ICA Rest API
 *
 * This API can be used to interact with Illumina Connected Analytics.<br> <p> Authentication to the  API can be done in multiple ways:<br> <ul><li>For the entire API, except for the POST /tokens endpoint: API-key + JWT</li> <li>Only for the POST /tokens endpoint: API-key + Basic Authentication</li></ul> </p> <p> <b>API-key</b><br> API keys are managed within the Illumina portal where you can manage your profile after you have logged on. The API-key has to be provided in the X-API-Key header parameter when executing API calls to ICA. In the background, a JWT will be requested at the IDP of Illumina to create a session. A good practice is to not use the API-key for every API call, but to first generate a JWT and to use that for authentication in subsequent calls.<br> </p> <p> <b>JWT</b><br> To avoid using an API-key for each call, we recommend to request a JWT via the POST /tokens endpoint  using this API-key. The JWT will expire after a pre-configured period specified by a tenant administrator through the IAM console in the Illumina portal. The JWT is the preferred way for authentication.<br>A not yet expired, still valid JWT could be refreshed using the POST /tokens:refresh endpoint.<br> </p> <p> <b>Basic Authentication</b><br> Basic authentication is only supported by the POST /tokens endpoint for generating a JWT. Use \"Basic base64encoded(emailaddress:password)\" in the \"Authorization\" header parameter for this authentication method. In case having access to multiple tenants using the same email-address, also provide the \"tenant\" request parameter to indicate what tenant you would like to request a JWT for. </p> 
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateConnector {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "active")]
    pub active: bool,
    /// ID of the country. If not provided then the country of the tenant will be used.
    #[serde(rename = "countryId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub country_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "addressLine1", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<Option<String>>,
    #[serde(rename = "addressLine2", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<Option<String>>,
    #[serde(rename = "addressLine3", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<Option<String>>,
    #[serde(rename = "postalCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Option<String>>,
    #[serde(rename = "city", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    #[serde(rename = "state", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// The general description of the connector instance including its purpose.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The mode the connector runs in.
    #[serde(rename = "mode")]
    pub mode: Mode,
    /// The maximum bandwidth defined in MB per second.
    #[serde(rename = "maxBandwidth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_bandwidth: Option<Option<f32>>,
    /// The maximum amount of concurrent transfers that this connector can execute.
    #[serde(rename = "maxConcurrentTransfers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_concurrent_transfers: Option<Option<i32>>,
    /// The target OS of the original connector installer.
    #[serde(rename = "os")]
    pub os: Os,
}

impl CreateConnector {
    pub fn new(code: String, active: bool, mode: Mode, os: Os) -> CreateConnector {
        CreateConnector {
            code,
            active,
            country_id: None,
            address_line1: None,
            address_line2: None,
            address_line3: None,
            postal_code: None,
            city: None,
            state: None,
            description: None,
            mode,
            max_bandwidth: None,
            max_concurrent_transfers: None,
            os,
        }
    }
}

/// The mode the connector runs in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "DOWNLOAD")]
    Download,
    #[serde(rename = "UPLOAD")]
    Upload,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "NONE")]
    None,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Download
    }
}
/// The target OS of the original connector installer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Os {
    #[serde(rename = "WINDOWS")]
    Windows,
    #[serde(rename = "LINUX")]
    Linux,
    #[serde(rename = "OSX")]
    Osx,
}

impl Default for Os {
    fn default() -> Os {
        Self::Windows
    }
}

