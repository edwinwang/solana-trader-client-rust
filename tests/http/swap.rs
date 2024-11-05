use anyhow::Result;
use solana_trader_client_rust::common::{constants::USDC, constants::WRAPPED_SOL};
use solana_trader_client_rust::provider::http::HTTPClient;
use solana_trader_proto::api;
use solana_trader_proto::api::TransactionMessage;
use test_case::test_case;

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    1.0;
    "Jupiter SOL to USDC swap via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_jupiter_swap_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

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

    let response = client.post_jupiter_swap(&request).await?;
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

    Ok(())
}
