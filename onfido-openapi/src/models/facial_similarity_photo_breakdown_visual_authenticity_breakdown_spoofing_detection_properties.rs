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
pub struct FacialSimilarityPhotoBreakdownVisualAuthenticityBreakdownSpoofingDetectionProperties {
    /// A floating point number between 0 and 1. The closer the score is to 0, the more likely it is to be a spoof.
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

impl FacialSimilarityPhotoBreakdownVisualAuthenticityBreakdownSpoofingDetectionProperties {
    pub fn new(
    ) -> FacialSimilarityPhotoBreakdownVisualAuthenticityBreakdownSpoofingDetectionProperties {
        FacialSimilarityPhotoBreakdownVisualAuthenticityBreakdownSpoofingDetectionProperties {
            score: None,
        }
    }
}
