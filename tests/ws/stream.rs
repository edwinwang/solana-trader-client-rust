use anyhow::Result;
use futures_util::StreamExt;
use solana_trader_client_rust::{common::constants::WRAPPED_SOL, provider::ws::WebSocketClient};
use solana_trader_proto::api;
use test_case::test_case;

#[test_case(
    vec![api::Project::PRaydium],
    vec![WRAPPED_SOL.to_string()] ;
    "raydium SOL price stream"
)]
#[tokio::test]
#[ignore]
async fn test_price_stream_ws(projects: Vec<api::Project>, tokens: Vec<String>) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_prices_stream(projects, tokens).await?;

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

    println!("Response received: {:#?}", response);
    assert!(
        response.price.unwrap().buy > 0.0,
        "Price should be positive"
    );

    ws.close().await?;
    Ok(())
}

#[test_case(1 ; "single block")]
#[tokio::test]
#[ignore]
async fn test_block_stream_ws(expected_blocks: usize) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_block_stream().await?;

    for block_num in 1..=expected_blocks {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;
        println!("Block {} received: {:#?}", block_num, response);
    }

    ws.close().await?;
    Ok(())
}

#[test_case(
    vec!["SOL/USDC".to_string(), "SOL-USDT".to_string()], 
    false ;
    "valid markets"
)]
#[tokio::test]
#[ignore]
async fn test_orderbook_stream_ws(markets: Vec<String>, expect_error: bool) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws
        .get_orderbook_stream(markets, 3, api::Project::POpenbook)
        .await?;

    match stream.next().await {
        Some(Ok(response)) => {
            println!("Response received: {:#?}", response);
            ws.close().await?;
            Ok(())
        }
        Some(Err(e)) if expect_error => {
            println!("Expected error received: {}", e);
            ws.close().await?;
            Ok(())
        }
        Some(Err(e)) => Err(anyhow::anyhow!("Stream error: {}", e)),
        None => Err(anyhow::anyhow!("Stream ended without data")),
    }
}

#[test_case(
    vec!["SOL/USDC".to_string(), "SOL-USDT".to_string()] ;
    "valid markets"
)]
#[tokio::test]
#[ignore]
async fn test_market_depth_stream_ws(valid_market: Vec<String>) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;

    let mut stream = ws
        .get_market_depths_stream(valid_market, 3, api::Project::POpenbook)
        .await?;

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;
    println!("Response received: {:#?}", response);

    ws.close().await?;
    Ok(())
}

#[test_case(
    vec![
        "BONK/SOL".to_string(),
        "wSOL/RAY".to_string(),
        "BONK/RAY".to_string(),
        "RAY/USDC".to_string(),
        "SOL/USDC".to_string(),
        "USDT/USDC".to_string()
    ],
    false ;
    "valid markets"
)]
#[tokio::test]
#[ignore]
async fn test_ticker_stream_ws(markets: Vec<String>, expect_error: bool) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws
        .get_ticker_stream(markets, api::Project::POpenbook)
        .await?;

    match stream.next().await {
        Some(Ok(response)) if !expect_error => {
            println!("Response received: {:#?}", response);
            ws.close().await?;
            Ok(())
        }
        Some(Err(e)) if expect_error => {
            println!("Expected error received: {}", e);
            ws.close().await?;
            Ok(())
        }
        _ => Err(anyhow::anyhow!("Unexpected stream result")),
    }
}

#[test_case(
    "SOL/USDC".to_string(),
    3,
    false ;
    "valid market"
)]
#[tokio::test]
#[ignore]
async fn test_trades_stream_ws(market: String, limit: u32, expect_error: bool) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws
        .get_trades_stream(market, limit, api::Project::POpenbook)
        .await?;

    match stream.next().await {
        Some(Ok(response)) if !expect_error => {
            println!("Response received: {:#?}", response);
            ws.close().await?;
            Ok(())
        }
        Some(Err(e)) if expect_error => {
            println!("Expected error received: {}", e);
            ws.close().await?;
            Ok(())
        }
        _ => Err(anyhow::anyhow!("Unexpected stream result")),
    }
}

#[test_case(
    vec![api::Project::PRaydium],
    vec!["58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2".to_string()],
    true ;
    "SOL-USDC Raydium pool"
)]
#[tokio::test]
#[ignore]
async fn test_swaps_stream_ws(
    projects: Vec<api::Project>,
    pools: Vec<String>,
    include_failed: bool,
) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_swaps_stream(projects, pools, include_failed).await?;

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;
    println!("Response received: {:#?}", response);

    ws.close().await?;
    Ok(())
}

// TODO: currently blocked until https://bloxroute.atlassian.net/browse/TRAD-1185
#[test_case(false ; "without cpmm")]
#[tokio::test]
#[ignore]
async fn test_new_raydium_pools_stream_ws(include_cpmm: bool) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_new_raydium_pools_stream(include_cpmm).await?;

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;
    println!("Response received: {:#?}", response);

    ws.close().await?;
    Ok(())
}

#[test_case(3 ; "without cpmm, three responses")]
#[tokio::test]
#[ignore]
async fn test_new_raydium_pools_by_transaction_stream_ws(expected_responses: usize) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_new_raydium_pools_by_transaction_stream().await?;

    for i in 1..=expected_responses {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;
        println!("Response {} received: {:#?}", i, response);
    }

    ws.close().await?;
    Ok(())
}

#[test_case(1 ; "single block hash")]
#[tokio::test]
#[ignore]
async fn test_recent_block_hash_stream_ws(expected_hashes: usize) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_recent_block_hash_stream().await?;

    for hash_num in 1..=expected_hashes {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

        println!("Block hash {} received: {:#?}", hash_num, response);
        assert!(
            !response.block_hash.is_empty(),
            "Block hash should not be empty"
        );
    }

    ws.close().await?;
    Ok(())
}

#[test_case(
    vec![api::Project::PRaydium],
    vec![
        "HZ1znC9XBasm9AMDhGocd9EHSyH8Pyj1EUdiPb4WnZjo".to_string(),
        "D8wAxwpH2aKaEGBKfeGdnQbCc2s54NrRvTDXCK98VAeT".to_string(),
        "DdpuaJgjB2RptGMnfnCZVmC4vkKsMV6ytRa2gggQtCWt".to_string(),
        "AVs9TA4nWDzfPJE9gGVNJMVhcQy3V9PGazuz33BfG2RA".to_string(),
        "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2".to_string(),
        "7XawhbbxtsRcQA8KTkHT9f9nc6d69UwqCDh6U5EEbEmX".to_string(),
    ];
    "Raydium pool reserves stream"
)]
#[tokio::test]
#[ignore]
async fn test_pool_reserves_stream_ws(
    projects: Vec<api::Project>,
    pools: Vec<String>,
) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_pool_reserves_stream(projects, pools).await?;

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

    println!("Response received: {:#?}", response);

    ws.close().await?;
    Ok(())
}

#[test_case(
    api::Project::PRaydium,
    None ;
    "Raydium priority fee stream"
)]
#[tokio::test]
#[ignore]
async fn test_priority_fee_stream_ws(project: api::Project, percentile: Option<f64>) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_priority_fee_stream(project, percentile).await?;

    let response = stream
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

    println!("Response received: {:#?}", response);

    ws.close().await?;
    Ok(())
}

#[test_case(1 ; "single bundle tip")]
#[tokio::test]
#[ignore]
async fn test_bundle_tip_stream_ws(expected_responses: usize) -> Result<()> {
    let ws = WebSocketClient::new(None).await?;
    let mut stream = ws.get_bundle_tip_stream().await?;

    for response_num in 1..=expected_responses {
        let response = stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream ended without data"))??;

        println!("Bundle tip {} received: {:#?}", response_num, response);
    }

    ws.close().await?;
    Ok(())
}
