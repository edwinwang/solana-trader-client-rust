use futures_util::Stream;
use rustls::crypto::ring::default_provider;
use solana_sdk::signature::Keypair;
use solana_trader_proto::api;
use std::collections::HashMap;
use std::time::Duration;
use tonic::service::Interceptor;
use tonic::transport::ClientTlsConfig;
use tonic::Streaming;
use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel, Request,
};

use crate::provider::{
    constants::LOCAL_HTTP,
    error::{ClientError, Result},
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
    private_key: Option<Keypair>,
}

#[derive(Debug)]
pub struct GrpcClientConfig {
    pub endpoint: String,
    pub private_key: Option<Keypair>,
    pub auth_header: String,
    pub use_tls: bool,
    pub disable_auth: bool,
}

impl GrpcClientConfig {
    pub fn try_from_env() -> Result<Self> {
        let private_key = std::env::var("PRIVATE_KEY")
            .ok()
            .map(|pk| Keypair::from_base58_string(&pk));

        let auth_header = std::env::var("AUTH_HEADER").map_err(|_| {
            ClientError::from(String::from("AUTH_HEADER environment variable not set"))
        })?;

        Ok(Self {
            endpoint: LOCAL_HTTP.to_string(),
            private_key,
            auth_header,
            use_tls: true,
            disable_auth: false,
        })
    }
}

impl Default for GrpcClientConfig {
    fn default() -> Self {
        Self::try_from_env().expect("Failed to load config from environment")
    }
}

impl GrpcClient {
    pub async fn new(endpoint: String) -> Result<Self> {
        let mut config = GrpcClientConfig::try_from_env()?;
        config.endpoint = endpoint;
        config.use_tls = true;
        Self::with_config(config).await
    }

    pub async fn with_config(config: GrpcClientConfig) -> Result<Self> {
        default_provider().install_default().map_err(|e| {
            ClientError::new(
                "Failed to install crypto provider:",
                anyhow::anyhow!("{:?}", e),
            )
        })?;

        let channel = Channel::from_shared(config.endpoint.clone())
            .map_err(|e| ClientError::new("Error: invalid uri:", e))?
            .timeout(Duration::from_secs(30))
            .tls_config(ClientTlsConfig::new().with_webpki_roots())
            .map_err(|e| ClientError::new("Error: tls config:", e))?
            .connect()
            .await
            .map_err(|e| ClientError::new("Error: connect:", e))?;

        let interceptor = AuthInterceptor::new(config.auth_header.clone(), !config.disable_auth);

        let client = api::api_client::ApiClient::with_interceptor(channel, interceptor);

        Ok(Self {
            client,
            private_key: config.private_key,
        })
    }

    pub async fn get_raydium_quotes(
        &mut self,
        request: &api::GetRaydiumQuotesRequest,
    ) -> Result<api::GetRaydiumQuotesResponse> {
        let response = self
            .client
            .get_raydium_quotes(tonic::Request::new(request.clone()))
            .await
            .map_err(|e| ClientError::new("GetRaydiumQuotes error:", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_prices_stream(
        &mut self,
        projects: Vec<api::Project>,
        tokens: Vec<String>,
    ) -> Result<Streaming<api::GetPricesStreamResponse>> {
        let request = Request::new(api::GetPricesStreamRequest {
            projects: projects.iter().map(|&p| p as i32).collect(),
            tokens,
        });

        let response = self
            .client
            .get_prices_stream(request)
            .await
            .map_err(|e| ClientError::new("GetPricesStream error:", e))?;

        Ok(response.into_inner())
    }
}
