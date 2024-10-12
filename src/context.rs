use std::time::Duration;

use anyhow::{bail, ensure};
use const_format::concatcp;
use log::{error, info, warn};
use reqwest::{blocking::Client, StatusCode};

use crate::request::{EditDnsRequest, PingRequest, PingResponse};

const PORKBUN_API_URL_BASE: &str = "https://api.porkbun.com/api/json/v3";

pub struct Context {
    pub api_key: String,
    pub secret_key: String,
    pub domain: String,
    pub wait_interval: Duration,
    pub client: Client,
}

impl Context {
    pub fn new() -> Context {
        let api_key = std::env::var("DDNS_API_KEY").expect("DDNS_API_KEY env variable is not set");
        let secret_key =
            std::env::var("DDNS_SECRET_KEY").expect("DDNS_SECRET_KEY env variable is not set");
        let domain = std::env::var("DDNS_DOMAIN").expect("DDNS_DOMAIN env variable is not set");

        const USER_AGENT: &str = concat!("Porkbun-DDNS", env!("CARGO_PKG_VERSION"));
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent(USER_AGENT)
            .https_only(true)
            .connection_verbose(true)
            .build()
            .expect("Could not build http client");

        let minutes_interval = match std::env::var("DDNS_MINUTES_INTERVAL") {
            Ok(i) => i.parse().unwrap_or_else(|_| {
                warn!(
                    "Invalid DDNS_MINUTES_INTERVAL env variable : {}, defaulting to 30 minutes",
                    i
                );
                30
            }),
            Err(_) => {
                warn!("DDNS_INTERVAL env variable is not set, defaulting to 30 minutes");
                30
            }
        };

        info!("Using wait interval of {minutes_interval} minute(s)");

        let wait_interval = Duration::from_secs(minutes_interval * 60);

        Context {
            api_key,
            secret_key,
            domain,
            wait_interval,
            client,
        }
    }

    pub fn get_ip(&self) -> anyhow::Result<String> {
        const PING_ENDPOINT: &str = "/ping/";
        let request_body = serde_json::to_string(&PingRequest {
            api_key: self.api_key.clone(),
            secret_key: self.secret_key.clone(),
        })?;
        let response = self
            .client
            .post(concatcp!(PORKBUN_API_URL_BASE, PING_ENDPOINT))
            .body(request_body)
            .send()?
            .text()
            .map(|t| serde_json::from_str::<PingResponse>(&t))??;

        ensure!(response.status == "SUCCESS");
        Ok(response.your_ip)
    }

    pub fn post_ip(&self, new_ip: &str) -> anyhow::Result<()> {
        let url = format!(
            "{PORKBUN_API_URL_BASE}/dns/editByNameType/{}/A/",
            self.domain,
        );

        let request_body = serde_json::to_string(&EditDnsRequest {
            api_key: self.api_key.clone(),
            secret_key: self.secret_key.clone(),
            ttl: 600,
            content: new_ip.to_string(),
        })?;

        let response = self.client.post(url).body(request_body).send()?;
        let response_status = response.status();
        let response_text = response.text()?;

        match response_status {
            StatusCode::OK => {}
            StatusCode::BAD_REQUEST => {
                bail!("400 bad request, could be the same ip is already registered if message is \"Edit error: We were unable to edit the DNS record.\" : {response_text}")
            }
            _ => {
                bail!("Ip posting failed : {response_text}");
            }
        }

        Ok(())
    }
}
