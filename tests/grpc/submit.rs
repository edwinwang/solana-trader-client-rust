use std::env;
use anyhow::Result;
use env_logger::Env;
use solana_trader_client_rust::{
    common::{USDC, WRAPPED_SOL},
    provider::grpc::GrpcClient,
};
use solana_trader_proto::api;
use test_case::test_case;
use dotenv::dotenv;
use solana_sdk::bs58::decode;
use solana_sdk::signature::{Keypair, SignerError};
use solana_trader_proto::api::TransactionMessage;

#[test_case(
    WRAPPED_SOL,
    USDC,
    0.0001,
    20.0;
    "BTC to USDC with higher slippage"
)]
#[tokio::test]
async fn test_raydium_swap_grpc(
    in_token: &str,
    out_token: &str,
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    dotenv().ok();

    let owner_address = env::var("PUBLIC_KEY").expect("PUBLIC_KEY not found in .env file");
    let pv_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env file");
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetRaydiumQuotesRequest{
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
    };


    let quote_response = client.get_raydium_quotes(&request).await?;


    let request = api::PostRaydiumSwapRequest {
        owner_address,
        in_token: in_token.to_string(),
        out_token: out_token.to_string(),
        in_amount,
        slippage,
        compute_limit: 30000,
        compute_price: 2000,
        tip: Some(2000001),
    };

    let response = client.post_raydium_swap(&request).await?;
    println!(
        "raydium Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let mut output = [0; 64];
    let _ = decode(pv_key).onto(&mut output).unwrap();

    // Create the keypair from the decoded private key bytes
    let keypair = Keypair::from_bytes(&output)?;
    let txs = response.transactions.as_slice();
    for tx in txs{
        let s = client.sign_and_submit(
            TransactionMessage{
                content: tx.clone().content,
                is_cleanup: tx.is_cleanup,
            },
            &keypair,
            true,
            false,
            false,
            false,
        ).await;
        println!("rayidum signature : {}", s);
    }

    Ok(())
}

#[test_case(
    0.0001,
    10.0;
    "Pumpfun swap"
)]
#[tokio::test]
async fn test_pumpfun_swap_grpc(
    in_amount: f64,
    slippage: f64,
) -> Result<()> {
    dotenv().ok();
    let bonding_curve_address = "Fh8fnZUVEpPStJ2hKFNNjMAyuyvoJLMouENawg4DYCBc";
    let mint_address = "2DEsbYgW94AtZxgUfYXoL8DqJAorsLrEWZdSfriipump";
    let owner_address = env::var("PUBLIC_KEY").expect("PUBLIC_KEY not found in .env file");
    let pv_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env file");
    let mut client = GrpcClient::new(None).await?;

    let request = api::GetPumpFunQuotesRequest {
        quote_type: "buy".to_string(),
        bonding_curve_address: bonding_curve_address.to_string(),
        amount: in_amount,
        mint_address: mint_address.to_string(),
    };

    let pump_quote_response = client.get_pump_fun_quotes(&request).await?;


    let request = api::PostPumpFunSwapRequest {
        user_address: owner_address,
        bonding_curve_address: bonding_curve_address.to_string(),
        token_address: "2DEsbYgW94AtZxgUfYXoL8DqJAorsLrEWZdSfriipump".to_string(),
        token_amount: pump_quote_response.out_amount,
        sol_threshold: pump_quote_response.in_amount,
        compute_limit: 300000,
        compute_price: 2000,
        slippage: slippage,
        tip: Some(2000001),
        is_buy: true,
    };
    let response = client.post_pumpfun_swap(&request).await?;
    println!(
        "pumpfun Quote: {}",
        serde_json::to_string_pretty(&response)?
    );

    let mut output = [0; 64];
    let _ = decode(pv_key).onto(&mut output).unwrap();


    let keypair = Keypair::from_bytes(&output)?;
    let tx_content = response.transaction.unwrap().content;
    let s = client.sign_and_submit(
        TransactionMessage{
            content: tx_content,
            is_cleanup: false,
        },
        &keypair,
        true,
        false,
        false,
        false,
    ).await;
    println!("signature : {}", s);
    Ok(())
}
