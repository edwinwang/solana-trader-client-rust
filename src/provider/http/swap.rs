use super::HTTPClient;
use anyhow::Result;
use solana_trader_proto::api;

impl HTTPClient {
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
