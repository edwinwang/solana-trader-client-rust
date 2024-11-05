use anyhow::Result;
use futures_util::StreamExt;
use solana_trader_client_rust::{common::constants::WRAPPED_SOL, provider::ws::WebSocketClient};
use solana_trader_proto::api;
use std::time::Duration;
use test_case::test_case;
use tokio::time::timeout;

#[test_case(
    "SOL/USDC",
    0,
    api::Project::POpenbook ;
    "SOL/USDC trades stream"
)]
#[tokio::test]
#[ignore]
async fn test_trades_stream_ws(market: &str, limit: i32, project: api::Project) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;

    let mut trade_stream = ws
        .get_trades_stream(market.to_string(), limit.try_into()?, project)
        .await?;

    if let Some(update) = trade_stream.next().await {
        let trade = update?;
        println!("trade stream received: {:?}", trade);
        assert!(
            trade.trades.unwrap().trades[0].order_price > 0.0,
            "Trade price should be positive"
        );
    }

    ws.close().await?;
    Ok(())
}

#[test_case(
    vec![api::Project::PRaydium],
    vec![WRAPPED_SOL.to_string()] ;
    "raydium SOL price stream"
)]
#[tokio::test]
#[ignore]
async fn test_prices_stream_ws(projects: Vec<api::Project>, tokens: Vec<String>) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;

    let mut stream = ws.get_prices_stream(projects, tokens).await?;

    let response = timeout(Duration::from_secs(30), stream.next())
        .await
        .map_err(|e| anyhow::anyhow!("Stream timeout: {}", e))?
        .ok_or_else(|| anyhow::anyhow!("Stream ended unexpectedly"))??;

    println!("response received: {:?}", response);
    assert!(
        response
            .price
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing price"))?
            .buy
            > 0.0,
        "Price should be positive"
    );

    ws.close().await?;
    Ok(())
}
