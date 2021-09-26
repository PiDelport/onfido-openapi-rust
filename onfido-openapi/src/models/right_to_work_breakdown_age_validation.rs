/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RightToWorkBreakdownAgeValidation : Asserts whether the age calculated from the document’s date of birth data point is greater than or equal to the minimum accepted age.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RightToWorkBreakdownAgeValidation {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl RightToWorkBreakdownAgeValidation {
    /// Asserts whether the age calculated from the document’s date of birth data point is greater than or equal to the minimum accepted age.
    pub fn new() -> RightToWorkBreakdownAgeValidation {
        RightToWorkBreakdownAgeValidation { result: None }
    }
}
