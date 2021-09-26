/*
 * Onfido API
 *
 * The Onfido API is used to submit check requests.
 *
 * The version of the OpenAPI document: 3.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentBreakdownDataComparisonBreakdown {
    #[serde(rename = "issuing_country", skip_serializing_if = "Option::is_none")]
    pub issuing_country:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "date_of_expiry", skip_serializing_if = "Option::is_none")]
    pub date_of_expiry:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "document_type", skip_serializing_if = "Option::is_none")]
    pub document_type:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "document_numbers", skip_serializing_if = "Option::is_none")]
    pub document_numbers:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth:
        Option<Box<crate::models::DocumentBreakdownDataComparisonBreakdownIssuingCountry>>,
}

impl DocumentBreakdownDataComparisonBreakdown {
    pub fn new() -> DocumentBreakdownDataComparisonBreakdown {
        DocumentBreakdownDataComparisonBreakdown {
            issuing_country: None,
            gender: None,
            date_of_expiry: None,
            last_name: None,
            document_type: None,
            document_numbers: None,
            first_name: None,
            date_of_birth: None,
        }
    }
}
