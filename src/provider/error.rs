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

impl From<&str> for ClientError {
    fn from(e: &str) -> Self {
        ClientError::Other("Error".to_string(), anyhow::anyhow!(e.to_string()))
    }
}

impl ClientError {
    pub fn new<M, E>(msg: M, error: E) -> Self
    where
        M: Into<String>,
        E: std::fmt::Display + Send + Sync + 'static,
    {
        ClientError::Other(msg.into(), anyhow::anyhow!(error.to_string()))
    }

    // Convenience method for string literal errors
    pub fn from_str<M: Into<String>>(msg: M, error: &str) -> Self {
        ClientError::Other(msg.into(), anyhow::anyhow!(error.to_string()))
    }
}

// Custom errors example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        // Using new
        let _err1 = ClientError::new("Test error", "Something went wrong");

        // Using from_str
        let _err2 = ClientError::from_str("Test error", "Something went wrong");

        // Using From trait
        let _err3: ClientError = "Direct error".into();

        // Using String
        let _err4: ClientError = "String error".to_string().into();
    }
}
