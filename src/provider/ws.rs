use std::time::Duration;

use crate::connections::ws::WS;
use crate::provider::{
    constants::LOCAL_HTTP,
    error::{ClientError, Result},
};
use solana_sdk::signature::Keypair;
use solana_trader_proto::api;
use tokio::time::timeout;

const INITIAL_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocketClient {
    conn: WS,
    private_key: Option<Keypair>,
}

pub struct WebSocketConfig {
    pub endpoint: String,
    pub private_key: Option<Keypair>,
    pub auth_header: String,
    pub use_tls: bool,
    pub disable_auth: bool,
}

impl WebSocketConfig {
    pub fn try_from_env() -> Result<Self> {
        let private_key = std::env::var("PRIVATE_KEY")
            .ok()
            .map(|pk| Keypair::from_base58_string(&pk));

        let auth_header = std::env::var("AUTH_HEADER").map_err(|_| {
            ClientError::from(String::from("AUTH_HEADER environment variable not set"))
        })?;

        Ok(Self {
            endpoint: LOCAL_HTTP.to_string(),
            private_key,
            auth_header,
            use_tls: true,
            disable_auth: false,
        })
    }
}

impl WebSocketClient {
    pub async fn new(endpoint: String) -> Result<Self> {
        let mut config = WebSocketConfig::try_from_env()?;
        config.endpoint = endpoint;
        config.use_tls = true;
        Self::with_config(config).await
    }

    pub async fn with_config(config: WebSocketConfig) -> Result<Self> {
        Self::attempt_connection(config)
            .await
            .map_err(|_| ClientError::new("Connection failed", "Max retries exceeded"))
    }

    pub async fn close(self) -> Result<()> {
        self.conn.close().await
    }

    async fn attempt_connection(config: WebSocketConfig) -> Result<Self> {
        if config.auth_header.is_empty() {
            return Err(ClientError::new(
                "Configuration error",
                "AUTH_HEADER is empty",
            ));
        }

        let conn = timeout(
            INITIAL_TIMEOUT,
            WS::new(config.endpoint.clone(), config.auth_header.clone()),
        )
        .await
        .map_err(|e| ClientError::new("Connection timeout", e))??;

        Ok(Self {
            conn,
            private_key: config.private_key,
        })
    }

    pub async fn get_raydium_quotes(
        &self,
        request: &api::GetRaydiumQuotesRequest,
    ) -> Result<api::GetRaydiumQuotesResponse> {
        // Add timeout to the request
        let response = self
            .conn
            .request("GetRaydiumQuotes", request)
            .await
            .map_err(|e| ClientError::new("Request timeout:", e))?;

        Ok(response)
    }
}
