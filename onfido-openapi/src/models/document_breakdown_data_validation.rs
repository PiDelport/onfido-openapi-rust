/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentBreakdownDataValidation : Asserts whether algorithmically validatable elements are correct.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentBreakdownDataValidation {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "breakdown", skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Box<crate::models::DocumentBreakdownDataValidationBreakdown>>,
}

impl DocumentBreakdownDataValidation {
    /// Asserts whether algorithmically validatable elements are correct.
    pub fn new() -> DocumentBreakdownDataValidation {
        DocumentBreakdownDataValidation {
            result: None,
            breakdown: None,
        }
    }
}
