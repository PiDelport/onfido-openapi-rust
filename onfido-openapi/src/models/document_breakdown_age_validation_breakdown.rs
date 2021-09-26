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
pub struct DocumentBreakdownAgeValidationBreakdown {
    #[serde(
        rename = "minimum_accepted_age",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_accepted_age:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
}

impl DocumentBreakdownAgeValidationBreakdown {
    pub fn new() -> DocumentBreakdownAgeValidationBreakdown {
        DocumentBreakdownAgeValidationBreakdown {
            minimum_accepted_age: None,
        }
    }
}
