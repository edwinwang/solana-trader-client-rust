// In src/common/signing.rs

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use bincode::deserialize;
use serde::Serialize;
use solana_sdk::{signature::Keypair, transaction::Transaction};
use solana_trader_proto::api::TransactionMessage;

#[derive(Debug, Clone, Serialize)]
pub struct SubmitParams {
    pub skip_pre_flight: bool,
    pub front_running_protection: bool,
    pub use_staked_rpcs: bool,
    pub fast_best_effort: bool,
}

impl Default for SubmitParams {
    fn default() -> Self {
        Self {
            skip_pre_flight: false,
            front_running_protection: true,
            use_staked_rpcs: true,
            fast_best_effort: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SignedTransaction {
    pub content: String,
    pub is_cleanup: bool,
}

pub async fn sign_transaction(
    tx: &TransactionMessage,
    keypair: &Keypair,
    blockhash: String,
) -> Result<SignedTransaction> {
    let rawbytes = general_purpose::STANDARD.decode(&tx.content)?;
    let mut transaction: Transaction = deserialize(&rawbytes)?;
    let parsed_hash = blockhash.parse()?;
    transaction.try_partial_sign(&[keypair], parsed_hash)?;
    let bincode = bincode::serialize(&transaction)?;

    Ok(SignedTransaction {
        content: general_purpose::STANDARD.encode(bincode),
        is_cleanup: tx.is_cleanup,
    })
}

pub fn get_keypair(keypair: &Option<Keypair>) -> Result<&Keypair> {
    keypair
        .as_ref()
        .ok_or_else(|| anyhow!("No keypair available for signing"))
}
