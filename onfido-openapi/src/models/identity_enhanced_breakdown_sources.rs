/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityEnhancedBreakdownSources : Asserts if any sources that an applicant's details have been verified against have produced a match.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityEnhancedBreakdownSources {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "breakdown", skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Box<crate::models::IdentityEnhancedBreakdownSourcesBreakdown>>,
}

impl IdentityEnhancedBreakdownSources {
    /// Asserts if any sources that an applicant's details have been verified against have produced a match.
    pub fn new() -> IdentityEnhancedBreakdownSources {
        IdentityEnhancedBreakdownSources {
            result: None,
            breakdown: None,
        }
    }
}
