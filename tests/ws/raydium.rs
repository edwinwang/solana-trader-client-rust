use anyhow::Result;
use solana_trader_client_rust::{
    common::{USDC, WRAPPED_SOL},
    provider::ws::WebSocketClient,
};
use solana_trader_proto::api;
use std::time::Duration;
use test_case::test_case;
use tokio::time::timeout;

#[test_case(
        WRAPPED_SOL,
        USDC,
        0.01,
        5.0 ;
        "SOL to USDC quote"
    )]
#[tokio::test]
#[ignore]
async fn test_raydium_quotes_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetRaydiumQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = timeout(Duration::from_secs(10), client.get_raydium_quotes(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!("Raydium Quote: {:?}", response);
    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    client.close().await?;
    Ok(())
}
