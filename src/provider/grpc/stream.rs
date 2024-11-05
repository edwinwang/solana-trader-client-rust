use anyhow::Result;
use solana_trader_proto::api;
use tonic::Request;
use tonic::Streaming;

use super::GrpcClient;

impl GrpcClient {
    pub async fn get_prices_stream(
        &mut self,
        projects: Vec<api::Project>,
        tokens: Vec<String>,
    ) -> Result<Streaming<api::GetPricesStreamResponse>> {
        let request = Request::new(api::GetPricesStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            tokens,
        });

        let response = self
            .client
            .get_prices_stream(request)
            .await
            .map_err(|e| anyhow::anyhow!("GetPricesStream error: {}", e))?;

        Ok(response.into_inner())
    }
}
