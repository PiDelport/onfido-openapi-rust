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
pub struct ProofOfAddressBreakdown {
    #[serde(rename = "data_comparison", skip_serializing_if = "Option::is_none")]
    pub data_comparison: Option<Box<crate::models::ProofOfAddressBreakdownDataComparison>>,
    #[serde(
        rename = "document_classification",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_classification:
        Option<Box<crate::models::ProofOfAddressBreakdownDocumentClassification>>,
    #[serde(rename = "image_integrity", skip_serializing_if = "Option::is_none")]
    pub image_integrity: Option<Box<crate::models::ProofOfAddressBreakdownImageIntegrity>>,
}

impl ProofOfAddressBreakdown {
    pub fn new() -> ProofOfAddressBreakdown {
        ProofOfAddressBreakdown {
            data_comparison: None,
            document_classification: None,
            image_integrity: None,
        }
    }
}
