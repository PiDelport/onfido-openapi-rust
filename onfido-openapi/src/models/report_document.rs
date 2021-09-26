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
pub struct ReportDocument {
    /// ID of uploaded document to use.
    #[serde(rename = "id")]
    pub id: String,
}

impl ReportDocument {
    pub fn new(id: String) -> ReportDocument {
        ReportDocument { id }
    }
}
