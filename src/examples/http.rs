use solana_trader_client_rust::provider::http::HTTPClient;
use solana_trader_proto::api;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "https://ny.solana.dex.blxrbdn.com/api/v2/raydium".to_string();
    let client = HTTPClient::new(endpoint)?;

    let request = api::GetRaydiumQuotesRequest {
        in_token: "SOL".to_string(),
        out_token: "USDC".to_string(),
        in_amount: 1.0,
        slippage: 0.1,
    };

    match client.get_raydium_quotes(&request).await {
        Ok(response) => {
            println!("Quote: {:?}", response);
        }
        Err(e) => {
            println!("Error making request: {:?}", e);
        }
    }

    Ok(())
}
