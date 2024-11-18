use crate::provider::ws::WebSocketClient;
use solana_trader_proto::api;
use solana_trader_proto::api::{
    GetAccountBalanceRequest, GetRateLimitRequest, GetRecentBlockHashRequest,
    GetRecentBlockHashRequestV2, GetTransactionRequest,
};

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
        request: &GetRecentBlockHashRequestV2,
    ) -> anyhow::Result<api::GetRecentBlockHashResponseV2> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRecentBlockHashV2", params).await
    }
    pub async fn get_rate_limit(
        &self,
        request: GetRateLimitRequest,
    ) -> anyhow::Result<api::GetRateLimitResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetRateLimit", params).await
    }

    pub async fn get_account_balance_v2(
        &self,
        request: GetAccountBalanceRequest,
    ) -> anyhow::Result<api::GetAccountBalanceResponse> {
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("Failed to serialize request: {}", e))?;

        self.conn.request("GetAccountBalanceV2", params).await
    }
}
