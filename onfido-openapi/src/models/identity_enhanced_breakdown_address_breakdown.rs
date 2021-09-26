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
pub struct IdentityEnhancedBreakdownAddressBreakdown {
    #[serde(rename = "credit_agencies", skip_serializing_if = "Option::is_none")]
    pub credit_agencies:
        Option<Box<crate::models::IdentityEnhancedBreakdownAddressBreakdownCreditAgencies>>,
    #[serde(rename = "telephone_database", skip_serializing_if = "Option::is_none")]
    pub telephone_database:
        Option<Box<crate::models::IdentityEnhancedBreakdownAddressBreakdownTelephoneDatabase>>,
    #[serde(rename = "voting_register", skip_serializing_if = "Option::is_none")]
    pub voting_register:
        Option<Box<crate::models::IdentityEnhancedBreakdownAddressBreakdownVotingRegister>>,
}

impl IdentityEnhancedBreakdownAddressBreakdown {
    pub fn new() -> IdentityEnhancedBreakdownAddressBreakdown {
        IdentityEnhancedBreakdownAddressBreakdown {
            credit_agencies: None,
            telephone_database: None,
            voting_register: None,
        }
    }
}
