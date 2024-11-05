use anyhow::Result;
use serde_json::json;
use solana_trader_proto::api;

use super::WebSocketClient;

impl WebSocketClient {
    pub async fn post_raydium_swap(
        &self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostRaydiumSwap", params).await
    }

    pub async fn post_raydium_cpmm_swap(
        &self,
        request: &api::PostRaydiumCpmmSwapRequest,
    ) -> Result<api::PostRaydiumCpmmSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "poolAddress": request.pool_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostRaydiumCPMMSwap", params).await
    }

    pub async fn post_raydium_clmm_swap(
        &self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let params = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostRaydiumCLMMSwap", params).await
    }

    // NOTE: Fast mode is not used as of 11/1, breaks the endpoint.
    pub async fn post_jupiter_swap(
        &self,
        request: &api::PostJupiterSwapRequest,
    ) -> Result<api::PostJupiterSwapResponse> {
        let modified_request = json!({
            "ownerAddress": request.owner_address,
            "inToken": request.in_token,
            "outToken": request.out_token,
            "inAmount": request.in_amount,
            "slippage": request.slippage,
            "computeLimit": request.compute_limit,
            "computePrice": request.compute_price,
            "tip": request.tip,
        });

        self.conn.request("PostJupiterSwap", modified_request).await
    }
}
