use anyhow::Result;

use solana_trader_client_rust::{
    common::{USDC, WRAPPED_SOL},
    provider::http::HTTPClient,
};
use solana_trader_proto::api;
use test_case::test_case;

#[test_case(
    WRAPPED_SOL,
    USDC,
    1.0,
    0.1 ;
    "SOL to USDC quote via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_quotes_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::GetRaydiumQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_raydium_quotes(&request).await?;
    println!(
        "Raydium Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    1.0,
    0.1;
    "SOL to USDC Cpmm quote via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_cpmm_quotes_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::GetRaydiumCpmmQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_raydium_cpmm_quotes(&request).await?;
    println!(
        "Raydium Cpmm Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    1.0,
    0.1;
    "SOL to USDC Clmm quote via HTTP"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_clmm_quotes_http(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let client = HTTPClient::new(None)?;

    let request = api::GetRaydiumClmmQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_raydium_clmm_quotes(&request).await?;
    println!(
        "Raydium Clmm Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    assert!(
        !response.routes.is_empty(),
        "Expected at least one route in response"
    );

    Ok(())
}