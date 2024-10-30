use std::time::Duration;

use crate::connections::ws::WS;
use crate::provider::{
    constants::LOCAL_HTTP,
    error::{ClientError, Result},
};
use solana_sdk::signature::Keypair;
use solana_trader_proto::api;
use tokio::time::timeout;
use tokio_stream::Stream;

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
        // Convert proto message to JSON value
        let params = serde_json::to_value(request)
            .map_err(|e| ClientError::new("Failed to serialize request:", e))?;

        self.conn.request("GetRaydiumQuotes", params).await
    }

    pub async fn get_trades_stream(
        &self,
        market: String,
        limit: u32,
        project: api::Project,
    ) -> Result<impl Stream<Item = Result<api::GetTradesStreamResponse>>> {
        let request = api::GetTradesRequest {
            market,
            limit,
            project: project as i32,
        };

        self.conn.stream_proto("GetTradesStream", &request).await
    }

    pub async fn get_prices_stream(
        &self,
        projects: Vec<api::Project>,
        tokens: Vec<String>,
    ) -> Result<impl Stream<Item = Result<api::GetPricesStreamResponse>>> {
        let request = api::GetPricesStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            tokens,
        };

        self.conn.stream_proto("GetPricesStream", &request).await
    }
}
