use anyhow::Result;
use rustls::crypto::ring::default_provider;
use solana_trader_proto::api;
use std::collections::HashMap;
use tonic::service::Interceptor;
use tonic::transport::ClientTlsConfig;
use tonic::Streaming;
use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel, Request,
};

use crate::common::{get_base_url_from_env, grpc_endpoint, BaseConfig, DEFAULT_TIMEOUT};

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

    /// QUOTES
    pub async fn get_raydium_quotes(
        &mut self,
        request: &api::GetRaydiumQuotesRequest,
    ) -> Result<api::GetRaydiumQuotesResponse> {
        let response = self
            .client
            .get_raydium_quotes(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRaydiumQuotes error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_raydium_cpmm_quotes(
        &mut self,
        request: &api::GetRaydiumCpmmQuotesRequest,
    ) -> Result<api::GetRaydiumCpmmQuotesResponse> {
        let response = self
            .client
            .get_raydium_cpmm_quotes(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRaydiumCPMMQuotes error: {}", e))?;

        Ok(response.into_inner())
    }

    pub async fn get_raydium_clmm_quotes(
        &mut self,
        request: &api::GetRaydiumClmmQuotesRequest,
    ) -> Result<api::GetRaydiumClmmQuotesResponse> {
        let response = self
            .client
            .get_raydium_clmm_quotes(Request::new(request.clone()))
            .await
            .map_err(|e| anyhow::anyhow!("GetRaydiumCLMMQuotes error: {}", e))?;

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
            .map_err(|e| anyhow::anyhow!("GetPricesStream error: {}", e))?;

        Ok(response.into_inner())
    }
}
