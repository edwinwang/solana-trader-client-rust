use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use bincode::deserialize;
use serde::Serialize;
use solana_sdk::{
    message::VersionedMessage,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::{Transaction, VersionedTransaction},
};
use solana_trader_proto::api;

use crate::provider::utils::IntoTransactionMessage;

#[derive(Debug, Clone, Serialize)]
pub struct SubmitParams {
    pub skip_pre_flight: bool,
    pub front_running_protection: bool,
    pub use_staked_rpcs: bool,
    pub fast_best_effort: bool,
    pub submit_strategy: api::SubmitStrategy
}

impl Default for SubmitParams {
    fn default() -> Self {
        Self {
            skip_pre_flight: true,
            front_running_protection: false,
            use_staked_rpcs: true,
            fast_best_effort: false,
            submit_strategy: api::SubmitStrategy::PSubmitAll
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SignedTransaction {
    pub content: String,
    pub is_cleanup: bool,
}

pub async fn sign_transaction<T: IntoTransactionMessage + Clone>(
    tx: &T,
    keypair: &Keypair,
    blockhash: String,
) -> Result<SignedTransaction> {
    let tx_message = tx.clone().into_transaction_message();

    let rawbytes = general_purpose::STANDARD.decode(&tx_message.content)?;

    let result = bincode::deserialize::<VersionedTransaction>(&rawbytes);

    let signed_data = match result {
        Ok(mut versioned_tx) => {
            let parsed_hash = blockhash.parse()?;
            match &mut versioned_tx.message {
                VersionedMessage::Legacy(message) => {
                    message.recent_blockhash = parsed_hash;
                }
                VersionedMessage::V0(message) => {
                    message.recent_blockhash = parsed_hash;
                }
            }

            versioned_tx.signatures = vec![Signature::default(); 1];
            let message_data = versioned_tx.message.serialize().to_vec();
            let signature = keypair.sign_message(&message_data);
            versioned_tx.signatures[0] = signature;

            bincode::serialize(&versioned_tx)?
        }
        Err(_) => {
            let mut legacy_tx: Transaction = deserialize(&rawbytes)?;
            let parsed_hash = blockhash.parse()?;
            legacy_tx.try_partial_sign(&[keypair], parsed_hash)?;
            bincode::serialize(&legacy_tx)?
        }
    };

    Ok(SignedTransaction {
        content: general_purpose::STANDARD.encode(signed_data),
        is_cleanup: tx_message.is_cleanup,
    })
}

pub fn get_keypair(keypair: &Option<Keypair>) -> Result<&Keypair> {
    keypair
        .as_ref()
        .ok_or_else(|| anyhow!("No keypair available for signing"))
}
