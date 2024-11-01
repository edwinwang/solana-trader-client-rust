
pub mod quotes;
pub mod streams;
use tonic::Request;
use bincode::deserialize;
use bincode::serialize;

use anyhow::Result;
use rustls::crypto::ring::default_provider;
use bitnet::base58::encode;
use solana_trader_proto::api;
use std::collections::HashMap;
use std::io::Read;
use tonic::service::Interceptor;
use tonic::transport::ClientTlsConfig;
use tonic::{Response, Status, Streaming};
use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel,
};

use solana_sdk::pubkey::Pubkey;
use crate::common::{get_base_url_from_env, grpc_endpoint, BaseConfig, DEFAULT_TIMEOUT};
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use std::str::FromStr;
use serde::Serialize;
use solana_hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::Transaction;
use solana_trader_proto::api::{PostSubmitRequest, PostSubmitResponse, TransactionMessage};
// use crate::provider::{
//     constants::LOCAL_HTTP,
//     error::{ClientError, Result},
// };

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
        // let bx_memo_marker_msg: String = String::from_str("Powered by bloXroute Trader Api").unwrap();
        // let TraderAPIMemoProgram = pubkey::new_rand("HQ2UUt18uJqKaQFJhgV9zaTdQxUZjNrsKFgoEDquBkcx")
        // let base58_str = "HQ2UUt18uJqKaQFJhgV9zaTdQxUZjNrsKFgoEDquBkcx";

        // let pb = Pubkey::from_str(base58_str).unwrap();

        //base64 encoding
        let rawbytes = general_purpose::STANDARD
            .decode(tx.content).unwrap();
        // Message::try_from(bytes);
        let mut transaction : Transaction =
            deserialize(&rawbytes).unwrap();
        let block_hash = Hash::default();

        // let memo_instruction = Instruction{
        //     program_id: pb,
        //     accounts: vec![],
        //     data: bx_memo_marker_msg.as_bytes().to_vec(),
        // };
        // let com_memo_instruction =
        //     transaction.message.compile_instruction(&memo_instruction);
        // transaction.message.instructions.push(com_memo_instruction);

        transaction.try_partial_sign(&[payer], block_hash).unwrap();


        // bincode encode
        // let content = transaction.message.serialize();

        // println!("content {}", String::from_utf8(content.clone()));

        // base 64 encoding
        // let encoded_base64: String = general_purpose::STANDARD
        //     .encode(content.clone());
      // let encoded_rawbytes_base64: String = general_purpose::STANDARD
      //       .encode(&rawbytes.clone());

        let bincode =
            bincode::serialize(&transaction).expect("Serialization failed");

        let encoded_rawbytes_base64: String = general_purpose::STANDARD
            .encode(bincode.clone());


        const CUSTOM_ENGINE: engine::GeneralPurpose =
            engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

        // let encodedBase58 = bitnet::base58::encode(encoded_rawbytes_base64.as_slice()).to_string();
        // println!("encoded_base64 {}", encoded_base64.clone());
        // println!("encoded_rawbytes_base64 {}", encoded_rawbytes_base64.clone());
        // println!("encodedBase58 {}", encodedBase58.clone());
        let req = PostSubmitRequest {
            transaction: Some(TransactionMessage{
                // content: encoded_base64,
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
            Ok(v) => {
                v.into_inner().signature
            }
            Err(v) => {
                "failed to send".to_string() + "err: " + v.message()
            }
        }
    }

    pub async fn post_raydium_swap(
        &mut self,
        request: &api::PostRaydiumSwapRequest,
    ) -> Result<api::PostRaydiumSwapResponse> {
        let response = self
            .client
            .post_raydium_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostRaydiumSwap error: {}", e))?;

        Ok(response.into_inner())
    }
  pub async fn post_pumpfun_swap(
        &mut self,
        request: &api::PostPumpFunSwapRequest,
    ) -> Result<api::PostPumpFunSwapResponse> {
        let response = self
            .client
            .post_pump_fun_swap(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("PostPumpFunSwap error: {}", e))?;

        Ok(response.into_inner())
    }

}
