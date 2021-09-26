/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentBreakdownVisualAuthenticityBreakdownOriginalDocumentPresent : The document was not present when the photo was taken.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentBreakdownVisualAuthenticityBreakdownOriginalDocumentPresent {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<crate::models::DocumentOdpReasons>>,
}

impl DocumentBreakdownVisualAuthenticityBreakdownOriginalDocumentPresent {
    /// The document was not present when the photo was taken.
    pub fn new() -> DocumentBreakdownVisualAuthenticityBreakdownOriginalDocumentPresent {
        DocumentBreakdownVisualAuthenticityBreakdownOriginalDocumentPresent {
            result: None,
            properties: None,
        }
    }
}
