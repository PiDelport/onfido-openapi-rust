/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RightToWorkBreakdownShareCodeValidationBreakdownDocumentIdProperties {
    /// The document ID to retrieve the GOV.UK evidence document of the applicant's right to work.
    #[serde(rename = "document_id", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
}

impl RightToWorkBreakdownShareCodeValidationBreakdownDocumentIdProperties {
    pub fn new() -> RightToWorkBreakdownShareCodeValidationBreakdownDocumentIdProperties {
        RightToWorkBreakdownShareCodeValidationBreakdownDocumentIdProperties { document_id: None }
    }
}
