/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FacialSimilarityPhotoBreakdownImageIntegrityBreakdownFaceDetected : Asserts a single face of good enough quality has been found in both the document image and the live photo.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FacialSimilarityPhotoBreakdownImageIntegrityBreakdownFaceDetected {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl FacialSimilarityPhotoBreakdownImageIntegrityBreakdownFaceDetected {
    /// Asserts a single face of good enough quality has been found in both the document image and the live photo.
    pub fn new() -> FacialSimilarityPhotoBreakdownImageIntegrityBreakdownFaceDetected {
        FacialSimilarityPhotoBreakdownImageIntegrityBreakdownFaceDetected {
            result: None,
            properties: None,
        }
    }
}
