use super::HTTPClient;
use anyhow::Result;
use solana_trader_proto::api;

impl HTTPClient {
    pub async fn post_raydium_swap(
        &self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/raydium/swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post_raydium_cpmm_swap(
        &self,
        request: &api::PostRaydiumCpmmSwapRequest,
    ) -> Result<api::PostRaydiumCpmmSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/raydium/cpmm-swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post_raydium_clmm_swap(
        &self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/raydium/clmm-swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post_jupiter_swap(
        &self,
        request: &api::PostJupiterSwapRequest,
    ) -> Result<api::PostJupiterSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/jupiter/swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }
}
