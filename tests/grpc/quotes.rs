use anyhow::Result;
use solana_trader_client_rust::{
    common::{USDC, WRAPPED_SOL},
    provider::grpc::GrpcClient,
};
use solana_trader_proto::api;
use test_case::test_case;

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.1,
    0.2;
    "BTC to USDC with higher slippage"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_quotes_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

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
        response.routes.len() > 0,
        "Expected at least one route in response"
    );

    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    5.0;
    "SOL to USDC CPMM quote"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_cpmm_quotes_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRaydiumCpmmQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_raydium_cpmm_quotes(&request).await?;
    println!(
        "Raydium CPMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        response.routes.len() > 0,
        "Expected at least one route in response"
    );

    Ok(())
}

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.01,
    5.0;
    "SOL to USDC CLMM quote"
)]
#[tokio::test]
#[ignore]
async fn test_raydium_clmm_quotes_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRaydiumClmmQuotesRequest {
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };

    let response = client.get_raydium_clmm_quotes(&request).await?;
    println!(
        "Raydium CLMM Quote: {}",
        serde_json::to_string_pretty(&response)?
    );
    assert!(
        response.routes.len() > 0,
        "Expected at least one route in response"
    );

    Ok(())
}
