use std::time::Duration;

use solana_trader_client_rust::provider::ws::WebSocketClient;
use solana_trader_proto::api;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "wss://ny.solana.dex.blxrbdn.com/ws".to_string();
    let client = WebSocketClient::new(endpoint).await?;

    let request = api::GetRaydiumQuotesRequest {
        in_token: "So11111111111111111111111111111111111111112".to_string(),
        out_token: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        in_amount: 0.01,
        slippage: 5.0,
    };

    match timeout(Duration::from_secs(3), client.get_raydium_quotes(&request)).await {
        Ok(result) => match result {
            Ok(response) => println!("Got response: {:?}", response),
            Err(e) => println!("Error getting quotes: {:?}", e),
        },
        Err(_) => println!("Operation timed out after 15 seconds"),
    }

    client.close().await?;

    Ok(())
}
