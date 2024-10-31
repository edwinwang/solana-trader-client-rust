use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;
use solana_sdk::signature::Keypair;
use solana_trader_proto::api;

use crate::common::{get_base_url_from_env, http_endpoint, DEFAULT_TIMEOUT};
use crate::provider::error::{ClientError, Result};

#[derive(Debug)]
pub struct HTTPClientConfig {
    pub endpoint: String,
    pub private_key: Option<Keypair>,
    pub auth_header: String,
}

impl HTTPClientConfig {
    pub fn try_from_env() -> Result<Self> {
        let private_key = std::env::var("PRIVATE_KEY")
            .ok()
            .map(|pk| Keypair::from_base58_string(&pk));

        let auth_header = std::env::var("AUTH_HEADER").map_err(|_| {
            ClientError::from(String::from("AUTH_HEADER environment variable not set"))
        })?;

        let (base, secure) = get_base_url_from_env();

        Ok(Self {
            endpoint: http_endpoint(&base, secure),
            private_key,
            auth_header,
        })
    }
}

pub struct HTTPClient {
    client: Client,
    base_url: String,
}

impl HTTPClient {
    pub fn new(endpoint: Option<String>) -> Result<Self> {
        let mut config = HTTPClientConfig::try_from_env()?;
        if let Some(endpoint) = endpoint {
            config.endpoint = endpoint;
        }
        Self::with_config(config)
    }

    pub fn with_config(config: HTTPClientConfig) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&config.auth_header)
                .map_err(|e| ClientError::new("Invalid auth header", e))?,
        );
        headers.insert("x-sdk", HeaderValue::from_static("rust-client"));
        headers.insert(
            "x-sdk-version",
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .map_err(|e| ClientError::new("Failed to create HTTP client", e))?;

        Ok(Self {
            client,
            base_url: config.endpoint,
        })
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read error response"));
            return Err(ClientError::new("HTTP request failed", error_text));
        }

        response
            .json::<T>()
            .await
            .map_err(|e| ClientError::new("Failed to parse response", e))
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
            .map_err(|e| ClientError::new("HTTP GET request failed", e))?;

        self.handle_response(response).await
    }
}
