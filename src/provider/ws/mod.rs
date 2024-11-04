pub mod quotes;
pub mod streams;

use anyhow::{anyhow, Result};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_trader_proto::api::TransactionMessage;
use tokio::time::timeout;

use crate::common::signing::{get_keypair, sign_transaction, SubmitParams};
use crate::common::{constants::DEFAULT_TIMEOUT, get_base_url_from_env, ws_endpoint, BaseConfig};
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

        let conn = timeout(DEFAULT_TIMEOUT, WS::new(Some(endpoint)))
            .await
            .map_err(|e| anyhow::anyhow!("Connection timeout: {}", e))??;

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
        fast_best_effort: bool,
    ) -> Result<String> {
        let keypair = get_keypair(&self.keypair)?;

        let block_hash: String = self.conn.request("getRecentBlockhash", json!([0])).await?;

        let signed_tx = sign_transaction(&tx, keypair, block_hash).await?;
        let params = SubmitParams {
            skip_pre_flight,
            front_running_protection,
            use_staked_rpcs,
            fast_best_effort,
        };

        let response: serde_json::Value = self
            .conn
            .request(
                "submitTransaction",
                json!([{
                    "transaction": signed_tx,
                    "skipPreFlight": params.skip_pre_flight,
                    "frontRunningProtection": params.front_running_protection,
                    "useStakedRPCs": params.use_staked_rpcs,
                    "fastBestEffort": params.fast_best_effort
                }]),
            )
            .await?;

        response
            .get("signature")
            .and_then(|s| s.as_str())
            .map(String::from)
            .ok_or_else(|| anyhow!("Missing signature in response"))
    }
}
