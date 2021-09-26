/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentBreakdownDataValidationBreakdownExpiryDate : If this is flagged, the expiration date has the incorrect format.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentBreakdownDataValidationBreakdownExpiryDate {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl DocumentBreakdownDataValidationBreakdownExpiryDate {
    /// If this is flagged, the expiration date has the incorrect format.
    pub fn new() -> DocumentBreakdownDataValidationBreakdownExpiryDate {
        DocumentBreakdownDataValidationBreakdownExpiryDate {
            result: None,
            properties: None,
        }
    }
}
