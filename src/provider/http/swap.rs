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

    pub async fn post_raydium_route_swap(
        &self,
        request: &api::PostRaydiumRouteSwapRequest,
    ) -> Result<api::PostRaydiumRouteSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/raydium/route-swap", self.base_url))
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

    pub async fn post_raydium_clmm_route_swap(
        &self,
        request: &api::PostRaydiumRouteSwapRequest,
    ) -> Result<api::PostRaydiumRouteSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/raydium/clmm-route-swap", self.base_url))
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

    pub async fn post_jupiter_route_swap(
        &self,
        request: &api::PostJupiterRouteSwapRequest,
    ) -> Result<api::PostJupiterRouteSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/jupiter/route-swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post_trade_swap(
        &self,
        request: &api::TradeSwapRequest,
    ) -> Result<api::TradeSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/trade/swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post_route_trade_swap(
        &self,
        request: &api::RouteTradeSwapRequest,
    ) -> Result<api::TradeSwapResponse> {
        let response = self
            .client
            .post(format!("{}/api/v2/trade/route-swap", self.base_url))
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }
}
