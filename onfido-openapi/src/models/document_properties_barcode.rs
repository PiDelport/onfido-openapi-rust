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
pub struct DocumentPropertiesBarcode {
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "middle_name", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "document_type", skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "date_of_expiry", skip_serializing_if = "Option::is_none")]
    pub date_of_expiry: Option<String>,
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "issuing_date", skip_serializing_if = "Option::is_none")]
    pub issuing_date: Option<String>,
    #[serde(rename = "address_line_1", skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    #[serde(rename = "address_line_2", skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    #[serde(rename = "address_line_3", skip_serializing_if = "Option::is_none")]
    pub address_line_3: Option<String>,
    #[serde(rename = "address_line_4", skip_serializing_if = "Option::is_none")]
    pub address_line_4: Option<String>,
    #[serde(rename = "address_line_5", skip_serializing_if = "Option::is_none")]
    pub address_line_5: Option<String>,
    #[serde(rename = "issuing_state", skip_serializing_if = "Option::is_none")]
    pub issuing_state: Option<String>,
    #[serde(rename = "class", skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "issuing_country", skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,
    #[serde(rename = "document_number", skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(
        rename = "real_id_classification",
        skip_serializing_if = "Option::is_none"
    )]
    pub real_id_classification: Option<String>,
}

impl DocumentPropertiesBarcode {
    pub fn new() -> DocumentPropertiesBarcode {
        DocumentPropertiesBarcode {
            first_name: None,
            middle_name: None,
            last_name: None,
            document_type: None,
            date_of_expiry: None,
            date_of_birth: None,
            issuing_date: None,
            address_line_1: None,
            address_line_2: None,
            address_line_3: None,
            address_line_4: None,
            address_line_5: None,
            issuing_state: None,
            class: None,
            gender: None,
            issuing_country: None,
            document_number: None,
            real_id_classification: None,
        }
    }
}
