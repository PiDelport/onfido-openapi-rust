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
pub struct WebhooksList {
    #[serde(rename = "webhooks", skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<crate::models::Webhook>>,
}

impl WebhooksList {
    pub fn new() -> WebhooksList {
        WebhooksList { webhooks: None }
    }
}
