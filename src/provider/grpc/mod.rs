pub mod quotes;
pub mod streams;
pub mod swaps;

use anyhow::Result;
use bincode::deserialize;
use rustls::crypto::ring::default_provider;
use solana_trader_proto::api;
use std::collections::HashMap;
use tonic::service::Interceptor;
use tonic::transport::ClientTlsConfig;
use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel,
};

use crate::common::{get_base_url_from_env, grpc_endpoint, BaseConfig, DEFAULT_TIMEOUT};
use base64::{engine::general_purpose, Engine as _};
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::Transaction;
use solana_trader_proto::api::{
    GetRecentBlockHashRequestV2, PostSubmitRequest, TransactionMessage,
};

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
    client: api::api_client::ApiClient<InterceptedService<Channel, AuthInterceptor>>,
}

impl GrpcClient {
    pub async fn new(endpoint: Option<String>) -> Result<Self> {
        let base = BaseConfig::try_from_env()?;
        let (base_url, secure) = get_base_url_from_env();
        let endpoint = endpoint.unwrap_or_else(|| grpc_endpoint(&base_url, secure));
        default_provider()
            .install_default()
            .map_err(|e| anyhow::anyhow!("Failed to install crypto provider: {:?}", e))?;

        let channel = Channel::from_shared(endpoint.clone())
            .map_err(|e| anyhow::anyhow!("Invalid URI: {}", e))?
            .timeout(DEFAULT_TIMEOUT)
            .tls_config(ClientTlsConfig::new().with_webpki_roots())
            .map_err(|e| anyhow::anyhow!("TLS config error: {}", e))?
            .connect()
            .await
            .map_err(|e| anyhow::anyhow!("Connection error: {}", e))?;

        let interceptor = AuthInterceptor::new(base.auth_header, true);
        let client = api::api_client::ApiClient::with_interceptor(channel, interceptor);

        Ok(Self { client })
    }

    pub async fn sign_and_submit(
        &mut self,
        tx: TransactionMessage,
        payer: &Keypair,
        skip_pre_flight: bool,
        front_running_protection: bool,
        use_staked_rpcs: bool,
        fast_best_effort: bool,
    ) -> String {
        let rawbytes = general_purpose::STANDARD.decode(tx.content).unwrap();

        let mut transaction: Transaction = deserialize(&rawbytes).unwrap();
        let block_hash = self
            .client
            .get_recent_block_hash_v2(GetRecentBlockHashRequestV2 { offset: 0 })
            .await
            .unwrap()
            .into_inner()
            .block_hash;

        transaction
            .try_partial_sign(&[payer], block_hash.parse().unwrap())
            .unwrap();

        let bincode = bincode::serialize(&transaction).expect("Serialization failed");

        let encoded_rawbytes_base64: String = general_purpose::STANDARD.encode(bincode.clone());

        let req = PostSubmitRequest {
            transaction: Some(TransactionMessage {
                content: encoded_rawbytes_base64,
                is_cleanup: tx.is_cleanup,
            }),
            skip_pre_flight,
            front_running_protection: Some(front_running_protection),
            tip: None,
            use_staked_rp_cs: Some(use_staked_rpcs),
            fast_best_effort: Some(fast_best_effort),
        };
        let res = self.client.post_submit(req).await;
        match res {
            Ok(v) => v.into_inner().signature,
            Err(v) => "failed to send".to_string() + "err: " + v.message(),
        }
    }
}
