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
pub struct LoadDataInBaseRequest {
    /// Enable to accept rows that are missing trailing optional columns. Missing values will be treated as nulls.
    #[serde(
        rename = "allowJaggedRows",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_jagged_rows: Option<Option<bool>>,
    /// Enable to include newlines contained in quoted data sections in the cell’s value. When disabled, newlines will signal a new row
    #[serde(
        rename = "allowQuotedNewlines",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_quoted_newlines: Option<Option<bool>>,
    /// ID of the data to load into the table
    #[serde(rename = "dataId")]
    pub data_id: String,
    /// field delimiter
    #[serde(
        rename = "delimiter",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub delimiter: Option<Option<String>>,
    /// Encoding
    #[serde(
        rename = "encoding",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub encoding: Option<Option<Encoding>>,
    /// When false (default): the data will not be loaded if it was already previously loaded to table ; when true, the data will be loaded even if already loaded in the past
    #[serde(
        rename = "forceLoad",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub force_load: Option<Option<bool>>,
    /// number of rows to skip (usually for headers)
    #[serde(
        rename = "headerRowsToSkip",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub header_rows_to_skip: Option<Option<i32>>,
    /// When enabled, rows with extra column values that do not match the schema will be ignored and will not be loaded into the table
    #[serde(
        rename = "ignoreUnknownValues",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_unknown_values: Option<bool>,
    /// Include references
    #[serde(
        rename = "includeReferences",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_references: Option<Option<bool>>,
    /// Include Data Reference
    #[serde(
        rename = "includeDataReference",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_data_reference: Option<Option<bool>>,
    /// Include Sample Reference
    #[serde(
        rename = "includeSampleReference",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_sample_reference: Option<Option<bool>>,
    /// Include Pipeline Reference
    #[serde(
        rename = "includePipelineReference",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_pipeline_reference: Option<Option<bool>>,
    /// Include Pipeline Execution Reference
    #[serde(
        rename = "includePipelineExecutionReference",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_pipeline_execution_reference: Option<Option<bool>>,
    /// Include Tenant Reference
    #[serde(
        rename = "includeTenantReference",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_tenant_reference: Option<Option<bool>>,
    /// Specifies a string that represents a null value in a CSV/TSV file.
    #[serde(
        rename = "nullMarker",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub null_marker: Option<Option<String>>,
    /// The maximum number of bad records that Base can ignore when running the job
    #[serde(
        rename = "numberOfErrorsAllowed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_errors_allowed: Option<Option<i32>>,
    /// The value that is used to quote data sections in a CSV/TSV file
    #[serde(
        rename = "quote",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote: Option<Option<String>>,
    /// specifies how to write data in the table.
    #[serde(
        rename = "writePreference",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub write_preference: Option<Option<WritePreference>>,
}

impl LoadDataInBaseRequest {
    pub fn new(data_id: String) -> LoadDataInBaseRequest {
        LoadDataInBaseRequest {
            allow_jagged_rows: None,
            allow_quoted_newlines: None,
            data_id,
            delimiter: None,
            encoding: None,
            force_load: None,
            header_rows_to_skip: None,
            ignore_unknown_values: None,
            include_references: None,
            include_data_reference: None,
            include_sample_reference: None,
            include_pipeline_reference: None,
            include_pipeline_execution_reference: None,
            include_tenant_reference: None,
            null_marker: None,
            number_of_errors_allowed: None,
            quote: None,
            write_preference: None,
        }
    }
}

/// Encoding
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Encoding {
    #[serde(rename = "UTF8")]
    Utf8,
    #[serde(rename = "ISO88591")]
    Iso88591,
}

impl Default for Encoding {
    fn default() -> Encoding {
        Self::Utf8
    }
}
/// specifies how to write data in the table.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WritePreference {
    #[serde(rename = "WRITEIFEMPTY")]
    Writeifempty,
    #[serde(rename = "APPENDTOTABLE")]
    Appendtotable,
    #[serde(rename = "OVERWRITETABLE")]
    Overwritetable,
}

impl Default for WritePreference {
    fn default() -> WritePreference {
        Self::Writeifempty
    }
}
