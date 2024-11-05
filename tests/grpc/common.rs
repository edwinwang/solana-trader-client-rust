use anyhow::Result;
use solana_trader_client_rust::{common::constants::WRAPPED_SOL, provider::grpc::GrpcClient};
use solana_trader_proto::api;
use std::time::Duration;
use test_case::test_case;
use tokio::time::timeout;
use tokio_stream::StreamExt;

#[test_case(
    vec![api::Project::PRaydium],
    vec![WRAPPED_SOL.to_string()] ;
    "raydium SOL price stream"
)]
#[tokio::test]
#[ignore]
async fn test_price_stream_grpc(projects: Vec<api::Project>, tokens: Vec<String>) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let mut stream = client.get_prices_stream(projects, tokens).await?;

    const TIMEOUT_DURATION: Duration = Duration::from_secs(30);
    let start = std::time::Instant::now();

    loop {
        if start.elapsed() >= TIMEOUT_DURATION {
            return Err(anyhow::anyhow!("Timeout waiting for price data"));
        }

        match timeout(Duration::from_secs(5), stream.next()).await {
            Ok(Some(Ok(response))) => {
                let price = response
                    .price
                    .ok_or_else(|| anyhow::anyhow!("Missing price"))?;
                assert!(price.buy > 0.0, "Price should be positive");
                break;
            }
            Ok(Some(Err(e))) => {
                return Err(anyhow::anyhow!(
                    "Stream error after {:?}: {}",
                    start.elapsed(),
                    e
                ));
            }
            Ok(None) => {
                return Err(anyhow::anyhow!(
                    "Stream ended without data after {:?}",
                    start.elapsed()
                ));
            }
            Err(_) => {
                continue;
            }
        }
    }
    Ok(())
}
