/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WatchlistEnhancedBreakdownAdverseMedia : Asserts if there are any records found of negative events reported by publicly and generally available media sources.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WatchlistEnhancedBreakdownAdverseMedia {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl WatchlistEnhancedBreakdownAdverseMedia {
    /// Asserts if there are any records found of negative events reported by publicly and generally available media sources.
    pub fn new() -> WatchlistEnhancedBreakdownAdverseMedia {
        WatchlistEnhancedBreakdownAdverseMedia { result: None }
    }
}
