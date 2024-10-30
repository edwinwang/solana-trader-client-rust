#[cfg(test)]
mod tests {
    use std::{error::Error, time::Duration};

    use futures_util::StreamExt;
    use solana_trader_client_rust::provider::grpc::GrpcClient;
    use solana_trader_proto::api;
    use test_case::test_case;
    use tokio::{sync::oneshot, time::timeout};

    const ENDPOINT: &str = "https://ny.solana.dex.blxrbdn.com";
    const TIMEOUT_DURATION: Duration = Duration::from_secs(10);

    #[test_case("BTC", "USDC", 0.1, 0.2 ; "BTC to USDC with higher slippage")]
    #[tokio::test]
    #[ignore]
    async fn test_raydium_quotes(
        in_token: &str,
        out_token: &str,
        in_amount: f64,
        slippage: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = GrpcClient::new(ENDPOINT.to_string()).await?;

        let request = api::GetRaydiumQuotesRequest {
            in_token: in_token.to_string(),
            out_token: out_token.to_string(),
            in_amount,
            slippage,
        };

        let response = client.get_raydium_quotes(&request).await?;
        assert!(
            response.routes.len() > 0,
            "Expected at least one route in response"
        );

        Ok(())
    }

    #[test_case(
        vec![api::Project::PRaydium], 
        vec!["So11111111111111111111111111111111111111112".to_string()] ; 
        "raydium SOL price stream"
    )]
    #[tokio::test]
    #[ignore]
    async fn test_price_stream(
        projects: Vec<api::Project>,
        tokens: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let (tx, rx) = oneshot::channel();

        let handle = tokio::spawn(async move {
            let mut client = GrpcClient::new(ENDPOINT.to_string()).await?;

            let mut stream = client.get_prices_stream(projects, tokens).await?;

            if let Some(result) = stream.next().await {
                tx.send(result).ok();
            }

            Result::<_, Box<dyn Error + Send + Sync>>::Ok(())
        });

        // Wait for either the first message or the task to complete
        let response = rx.await?.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        assert!(response.price.unwrap().buy > 0.0);

        // Clean up
        handle.abort();

        Ok(())
    }
}
