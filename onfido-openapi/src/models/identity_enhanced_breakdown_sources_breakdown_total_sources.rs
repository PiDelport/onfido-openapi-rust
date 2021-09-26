/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityEnhancedBreakdownSourcesBreakdownTotalSources : The number of sources which produced a match to applicant details.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityEnhancedBreakdownSourcesBreakdownTotalSources {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<Box<crate::models::IdentityEnhancedBreakdownSourcesBreakdownTotalSourcesProperties>>,
}

impl IdentityEnhancedBreakdownSourcesBreakdownTotalSources {
    /// The number of sources which produced a match to applicant details.
    pub fn new() -> IdentityEnhancedBreakdownSourcesBreakdownTotalSources {
        IdentityEnhancedBreakdownSourcesBreakdownTotalSources {
            result: None,
            properties: None,
        }
    }
}
