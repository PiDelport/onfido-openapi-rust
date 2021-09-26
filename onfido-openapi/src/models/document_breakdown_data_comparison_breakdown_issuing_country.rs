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
pub struct DocumentBreakdownDataComparisonBreakdownIssuingCountry {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl DocumentBreakdownDataComparisonBreakdownIssuingCountry {
    pub fn new() -> DocumentBreakdownDataComparisonBreakdownIssuingCountry {
        DocumentBreakdownDataComparisonBreakdownIssuingCountry {
            result: None,
            properties: None,
        }
    }
}
