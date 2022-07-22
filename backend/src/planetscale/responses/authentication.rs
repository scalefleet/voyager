use std::time::Duration;

#[derive(serde::Deserialize)]
pub struct DeviceVerification {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_complete_uri: String,
    #[serde(with = "humantime_serde")]
    pub expires_in: Duration,
    #[serde(with = "humantime_serde", rename = "interval")]
    pub polling_interval: Duration,
}

#[derive(serde::Deserialize)]
pub struct AuthenticationError {
    pub error_code: String,
    pub error_description: String,
}

#[derive(serde::Deserialize)]
pub struct OauthTokens {
    access_token: String,
    referesh_token: String,
    id_token: String,
    #[serde(with = "humantime_serde")]
    expires_in: Duration,
}
