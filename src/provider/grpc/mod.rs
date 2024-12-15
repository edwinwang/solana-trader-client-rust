pub mod general;
pub mod quote;
pub mod stream;
pub mod swap;

use anyhow::Result;
use rustls::crypto::ring::default_provider;
use rustls::crypto::CryptoProvider;
use solana_sdk::pubkey::Pubkey;
use solana_trader_proto::api;
use std::collections::HashMap;
use tonic::service::Interceptor;
use tonic::transport::ClientTlsConfig;
use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel,
};

use crate::common::signing::{sign_transaction, SubmitParams};
use crate::common::{get_base_url_from_env, grpc_endpoint, BaseConfig};
use solana_sdk::signature::Keypair;
use solana_trader_proto::api::{
    GetRecentBlockHashRequestV2, PostSubmitRequest, TransactionMessage,
};

use super::utils::IntoTransactionMessage;

#[derive(Clone)]
struct AuthInterceptor {
    headers: HashMap<&'static str, String>,
    enabled: bool,
}

impl AuthInterceptor {
    fn new(auth_header: String, enabled: bool) -> Self {
        let mut headers = HashMap::new();
        headers.insert("authorization", auth_header);
        headers.insert("x-sdk", "rust-client".to_string());
        headers.insert("x-sdk-version", env!("CARGO_PKG_VERSION").to_string());

        Self { headers, enabled }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        if self.enabled {
            for (key, value) in &self.headers {
                request.metadata_mut().insert(
                    *key,
                    MetadataValue::try_from(value)
                        .map_err(|e| tonic::Status::internal(e.to_string()))?,
                );
            }
        }
        Ok(request)
    }
}

#[derive(Debug)]
pub struct GrpcClient {
    pub client: api::api_client::ApiClient<InterceptedService<Channel, AuthInterceptor>>,
    pub keypair: Option<Keypair>,
    pub public_key: Option<Pubkey>,
}

impl GrpcClient {
    pub fn get_keypair(&self) -> Result<&Keypair> {
        Ok(self.keypair.as_ref().unwrap())
    }

    pub async fn new(endpoint: Option<String>) -> Result<Self> {
        let base = BaseConfig::try_from_env()?;
        let (default_base_url, secure) = get_base_url_from_env();
        let final_base_url = endpoint.unwrap_or(default_base_url);
        let endpoint = grpc_endpoint(&final_base_url, secure);

        if CryptoProvider::get_default().is_none() {
            default_provider()
                .install_default()
                .map_err(|e| anyhow::anyhow!("Failed to install crypto provider: {:?}", e))?;
        }

        let channel = Channel::from_shared(endpoint.clone())
            .map_err(|e| anyhow::anyhow!("Invalid URI: {}", e))?
            .tls_config(ClientTlsConfig::new().with_webpki_roots())
            .map_err(|e| anyhow::anyhow!("TLS config error: {}", e))?
            .connect()
            .await
            .map_err(|e| anyhow::anyhow!("Connection error: {}", e))?;

        let interceptor = AuthInterceptor::new(base.auth_header, true);
        let client = api::api_client::ApiClient::with_interceptor(channel, interceptor);

        Ok(Self {
            client,
            public_key: base.public_key,
            keypair: base.keypair,
        })
    }

    pub async fn sign_and_submit<T: IntoTransactionMessage + Clone>(
        &mut self,
        txs: Vec<T>,
        submit_opts: SubmitParams,
        use_bundle: bool,
    ) -> Result<Vec<String>> {
        let block_hash = self
            .client
            .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
            .await?
            .into_inner()
            .block_hash;

        let keypair = self.get_keypair()?;

        if txs.len() == 1 {
            let signed_tx = sign_transaction(&txs[0], keypair, block_hash).await?;

            let req = PostSubmitRequest {
                transaction: Some(TransactionMessage {
                    content: signed_tx.content,
                    is_cleanup: signed_tx.is_cleanup,
                }),
                skip_pre_flight: submit_opts.skip_pre_flight,
                front_running_protection: Some(submit_opts.front_running_protection),
                use_staked_rp_cs: Some(submit_opts.use_staked_rpcs),
                fast_best_effort: Some(submit_opts.fast_best_effort),
                tip: None,
                allow_back_run: submit_opts.allow_back_run,
                revenue_address: submit_opts.revenue_address,
            };

            let signature = self
                .client
                .post_submit_v2(req)
                .await?
                .into_inner()
                .signature;

            return Ok(vec![signature]);
        }

        let mut entries = Vec::with_capacity(txs.len());
        for tx in txs {
            let signed_tx = sign_transaction(&tx, keypair, block_hash.clone()).await?;

            let entry = api::PostSubmitRequestEntry {
                transaction: Some(TransactionMessage {
                    content: signed_tx.content,
                    is_cleanup: signed_tx.is_cleanup,
                }),
                skip_pre_flight: submit_opts.skip_pre_flight,
            };
            entries.push(entry);
        }

        let batch_request = api::PostSubmitBatchRequest {
            entries,
            use_bundle: Some(use_bundle),
            submit_strategy: submit_opts.submit_strategy.into(),
            front_running_protection: Some(submit_opts.front_running_protection),
        };

        let response = self
            .client
            .post_submit_batch_v2(batch_request)
            .await?
            .into_inner();

        let signatures = response
            .transactions
            .into_iter()
            .filter(|entry| entry.submitted)
            .map(|entry| entry.signature)
            .collect();

        Ok(signatures)
    }
}
