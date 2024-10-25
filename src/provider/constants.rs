use crate::provider::utils::http_endpoint;

pub const MAINNET_NY: &str = "ny.solana.dex.blxrbdn.com";
pub const MAINNET_PUMP_NY: &str = "pump-ny.solana.dex.blxrbdn.com";
pub const MAINNET_UK: &str = "uk.solana.dex.blxrbdn.com";
pub const TESTNET: &str = "solana.dex.bxrtest.com";
pub const DEVNET: &str = "solana-trader-api-nlb-6b0f765f2fc759e1.elb.us-east-1.amazonaws.com";

lazy_static::lazy_static! {
    pub static ref MAINNET_NY_HTTP: String = http_endpoint(MAINNET_NY, true);
    pub static ref MAINNET_PUMP_NY_HTTP: String = http_endpoint(MAINNET_PUMP_NY, true);
    pub static ref MAINNET_UK_HTTP: String = http_endpoint(MAINNET_UK, true);
    pub static ref TESTNET_HTTP: String = http_endpoint(TESTNET, true);
    pub static ref DEVNET_HTTP: String = http_endpoint(DEVNET, false);
    pub static ref LOCAL_HTTP: String = "http://localhost:9000".to_string();
}
