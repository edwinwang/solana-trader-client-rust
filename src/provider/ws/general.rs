use solana_trader_proto::api;
use solana_trader_proto::api::{GetRecentBlockHashRequest, GetRecentBlockHashRequestV2, GetTransactionRequest};
use crate::provider::ws::WebSocketClient;

impl WebSocketClient {
    pub async fn get_transaction(
        &self,
        request: GetTransactionRequest,
    ) -> anyhow::Result<api::GetTransactionResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetTransaction", params).await
    }

    pub async fn get_recent_block_hash(
        &self,
        request: GetRecentBlockHashRequest,
    ) -> anyhow::Result<api::GetRecentBlockHashResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRecentBlockHash", params).await
    }

    pub async fn get_recent_block_hash_v2(
        &self,
        request: GetRecentBlockHashRequestV2,
    ) -> anyhow::Result<api::GetRecentBlockHashResponseV2> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRecentBlockHashV2", params).await
    }
}