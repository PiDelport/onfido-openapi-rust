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
pub struct DocumentPropertiesDocumentNumbers {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl DocumentPropertiesDocumentNumbers {
    pub fn new() -> DocumentPropertiesDocumentNumbers {
        DocumentPropertiesDocumentNumbers {
            value: None,
            type_: None,
        }
    }
}
