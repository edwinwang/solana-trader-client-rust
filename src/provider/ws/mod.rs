pub mod quote;
pub mod stream;
pub mod swap;

use anyhow::{anyhow, Result};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_trader_proto::api::{GetRecentBlockHashResponseV2, TransactionMessage};

use crate::common::signing::{get_keypair, sign_transaction};
use crate::common::{get_base_url_from_env, ws_endpoint, BaseConfig};
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
    keypair: Option<Keypair>,
    pub public_key: Option<Pubkey>,
}

impl WebSocketClient {
    pub async fn new(endpoint: Option<String>) -> Result<Self> {
        let base = BaseConfig::try_from_env()?;
        let (base_url, secure) = get_base_url_from_env();
        let endpoint = endpoint.unwrap_or_else(|| ws_endpoint(&base_url, secure));

        if base.auth_header.is_empty() {
            return Err(anyhow::anyhow!("AUTH_HEADER is empty"));
        }

        let conn = WS::new(Some(endpoint))
            .await
            .map_err(|e| anyhow::anyhow!("Connection timeout: {}", e))?;

        Ok(Self {
            conn,
            keypair: base.keypair,
            public_key: base.public_key,
        })
    }

    pub async fn close(self) -> Result<()> {
        self.conn.close().await
    }

    pub async fn sign_and_submit(
        &self,
        tx: TransactionMessage,
        skip_pre_flight: bool,
        front_running_protection: bool,
        use_staked_rpcs: bool,
        _fast_best_effort: bool,
    ) -> Result<String> {
        let keypair = get_keypair(&self.keypair)?;

        let hash_res: GetRecentBlockHashResponseV2 =
            self.conn.request("GetRecentBlockHashV2", json!({})).await?;

        let signed_tx = sign_transaction(&tx, keypair, hash_res.block_hash).await?;

        let request = json!({
            "transaction": {
                "content": signed_tx.content
            },
            "skipPreFlight": skip_pre_flight,
            "frontRunningProtection": front_running_protection,
            "useStakedRPCs": use_staked_rpcs
        });

        let response: serde_json::Value = self.conn.request("PostSubmitV2", request).await?;

        response
            .get("signature")
            .and_then(|s| s.as_str())
            .map(String::from)
            .ok_or_else(|| anyhow!("Missing signature in response"))
    }
}
