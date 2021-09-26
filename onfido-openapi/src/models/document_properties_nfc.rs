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
pub struct DocumentPropertiesNfc {
    #[serde(rename = "document_type", skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "issuing_country", skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "document_number", skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(rename = "nationality", skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "date_of_expiry", skip_serializing_if = "Option::is_none")]
    pub date_of_expiry: Option<String>,
    #[serde(rename = "personal_number", skip_serializing_if = "Option::is_none")]
    pub personal_number: Option<String>,
    #[serde(rename = "place_of_birth", skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "issuing_date", skip_serializing_if = "Option::is_none")]
    pub issuing_date: Option<String>,
    #[serde(rename = "issuing_authority", skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<String>,
}

impl DocumentPropertiesNfc {
    pub fn new() -> DocumentPropertiesNfc {
        DocumentPropertiesNfc {
            document_type: None,
            issuing_country: None,
            full_name: None,
            document_number: None,
            nationality: None,
            date_of_birth: None,
            gender: None,
            date_of_expiry: None,
            personal_number: None,
            place_of_birth: None,
            address: None,
            issuing_date: None,
            issuing_authority: None,
        }
    }
}
