pub mod quote;
pub mod stream;
pub mod swap;
mod general;

use anyhow::{anyhow, Result};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_trader_proto::api::GetRecentBlockHashResponseV2;

use crate::common::signing::{get_keypair, sign_transaction};
use crate::common::{get_base_url_from_env, ws_endpoint, BaseConfig};
use crate::connections::ws::WS;

use super::utils::IntoTransactionMessage;

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

    pub async fn sign_and_submit<T: IntoTransactionMessage + Clone>(
        &self,
        txs: Vec<T>,
        skip_pre_flight: bool,
        front_running_protection: bool,
        use_staked_rpcs: bool,
        fast_best_effort: bool,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let keypair = get_keypair(&self.keypair)?;

        let hash_res: GetRecentBlockHashResponseV2 =
            self.conn.request("GetRecentBlockHashV2", json!({})).await?;

        // Handle single transaction case
        if txs.len() == 1 {
            let signed_tx = sign_transaction(&txs[0], keypair, hash_res.block_hash).await?;

            let request = json!({
                "transaction": {
                    "content": signed_tx.content,
                    "isCleanup": signed_tx.is_cleanup
                },
                "skipPreFlight": skip_pre_flight,
                "frontRunningProtection": front_running_protection,
                "useStakedRPCs": use_staked_rpcs,
                "fastBestEffort": fast_best_effort
            });

            let response: serde_json::Value = self.conn.request("PostSubmitV2", request).await?;

            return Ok(vec![response
                .get("signature")
                .and_then(|s| s.as_str())
                .map(String::from)
                .ok_or_else(|| anyhow!("Missing signature in response"))?]);
        }

        // Handle batch case
        let mut entries = Vec::with_capacity(txs.len());
        for tx in txs {
            let signed_tx = sign_transaction(&tx, keypair, hash_res.block_hash.clone()).await?;
            entries.push(json!({
                "transaction": {
                    "content": signed_tx.content,
                    "isCleanup": signed_tx.is_cleanup
                },
                "skipPreFlight": skip_pre_flight,
                "frontRunningProtection": front_running_protection,
                "useStakedRPCs": use_staked_rpcs,
                "fastBestEffort": fast_best_effort
            }));
        }

        let request = json!({
            "entries": entries,
            "useBundle": use_bundle,
            "submitStrategy": 0
        });

        let response: serde_json::Value = self.conn.request("PostSubmitBatchV2", request).await?;

        // Extract signatures from successful transactions
        let signatures = response["transactions"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .iter()
            .filter(|entry| entry["submitted"].as_bool().unwrap_or(false))
            .filter_map(|entry| entry["signature"].as_str().map(String::from))
            .collect();

        Ok(signatures)
    }
}
