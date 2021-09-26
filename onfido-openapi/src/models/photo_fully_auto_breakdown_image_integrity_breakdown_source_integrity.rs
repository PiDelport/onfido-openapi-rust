/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PhotoFullyAutoBreakdownImageIntegrityBreakdownSourceIntegrity : Asserts whether the live photo is trustworthy - i.e. not digitally tampered, from a fake webcam, or from other dubious sources.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PhotoFullyAutoBreakdownImageIntegrityBreakdownSourceIntegrity {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<crate::models::PhotoAutoReasons>>,
}

impl PhotoFullyAutoBreakdownImageIntegrityBreakdownSourceIntegrity {
    /// Asserts whether the live photo is trustworthy - i.e. not digitally tampered, from a fake webcam, or from other dubious sources.
    pub fn new() -> PhotoFullyAutoBreakdownImageIntegrityBreakdownSourceIntegrity {
        PhotoFullyAutoBreakdownImageIntegrityBreakdownSourceIntegrity {
            result: None,
            properties: None,
        }
    }
}
