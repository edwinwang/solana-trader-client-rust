use anyhow::Result;
use solana_trader_proto::api;
use tonic::Request;

use super::GrpcClient;

impl GrpcClient {
    pub async fn get_transaction(
        &mut self,
        request: &api::GetTransactionRequest,
    ) -> Result<api::GetTransactionResponse> {
        let response = self
            .client
            .get_transaction(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetTransactionResponse error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_recent_block_hash(
        &mut self,
        request: &api::GetRecentBlockHashRequest,
    ) -> Result<api::GetRecentBlockHashResponse> {
        let response = self
            .client
            .get_recent_block_hash(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRecentBlockHash error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_recent_block_hash_v2(
        &mut self,
        request: &api::GetRecentBlockHashRequestV2,
    ) -> Result<api::GetRecentBlockHashResponseV2> {
        let response = self
            .client
            .get_recent_block_hash_v2(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRecentBlockHashV2 error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_rate_limit(
        &mut self,
        request: &api::GetRateLimitRequest,
    ) -> Result<api::GetRateLimitResponse> {
        let response = self
            .client
            .get_rate_limit(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRateLimit error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_account_balance_v2(
        &mut self,
        request: &api::GetAccountBalanceRequest,
    ) -> Result<api::GetAccountBalanceResponse> {
        let response = self
            .client
            .get_account_balance_v2(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetAccountBalanceV2 error: {}", e))?;

        Ok(response.into_inner())
    }
}
