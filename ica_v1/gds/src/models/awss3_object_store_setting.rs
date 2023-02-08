/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Awss3ObjectStoreSetting {
    /// The bucket name
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// Key prefix within the bucket for GDS to operate within. Volumes may only be created within this prefix and the given credentials need only authorize  access here. If not set, default is to allow operation on the full bucket. No leading slash, and must end with a trailing slash.
    #[serde(rename = "keyPrefix", skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// Used to specify the type of server-side encryption (SSE) to be used on the object provider.  This value is used to determine the Amazon S3 header \"x-amz-server-side-encryption\" value.  For example, specify \"AES256\" for SSE-S3, or \"AWS:KMS\" for SSE-KMS.  By default if none is specified, \"AES256\" will be used.
    #[serde(
        rename = "serverSideEncryptionAlgorithm",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_side_encryption_algorithm: Option<String>,
    /// Used to specify the serve-side encryption key that might be associated with the specified server-side encryption algorithm  This value can be the AWS KMS arn key, to be used for the Amazon S3 header \"x-amz-server-side-encryption-aws-kms-key-id\" value  Value will be ignored if encryption is \"AES256\"
    #[serde(
        rename = "serverSideEncryptionKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_side_encryption_key: Option<String>,
}

impl Awss3ObjectStoreSetting {
    pub fn new(bucket_name: String) -> Awss3ObjectStoreSetting {
        Awss3ObjectStoreSetting {
            bucket_name,
            key_prefix: None,
            server_side_encryption_algorithm: None,
            server_side_encryption_key: None,
        }
    }
}