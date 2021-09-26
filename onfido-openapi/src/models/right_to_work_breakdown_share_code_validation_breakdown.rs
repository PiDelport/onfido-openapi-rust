/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RightToWorkBreakdownShareCodeValidationBreakdown {
    #[serde(rename = "document_id", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<Box<crate::models::RightToWorkBreakdownShareCodeValidationBreakdownDocumentId>>,
    #[serde(rename = "applicant_has_valid_share_code", skip_serializing_if = "Option::is_none")]
    pub applicant_has_valid_share_code: Option<Box<crate::models::RightToWorkBreakdownShareCodeValidationBreakdownApplicantHasValidShareCode>>,
    #[serde(rename = "name_matched", skip_serializing_if = "Option::is_none")]
    pub name_matched: Option<Box<crate::models::RightToWorkBreakdownShareCodeValidationBreakdownNameMatched>>,
}

impl RightToWorkBreakdownShareCodeValidationBreakdown {
    pub fn new() -> RightToWorkBreakdownShareCodeValidationBreakdown {
        RightToWorkBreakdownShareCodeValidationBreakdown {
            document_id: None,
            applicant_has_valid_share_code: None,
            name_matched: None,
        }
    }
}
