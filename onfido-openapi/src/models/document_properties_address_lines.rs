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
pub struct DocumentPropertiesAddressLines {
    #[serde(rename = "street_address", skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl DocumentPropertiesAddressLines {
    pub fn new() -> DocumentPropertiesAddressLines {
        DocumentPropertiesAddressLines {
            street_address: None,
            state: None,
            postal_code: None,
            country: None,
            city: None,
            country_code: None,
        }
    }
}
