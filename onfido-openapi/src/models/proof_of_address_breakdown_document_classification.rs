/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ProofOfAddressBreakdownDocumentClassification : Asserts whether the document is of a valid type as PoA.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProofOfAddressBreakdownDocumentClassification {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "breakdown", skip_serializing_if = "Option::is_none")]
    pub breakdown:
        Option<Box<crate::models::ProofOfAddressBreakdownDocumentClassificationBreakdown>>,
}

impl ProofOfAddressBreakdownDocumentClassification {
    /// Asserts whether the document is of a valid type as PoA.
    pub fn new() -> ProofOfAddressBreakdownDocumentClassification {
        ProofOfAddressBreakdownDocumentClassification {
            result: None,
            breakdown: None,
        }
    }
}
