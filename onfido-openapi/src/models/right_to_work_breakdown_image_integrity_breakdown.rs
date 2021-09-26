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
pub struct RightToWorkBreakdownImageIntegrityBreakdown {
    #[serde(rename = "image_quality", skip_serializing_if = "Option::is_none")]
    pub image_quality:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "supported_document", skip_serializing_if = "Option::is_none")]
    pub supported_document:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "colour_picture", skip_serializing_if = "Option::is_none")]
    pub colour_picture:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(
        rename = "conclusive_document_quality",
        skip_serializing_if = "Option::is_none"
    )]
    pub conclusive_document_quality:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
}

impl RightToWorkBreakdownImageIntegrityBreakdown {
    pub fn new() -> RightToWorkBreakdownImageIntegrityBreakdown {
        RightToWorkBreakdownImageIntegrityBreakdown {
            image_quality: None,
            supported_document: None,
            colour_picture: None,
            conclusive_document_quality: None,
        }
    }
}
