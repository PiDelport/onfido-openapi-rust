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
pub struct RightToWorkBreakdown {
    #[serde(rename = "right_to_work", skip_serializing_if = "Option::is_none")]
    pub right_to_work: Option<Box<crate::models::RightToWorkBreakdownRightToWork>>,
    #[serde(rename = "data_comparison", skip_serializing_if = "Option::is_none")]
    pub data_comparison: Option<Box<crate::models::DocumentBreakdownDataComparison>>,
    #[serde(rename = "data_validation", skip_serializing_if = "Option::is_none")]
    pub data_validation: Option<Box<crate::models::RightToWorkBreakdownDataValidation>>,
    #[serde(rename = "image_integrity", skip_serializing_if = "Option::is_none")]
    pub image_integrity: Option<Box<crate::models::RightToWorkBreakdownImageIntegrity>>,
    #[serde(
        rename = "visual_authenticity",
        skip_serializing_if = "Option::is_none"
    )]
    pub visual_authenticity: Option<Box<crate::models::RightToWorkBreakdownVisualAuthenticity>>,
    #[serde(rename = "data_consistency", skip_serializing_if = "Option::is_none")]
    pub data_consistency: Option<Box<crate::models::RightToWorkBreakdownDataConsistency>>,
    #[serde(rename = "police_record", skip_serializing_if = "Option::is_none")]
    pub police_record: Option<Box<crate::models::DocumentBreakdownPoliceRecord>>,
    #[serde(
        rename = "compromised_document",
        skip_serializing_if = "Option::is_none"
    )]
    pub compromised_document: Option<Box<crate::models::DocumentBreakdownCompromisedDocument>>,
    #[serde(rename = "age_validation", skip_serializing_if = "Option::is_none")]
    pub age_validation: Option<Box<crate::models::RightToWorkBreakdownAgeValidation>>,
    #[serde(
        rename = "share_code_validation",
        skip_serializing_if = "Option::is_none"
    )]
    pub share_code_validation: Option<Box<crate::models::RightToWorkBreakdownShareCodeValidation>>,
}

impl RightToWorkBreakdown {
    pub fn new() -> RightToWorkBreakdown {
        RightToWorkBreakdown {
            right_to_work: None,
            data_comparison: None,
            data_validation: None,
            image_integrity: None,
            visual_authenticity: None,
            data_consistency: None,
            police_record: None,
            compromised_document: None,
            age_validation: None,
            share_code_validation: None,
        }
    }
}
