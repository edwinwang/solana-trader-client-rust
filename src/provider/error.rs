use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("gRPC error: {0}")]
    Grpc(#[from] Status),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{0}: {1}")]
    Other(String, #[source] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ClientError>;

impl From<String> for ClientError {
    fn from(e: String) -> Self {
        ClientError::Other("Error".to_string(), anyhow::anyhow!(e))
    }
}

impl ClientError {
    pub fn new(msg: impl Into<String>, error: impl Into<anyhow::Error>) -> Self {
        ClientError::Other(msg.into(), error.into())
    }
}
