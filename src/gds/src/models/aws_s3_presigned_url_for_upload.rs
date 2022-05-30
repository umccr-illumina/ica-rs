/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AwsS3PresignedUrlForUpload : AwsS3PresignedUrlForUpload



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AwsS3PresignedUrlForUpload {
    /// A single part presigned url for upload
    #[serde(rename = "singlePartUrl", skip_serializing_if = "Option::is_none")]
    pub single_part_url: Option<String>,
    /// A url template for multi parts presigned url for upload
    #[serde(rename = "multipartTemplate", skip_serializing_if = "Option::is_none")]
    pub multipart_template: Option<String>,
    /// Multi parts info that needs to be applied to the MultipartTemplate
    #[serde(rename = "multipartSignatures", skip_serializing_if = "Option::is_none")]
    pub multipart_signatures: Option<Vec<crate::models::PartInfo>>,
    /// Multi part upload id
    #[serde(rename = "multipartUploadId", skip_serializing_if = "Option::is_none")]
    pub multipart_upload_id: Option<String>,
    /// The server side encryption method used by S3.  This value is used to determine the Amazon S3 header \"x-amz-server-side-encryption\" value.  Possible values: 'AES256' and 'aws:kms'.
    #[serde(rename = "serverSideEncryptionAlgorithm", skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_algorithm: Option<String>,
    /// Server-side encryption key that might be associated with the specified server-side encryption algorithm  This value can be the AWS KMS arn key, to be used for the Amazon S3 header \"x-amz-server-side-encryption-aws-kms-key-id\" value  This is only used when ServerSideEncryptionAlgorithm is 'aws:kms'
    #[serde(rename = "serverSideEncryptionKey", skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_key: Option<String>,
}

impl AwsS3PresignedUrlForUpload {
    /// AwsS3PresignedUrlForUpload
    pub fn new() -> AwsS3PresignedUrlForUpload {
        AwsS3PresignedUrlForUpload {
            single_part_url: None,
            multipart_template: None,
            multipart_signatures: None,
            multipart_upload_id: None,
            server_side_encryption_algorithm: None,
            server_side_encryption_key: None,
        }
    }
}


