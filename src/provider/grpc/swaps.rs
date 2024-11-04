use anyhow::Result;
use solana_trader_proto::api;
use tonic::Request;

use super::GrpcClient;

impl GrpcClient {
    pub async fn post_raydium_swap(
        &mut self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let response = self
            .client
            .post_raydium_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostRaydiumSwap error: {}", e))?;

        Ok(response.into_inner())
    }
    pub async fn post_pumpfun_swap(
        &mut self,
        request: &api::PostPumpFunSwapRequest,
    ) -> Result<api::PostPumpFunSwapResponse> {
        let response = self
            .client
            .post_pump_fun_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostPumpFunSwap error: {}", e))?;

        Ok(response.into_inner())
    }
}
