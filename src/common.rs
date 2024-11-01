use std::time::Duration;

use anyhow::anyhow;
use solana_sdk::signature::Keypair;

pub const LOCAL: &str = "localhost:9000";
pub const TESTNET: &str = "solana.dex.bxrtest.com";
pub const MAINNET_NY: &str = "ny.solana.dex.blxrbdn.com";
pub const MAINNET_UK: &str = "uk.solana.dex.blxrbdn.com";
pub const MAINNET_PUMP_NY: &str = "pump-ny.solana.dex.blxrbdn.com";

// Common tokens
pub const WRAPPED_SOL: &str = "So11111111111111111111111111111111111111112";
pub const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

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
        ("LOCAL", _) => (LOCAL.to_string(), false),
        ("TESTNET", _) => (TESTNET.to_string(), true),
        ("MAINNET", "UK") => (MAINNET_UK.to_string(), true),
        ("MAINNET", "PUMP") => (MAINNET_PUMP_NY.to_string(), true),
        _ => (MAINNET_NY.to_string(), true), // Default to NY mainnet
    }
}

pub struct BaseConfig {
    pub private_key: Option<Keypair>,
    pub auth_header: String,
}

impl BaseConfig {
    pub fn try_from_env() -> anyhow::Result<Self> {
        Ok(Self {
            private_key: std::env::var("PRIVATE_KEY")
                .ok()
                .map(|pk| Keypair::from_base58_string(&pk)),
            auth_header: std::env::var("AUTH_HEADER")
                .map_err(|_| anyhow!("AUTH_HEADER environment variable not set"))?,
        })
    }
}
