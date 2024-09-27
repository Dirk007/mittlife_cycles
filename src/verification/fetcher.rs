use anyhow::{bail, Result};
use serde::Deserialize;

const DEFAULT_FETCH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(6);
const DEFAULT_MITTWALD_BASE_URL: &str = "https://api.mittwald.de";
const PUBLIC_KEY_ROUTE: &str = "/v2/webhook-public-keys";

/// Trait for fetching public keys from a server.
#[async_trait::async_trait]
pub trait KeyFetcher {
    async fn fetch(&self, serial: &str) -> Result<PublicKeyResponse>;
}

/// Public key response from the server.
#[derive(Deserialize)]
pub struct PublicKeyResponse {
    #[serde(rename = "key")]
    pub key_base64: String,
    pub serial: String,
}

/// Implementation of KeyFetcher using reqwest.
pub struct ReqwestFetcher {
    client: reqwest::Client,
    base_url: String,
}

impl Default for ReqwestFetcher {
    fn default() -> Self {
        ReqwestFetcher {
            client: reqwest::Client::default(),
            base_url: DEFAULT_MITTWALD_BASE_URL.to_string(),
        }
    }
}

impl ReqwestFetcher {
    pub fn with_base_url(self, base_url: &str) -> Self {
        ReqwestFetcher {
            client: self.client,
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl KeyFetcher for ReqwestFetcher {
    async fn fetch(&self, serial: &str) -> Result<PublicKeyResponse> {
        let final_url = format!("{}{}/{}", self.base_url, PUBLIC_KEY_ROUTE, serial);
        log::info!("Fetching remote key {} from {}", serial, self.base_url);
        let response = self
            .client
            .get(final_url)
            .header("accept", "application/json")
            .fetch_mode_no_cors()
            .timeout(DEFAULT_FETCH_TIMEOUT)
            .send()
            .await
            .inspect_err(|err| log::info!("blablub error: {:?}", err))?;
        log::info!("Response status from fetch: {}", response.status());
        if response.status() != reqwest::StatusCode::OK {
            bail!("failed to fetch content. HTTP status {}", response.status());
        }
        let response = response.json::<PublicKeyResponse>().await?;
        Ok(response)
    }
}
