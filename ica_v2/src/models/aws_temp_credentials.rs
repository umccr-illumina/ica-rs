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
pub struct AwsTempCredentials {
    /// The S3 access key.
    #[serde(rename = "accessKey")]
    pub access_key: String,
    /// The S3 secret key.
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    /// The S3 session token.
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    /// The S3 region.
    #[serde(rename = "region")]
    pub region: String,
    /// The S3 bucket name.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// The S3 object prefix these temporary credentials will give access to.
    #[serde(rename = "objectPrefix")]
    pub object_prefix: String,
    /// Used to specify the type of server-side encryption (SSE) to be used on the object provider. This value is used to determine the Amazon S3 header \"x-amz-server-side-encryption\" value. For example, specify \"AES256\" for SSE-S3, or \"AWS:KMS\" for SSE-KMS. By default if none is specified, \"AES256\" will be used.
    #[serde(rename = "serverSideEncryptionAlgorithm", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_algorithm: Option<Option<String>>,
    /// Used to specify the server-side encryption key that might be associated with the specified server-side encryption algorithm. This value can be the AWS KMS arn key, to be used for the Amazon S3 header \"x-amz-server-side-encryption-aws-kms-key-id\" value. Value will be ignored if encryption is \"AES256\"
    #[serde(rename = "serverSideEncryptionKey", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_key: Option<Option<String>>,
}

impl AwsTempCredentials {
    pub fn new(access_key: String, secret_key: String, session_token: String, region: String, bucket: String, object_prefix: String) -> AwsTempCredentials {
        AwsTempCredentials {
            access_key,
            secret_key,
            session_token,
            region,
            bucket,
            object_prefix,
            server_side_encryption_algorithm: None,
            server_side_encryption_key: None,
        }
    }
}

