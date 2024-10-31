use anyhow::Result;
use solana_sdk::signature::Keypair;
use solana_trader_proto::api;
use tokio::time::timeout;
use tokio_stream::Stream;

use crate::common::{get_base_url_from_env, ws_endpoint, BaseConfig, DEFAULT_TIMEOUT};
use crate::connections::ws::WS;

pub struct WebSocketConfig {
    pub endpoint: String,
    pub private_key: Option<Keypair>,
    pub auth_header: String,
    pub use_tls: bool,
    pub disable_auth: bool,
}

pub struct WebSocketClient {
    conn: WS,
}

impl WebSocketClient {
    pub async fn new(endpoint: Option<String>) -> Result<Self> {
        let base = BaseConfig::try_from_env()?;
        let (base_url, secure) = get_base_url_from_env();
        let endpoint = endpoint.unwrap_or_else(|| ws_endpoint(&base_url, secure));

        if base.auth_header.is_empty() {
            return Err(anyhow::anyhow!("AUTH_HEADER is empty"));
        }

        let conn = timeout(DEFAULT_TIMEOUT, WS::new(Some(endpoint)))
            .await
            .map_err(|e| anyhow::anyhow!("Connection timeout: {}", e))??;

        Ok(Self { conn })
    }

    pub async fn close(self) -> Result<()> {
        self.conn.close().await
    }

    pub async fn get_raydium_quotes(
        &self,
        request: &api::GetRaydiumQuotesRequest,
    ) -> Result<api::GetRaydiumQuotesResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

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
