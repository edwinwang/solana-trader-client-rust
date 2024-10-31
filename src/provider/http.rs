use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;
use solana_sdk::signature::Keypair;
use solana_trader_proto::api;

use crate::common::{get_base_url_from_env, http_endpoint, BaseConfig, DEFAULT_TIMEOUT};

#[derive(Debug)]
pub struct HTTPClientConfig {
    pub endpoint: String,
    pub private_key: Option<Keypair>,
    pub auth_header: String,
}

pub struct HTTPClient {
    client: Client,
    base_url: String,
}

impl HTTPClient {
    pub fn new(endpoint: Option<String>) -> Result<Self> {
        let base = BaseConfig::try_from_env()?;
        let (base_url, secure) = get_base_url_from_env();
        let endpoint = endpoint.unwrap_or_else(|| http_endpoint(&base_url, secure));

        let headers = Self::build_headers(&base.auth_header)?;
        let client = Client::builder()
            .default_headers(headers)
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            client,
            base_url: endpoint,
        })
    }

    fn build_headers(auth_header: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(auth_header)
                .map_err(|e| anyhow!("Invalid auth header: {}", e))?,
        );
        headers.insert("x-sdk", HeaderValue::from_static("rust-client"));
        headers.insert(
            "x-sdk-version",
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        );
        Ok(headers)
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".into());
            return Err(anyhow::anyhow!("HTTP request failed: {}", error_text));
        }

        response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))
    }

    pub async fn get_raydium_quotes(
        &self,
        request: &api::GetRaydiumQuotesRequest,
    ) -> Result<api::GetRaydiumQuotesResponse> {
        let url = format!(
            "{}/api/v2/raydium/quotes?inToken={}&outToken={}&inAmount={}&slippage={}",
            self.base_url, request.in_token, request.out_token, request.in_amount, request.slippage
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP GET request failed: {}", e))?;

        self.handle_response(response).await
    }
}
