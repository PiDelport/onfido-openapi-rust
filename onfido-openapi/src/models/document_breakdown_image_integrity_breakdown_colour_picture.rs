/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DocumentBreakdownImageIntegrityBreakdownColourPicture : Asserts whether the image was a colour one.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentBreakdownImageIntegrityBreakdownColourPicture {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl DocumentBreakdownImageIntegrityBreakdownColourPicture {
    /// Asserts whether the image was a colour one.
    pub fn new() -> DocumentBreakdownImageIntegrityBreakdownColourPicture {
        DocumentBreakdownImageIntegrityBreakdownColourPicture {
            result: None,
            properties: None,
        }
    }
}
