/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityEnhancedBreakdownAddressBreakdownVotingRegister : Address match against voting register.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityEnhancedBreakdownAddressBreakdownVotingRegister {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl IdentityEnhancedBreakdownAddressBreakdownVotingRegister {
    /// Address match against voting register.
    pub fn new() -> IdentityEnhancedBreakdownAddressBreakdownVotingRegister {
        IdentityEnhancedBreakdownAddressBreakdownVotingRegister {
            result: None,
            properties: None,
        }
    }
}
