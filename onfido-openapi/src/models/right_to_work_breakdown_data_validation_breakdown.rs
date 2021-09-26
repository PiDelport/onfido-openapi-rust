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
pub struct RightToWorkBreakdownDataValidationBreakdown {
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "document_numbers", skip_serializing_if = "Option::is_none")]
    pub document_numbers:
        Option<Box<crate::models::RightToWorkBreakdownDataValidationBreakdownDocumentNumbers>>,
    #[serde(
        rename = "document_expiration",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_expiration:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "mrz", skip_serializing_if = "Option::is_none")]
    pub mrz: Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
}

impl RightToWorkBreakdownDataValidationBreakdown {
    pub fn new() -> RightToWorkBreakdownDataValidationBreakdown {
        RightToWorkBreakdownDataValidationBreakdown {
            gender: None,
            date_of_birth: None,
            document_numbers: None,
            document_expiration: None,
            expiry_date: None,
            mrz: None,
        }
    }
}
