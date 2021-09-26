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
pub struct ProofOfAddressBreakdownImageIntegrityBreakdown {
    #[serde(rename = "image_quality", skip_serializing_if = "Option::is_none")]
    pub image_quality:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
}

impl ProofOfAddressBreakdownImageIntegrityBreakdown {
    pub fn new() -> ProofOfAddressBreakdownImageIntegrityBreakdown {
        ProofOfAddressBreakdownImageIntegrityBreakdown {
            image_quality: None,
        }
    }
}
