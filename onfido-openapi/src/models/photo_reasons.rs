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
pub struct PhotoReasons {
    /// Flags when evidence is found that the image was manipulated by Photoshop, or other software.
    #[serde(rename = "digital_tampering", skip_serializing_if = "Option::is_none")]
    pub digital_tampering: Option<String>,
    /// Flags when evidence is found that a fake webcam was used.
    #[serde(rename = "fake_webcam", skip_serializing_if = "Option::is_none")]
    pub fake_webcam: Option<String>,
    /// Flags when evidence is found that the live photo was taken more than 24 hours before live photo upload.
    #[serde(rename = "time_of_capture", skip_serializing_if = "Option::is_none")]
    pub time_of_capture: Option<String>,
    /// Flags when evidence is found that an Android emulator was used.
    #[serde(rename = "emulator", skip_serializing_if = "Option::is_none")]
    pub emulator: Option<String>,
    /// Additional comma separated details such as the exact digital tampering software used, or the name of the fake webcam.
    #[serde(rename = "reasons", skip_serializing_if = "Option::is_none")]
    pub reasons: Option<String>,
}

impl PhotoReasons {
    pub fn new() -> PhotoReasons {
        PhotoReasons {
            digital_tampering: None,
            fake_webcam: None,
            time_of_capture: None,
            emulator: None,
            reasons: None,
        }
    }
}
