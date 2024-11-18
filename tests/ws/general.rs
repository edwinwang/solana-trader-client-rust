use anyhow::Result;

use solana_trader_client_rust::{
    common::{constants::SAMPLE_OWNER_ADDR, constants::SAMPLE_TX_SIGNATURE},
    provider::ws::WebSocketClient,
};
use solana_trader_proto::api;
use test_case::test_case;

#[test_case(SAMPLE_TX_SIGNATURE)]
#[tokio::test]
#[ignore]

async fn test_get_transaction_ws(signature: &str) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetTransactionRequest {
        signature: signature.to_string(),
    };

    let response = client.get_transaction(request).await?;
    println!(
        "Get Transaction Response: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        !response.status.is_empty(),
        "Expected a lot in the tx response"
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash() -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetRecentBlockHashRequest {};

    let response = client.get_recent_block_hash(request).await?;
    println!(
        "Get Transaction Response: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert_ne!(response.block_hash, "", "Expected a valid recent blockhash");

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_recent_block_hash_v2_ws() -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    // Test different offset values
    for offset in 0..5 {
        let request = api::GetRecentBlockHashRequestV2 { offset };

        let response = client.get_recent_block_hash_v2(&request).await?;
        println!(
            "GetRecentBlockHashV2 Response for offset {}: {}",
            offset,
            serde_json::to_string_pretty(&response)?
        );

        assert_ne!(response.block_hash, "", "Expected a recent blockhash");
    }
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_rate_limit_ws() -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetRateLimitRequest {};

    let response = client.get_rate_limit(request).await?;
    println!(
        "Get Rate Limit Response: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert_ne!(response.tier, "", "Expected a valid account tier");

    Ok(())
}

#[test_case(SAMPLE_OWNER_ADDR)]
#[tokio::test]
#[ignore]
async fn test_get_account_balance_v2_ws(owner_addr: &str) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::GetAccountBalanceRequest {
        owner_address: owner_addr.to_string(),
    };

    let response = client.get_account_balance_v2(request).await?;
    println!(
        "GetAccountBalanceV2 Response: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        response.tokens.len() > 0,
        "Expected at least one token account"
    );
    Ok(())
}
