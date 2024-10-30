use futures_util::StreamExt;
use solana_trader_client_rust::provider::grpc::GrpcClient;
use solana_trader_proto::api;
use tokio;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Print environment variables (sanitized) to verify they're set
//     println!("Auth header set: {}", std::env::var("AUTH_HEADER").is_ok());
//     println!("Private key set: {}", std::env::var("PRIVATE_KEY").is_ok());

//     // Use MAINNET_NY_GRPC endpoint
//     let endpoint = "https://ny.solana.dex.blxrbdn.com:443".to_string();
//     println!("Connecting to: {}", endpoint);

//     let mut client: GrpcClient = GrpcClient::new(endpoint).await?;

//     let request = api::GetRaydiumQuotesRequest {
//         in_token: "SOL".to_string(),
//         out_token: "USDC".to_string(),
//         in_amount: 1.0,
//         slippage: 0.1,
//     };

//     let response = client.get_raydium_quotes(&request).await?;
//     println!("Got response: {:?}", response);

//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "https://ny.solana.dex.blxrbdn.com".to_string();
    let mut client = GrpcClient::new(endpoint).await?;

    let mut stream = client
        .get_prices_stream(
            vec![api::Project::PRaydium],
            vec!["So11111111111111111111111111111111111111112".to_string()],
        )
        .await?;

    match stream.next().await {
        Some(Ok(response)) => {
            println!("Got response: {:?}", response);
        }
        Some(Err(e)) => {
            println!("Error: {}", e);
        }
        None => {}
    }

    Ok(())
}
