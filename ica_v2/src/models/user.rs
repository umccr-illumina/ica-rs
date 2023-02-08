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
pub struct User {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "timeModified")]
    pub time_modified: String,
    #[serde(rename = "ownerId")]
    pub owner_id: uuid::Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    #[serde(rename = "tenantName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<Option<String>>,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "firstname", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub firstname: Option<Option<String>>,
    #[serde(rename = "lastname", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lastname: Option<Option<String>>,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "tenantAdministrator")]
    pub tenant_administrator: bool,
    #[serde(rename = "jobTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<Option<String>>,
    #[serde(rename = "greeting", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub greeting: Option<Option<Greeting>>,
    #[serde(rename = "mobilePhoneNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mobile_phone_number: Option<Option<String>>,
    #[serde(rename = "phoneNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    #[serde(rename = "faxNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fax_number: Option<Option<String>>,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "twoFactorAuthentication")]
    pub two_factor_authentication: bool,
    #[serde(rename = "country", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub country: Option<Option<Box<crate::models::Country>>>,
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
}

impl User {
    pub fn new(id: uuid::Uuid, time_created: String, time_modified: String, owner_id: uuid::Uuid, tenant_id: uuid::Uuid, username: String, email: String, active: bool, tenant_administrator: bool, email_verified: bool, two_factor_authentication: bool) -> User {
        User {
            id,
            time_created,
            time_modified,
            owner_id,
            tenant_id,
            tenant_name: None,
            username,
            email,
            firstname: None,
            lastname: None,
            active,
            tenant_administrator,
            job_title: None,
            greeting: None,
            mobile_phone_number: None,
            phone_number: None,
            fax_number: None,
            email_verified,
            two_factor_authentication,
            country: None,
            address_line1: None,
            address_line2: None,
            address_line3: None,
            postal_code: None,
            city: None,
            state: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Greeting {
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MRS")]
    Mrs,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "DR")]
    Dr,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "SR")]
    Sr,
}

impl Default for Greeting {
    fn default() -> Greeting {
        Self::Mr
    }
}

