use anyhow::Result;
use solana_trader_client_rust::{
    common::{constants::USDC, constants::WRAPPED_SOL},
    provider::ws::WebSocketClient,
};
use solana_trader_proto::api::{self, TransactionMessage};
use std::time::Duration;
use test_case::test_case;
use tokio::time::timeout;

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    1.0;
    "Jupiter SOL to USDC swap via WebSocket"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_swap_ws(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = WebSocketClient::new(None).await?;

    let request = api::PostJupiterSwapRequest {
        owner_address: client
            .public_key
            .unwrap_or_else(|| panic!("Public key is required for Jupiter swap"))
            .to_string(),
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_limit: 300000,
        compute_price: 2000,
        tip: Some(2000001),
        fast_mode: None,
    };

    let response = timeout(Duration::from_secs(10), client.post_jupiter_swap(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Timeout: {}", e))??;

    println!(
        "Jupiter Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let txs = response.transactions.as_slice();
    for tx in txs {
        let s = client
            .sign_and_submit(
                TransactionMessage {
                    content: tx.clone().content,
                    is_cleanup: tx.is_cleanup,
                },
                true,
                false,
                false,
                false,
            )
            .await;
        println!("Jupiter signature: {}", s?);
    }

    client.close().await?;
    Ok(())
}
