use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PingRequest {
    #[serde(rename = "apikey")]
    pub api_key: String,
    #[serde(rename = "secretapikey")]
    pub secret_key: String,
}

#[derive(Deserialize)]
pub struct PingResponse {
    pub status: String,
    #[serde(rename = "yourIp")]
    pub your_ip: String,
}

#[derive(Serialize)]
pub struct EditDnsRequest {
    #[serde(rename = "apikey")]
    pub api_key: String,
    #[serde(rename = "secretapikey")]
    pub secret_key: String,
    pub content: String,
    pub ttl: i32,
}
