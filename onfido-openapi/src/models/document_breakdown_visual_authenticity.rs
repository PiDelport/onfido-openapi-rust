/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentBreakdownVisualAuthenticity : Asserts whether visual, non-textual, elements are correct given the type of document.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentBreakdownVisualAuthenticity {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "breakdown", skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Box<crate::models::DocumentBreakdownVisualAuthenticityBreakdown>>,
}

impl DocumentBreakdownVisualAuthenticity {
    /// Asserts whether visual, non-textual, elements are correct given the type of document.
    pub fn new() -> DocumentBreakdownVisualAuthenticity {
        DocumentBreakdownVisualAuthenticity {
            result: None,
            breakdown: None,
        }
    }
}
