pub const LOCAL: &str = "localhost:9000";
pub const TESTNET: &str = "solana.dex.bxrtest.com";
pub const MAINNET_NY: &str = "ny.solana.dex.blxrbdn.com";
pub const MAINNET_UK: &str = "uk.solana.dex.blxrbdn.com";
pub const MAINNET_PUMP_NY: &str = "pump-ny.solana.dex.blxrbdn.com";

// Common tokens
pub const WRAPPED_SOL: &str = "So11111111111111111111111111111111111111112";
pub const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

// Market pairs
pub const SOL_USDC_PAIR: &str = "SOL/USDC";

pub fn http_endpoint(base_url: &str, secure: bool) -> String {
    let prefix = if secure { "https" } else { "http" };
    format!("{}://{}", prefix, base_url)
}

pub fn ws_endpoint(base_url: &str, secure: bool) -> String {
    let prefix = if secure { "wss" } else { "ws" };
    format!("{}://{}/ws", prefix, base_url)
}

pub fn grpc_endpoint(base_url: &str, secure: bool) -> String {
    let prefix = if secure { "https" } else { "http" };
    let port = if secure { "443" } else { "80" };
    format!("{}://{}:{}", prefix, base_url, port)
}

pub fn get_base_url_from_env() -> (String, bool) {
    let network = std::env::var("NETWORK").unwrap_or_else(|_| "mainnet".to_string());
    let region = std::env::var("REGION").unwrap_or_else(|_| "NY".to_string());

    match (network.as_str(), region.as_str()) {
        ("local", _) => (LOCAL.to_string(), false),
        ("testnet", _) => (TESTNET.to_string(), true),
        ("mainnet", "UK") => (MAINNET_UK.to_string(), true),
        ("mainnet", "PUMP") => (MAINNET_PUMP_NY.to_string(), true),
        _ => (MAINNET_NY.to_string(), true), // Default to NY mainnet
    }
}
