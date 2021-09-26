/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ProofOfAddressBreakdownDataComparison : Asserts whether the first name, last name and address provided by the applicant match those on the PoA document.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProofOfAddressBreakdownDataComparison {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "breakdown", skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Box<crate::models::ProofOfAddressBreakdownDataComparisonBreakdown>>,
}

impl ProofOfAddressBreakdownDataComparison {
    /// Asserts whether the first name, last name and address provided by the applicant match those on the PoA document.
    pub fn new() -> ProofOfAddressBreakdownDataComparison {
        ProofOfAddressBreakdownDataComparison {
            result: None,
            breakdown: None,
        }
    }
}
