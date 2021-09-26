/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentBreakdownVisualAuthenticityBreakdownTemplate : The document doesn’t match the expected template for the document type and country it is from.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentBreakdownVisualAuthenticityBreakdownTemplate {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl DocumentBreakdownVisualAuthenticityBreakdownTemplate {
    /// The document doesn’t match the expected template for the document type and country it is from.
    pub fn new() -> DocumentBreakdownVisualAuthenticityBreakdownTemplate {
        DocumentBreakdownVisualAuthenticityBreakdownTemplate {
            result: None,
            properties: None,
        }
    }
}
