pub mod quotes;
pub mod streams;

use anyhow::Result;
use solana_sdk::signature::Keypair;
use tokio::time::timeout;

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
}
