use futures_util::{SinkExt, StreamExt};
use rustls::crypto::ring::default_provider;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::{any::Any, time::Duration};
use tokio::net::TcpStream;
use tokio::sync::{broadcast, Mutex};
use tokio::time::timeout;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::{connect_async_tls_with_config, Connector};
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
use url::Url;

use crate::provider::error::{ClientError, Result};

pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(5);
pub const CONNECTION_RETRY_TIMEOUT: Duration = Duration::from_secs(15);
pub const CONNECTION_RETRY_INTERVAL: Duration = Duration::from_millis(100);
pub const SUBSCRIPTION_BUFFER: usize = 1000;
pub const UNSUBSCRIBE_GRACE_PERIOD: Duration = Duration::from_secs(3);
pub const PING_INTERVAL: Duration = Duration::from_secs(30);
pub const PING_WRITE_WAIT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct SubscriptionEntry<T> {
    pub active: bool,
    pub sender: broadcast::Sender<T>,
}

pub trait AnySubscription: Send + Sync {
    fn is_active(&self) -> bool;
    fn set_active(&mut self, active: bool);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Clone + Send + Sync + 'static> AnySubscription for SubscriptionEntry<T> {
    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
struct ResponseUpdate {
    response: String,
}

struct RequestTracker {
    ch: tokio::sync::mpsc::Sender<ResponseUpdate>,
}

pub struct WS {
    stream: Arc<Mutex<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>>,
    write_tx: tokio::sync::mpsc::Sender<Message>,
    shutdown_tx: broadcast::Sender<()>,
    request_id: AtomicU64,
    request_map: Arc<Mutex<HashMap<u64, RequestTracker>>>,
}

impl WS {
    pub async fn new(endpoint: String, auth_header: String) -> Result<Self> {
        let url =
            Url::parse(&endpoint).map_err(|e| ClientError::new("Invalid WebSocket URL:", e))?;

        let stream = Self::connect(&url, &auth_header).await?;
        let stream = Arc::new(Mutex::new(stream));

        let (write_tx, write_rx) = tokio::sync::mpsc::channel(100);
        let (shutdown_tx, _) = broadcast::channel::<()>(1);

        let ws = Self {
            stream: stream.clone(),
            write_tx,
            shutdown_tx,
            request_id: AtomicU64::new(0),
            request_map: Arc::new(Mutex::new(HashMap::new())),
        };

        ws.start_loops(stream.clone(), write_rx);

        Ok(ws)
    }

    fn start_loops(
        &self,
        stream: Arc<Mutex<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>>,
        mut write_rx: tokio::sync::mpsc::Receiver<Message>,
    ) {
        // Write loop
        // let mut shutdown_rx: broadcast::Receiver<()> = self.shutdown_tx.subscribe();
        let write_stream = stream.clone();
        tokio::spawn(async move {
            while let Some(msg) = write_rx.recv().await {
                println!("Write loop: Sending message: {}", msg);
                let mut stream = write_stream.lock().await;
                if let Err(e) = stream.send(msg).await {
                    eprintln!("Write error: {}", e);
                    break;
                }
            }
        });

        // Read loop
        let read_stream = stream.clone();
        let request_map = self.request_map.clone();
        tokio::spawn(async move {
            loop {
                let mut stream = read_stream.lock().await;

                // Use a short timeout for reading
                match timeout(Duration::from_millis(100), stream.next()).await {
                    Ok(Some(Ok(Message::Text(text)))) => {
                        if let Ok(value) = serde_json::from_str::<Value>(&text) {
                            if let Some(id) = value.get("id").and_then(|id| id.as_u64()) {
                                let request_map = request_map.lock().await;
                                if let Some(tracker) = request_map.get(&id) {
                                    let update = ResponseUpdate { response: text };
                                    if let Err(e) = tracker.ch.send(update).await {
                                        println!("Failed to send response: {}", e);
                                    }
                                }
                            }
                        }
                    }
                    Ok(Some(Ok(Message::Close(_)))) => {
                        break;
                    }
                    Ok(Some(Err(e))) => {
                        println!("Read loop: WebSocket error: {}", e);
                        break;
                    }
                    Ok(None) => {
                        println!("Read loop: Stream ended");
                        break;
                    }
                    Err(_) => continue, // Timeout, just try again
                    _ => continue,
                }
            }
        });

        // Ping loop
        let ping_stream = stream.clone();
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(PING_INTERVAL);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let mut stream = ping_stream.lock().await;
                        if let Err(e) = stream.send(Message::Ping(vec![])).await {
                            eprintln!("Ping error: {}", e);
                            break;
                        }
                    }
                    Ok(_) = shutdown_rx.recv() => {
                        break;
                    }
                }
            }
        });
    }

    pub async fn request<Req, Resp>(&self, method: &str, request: &Req) -> Result<Resp>
    where
        Req: prost::Message + serde::Serialize,
        Resp: prost::Message + Default + serde::de::DeserializeOwned,
    {
        let request_id = self.request_id.fetch_add(1, Ordering::SeqCst);

        let params = serde_json::to_value(request)
            .map_err(|e| ClientError::new("Failed to serialize request:", e))?;

        let request_json = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": method,
            "params": params
        });

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        {
            let mut request_map = self.request_map.lock().await;
            request_map.insert(request_id, RequestTracker { ch: tx });
        }

        let msg = Message::Text(request_json.to_string());
        match timeout(Duration::from_secs(5), self.write_tx.send(msg)).await {
            Ok(result) => result.map_err(|e| ClientError::new("Failed to send request:", e))?,
            Err(_) => {
                return Err(ClientError::new(
                    "Send timeout",
                    "Failed to send within timeout",
                ))
            }
        }

        match timeout(Duration::from_secs(10), rx.recv()).await {
            Ok(Some(update)) => {
                let json_response: Value = serde_json::from_str(&update.response)
                    .map_err(|e| ClientError::new("Failed to parse response:", e))?;

                if let Some(error) = json_response.get("error") {
                    return Err(ClientError::new(
                        "RPC error:",
                        anyhow::anyhow!("{}", error.to_string()),
                    ));
                }

                let result = json_response.get("result").ok_or_else(|| {
                    ClientError::new(
                        "Missing result in response:",
                        anyhow::anyhow!("no result field"),
                    )
                })?;

                let resp = serde_json::from_value(result.clone())
                    .map_err(|e| ClientError::new("Failed to parse result:", e))?;

                Ok(resp)
            }
            Ok(None) => Err(ClientError::new(
                "Channel closed",
                "Response channel closed unexpectedly",
            )),
            Err(_) => Err(ClientError::new(
                "Response timeout",
                "No response received within timeout",
            )),
        }
    }

    async fn connect(
        url: &Url,
        auth_header: &str,
    ) -> Result<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>> {
        let mut retry_count = 0;
        loop {
            let mut request = url
                .as_str()
                .into_client_request()
                .map_err(|e| ClientError::new("Failed to build request:", e))?;

            // Match Go's headers exactly
            let headers = request.headers_mut();
            headers.insert("Authorization", auth_header.parse().unwrap());
            headers.insert("x-sdk", "rust-client".parse().unwrap());
            headers.insert("x-sdk-version", env!("CARGO_PKG_VERSION").parse().unwrap());
            headers.insert("Connection", "Upgrade".parse().unwrap());
            headers.insert("Upgrade", "websocket".parse().unwrap());
            headers.insert("Sec-WebSocket-Version", "13".parse().unwrap());

            // Configure WebSocket with all required fields
            let ws_config = WebSocketConfig {
                ..Default::default() // 64KB
            };

            // Install the default crypto provider
            default_provider().install_default().map_err(|e| {
                ClientError::new(
                    "Failed to install crypto provider:",
                    anyhow::anyhow!("{:?}", e),
                )
            })?;

            let root_store = RootCertStore {
                roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
            };

            let tls_config = ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth();

            let connector = Arc::new(tls_config);

            match connect_async_tls_with_config(
                request,
                Some(ws_config),
                true, // Enable no_delay
                Some(Connector::Rustls(connector)),
            )
            .await
            {
                Ok((stream, _response)) => {
                    println!("Connected to: {}", url);
                    return Ok(stream);
                }
                Err(e) => {
                    if retry_count
                        >= (CONNECTION_RETRY_TIMEOUT.as_millis()
                            / CONNECTION_RETRY_INTERVAL.as_millis())
                            as u32
                    {
                        return Err(ClientError::new("WebSocket connection failed:", e));
                    }
                    retry_count += 1;
                    tokio::time::sleep(CONNECTION_RETRY_INTERVAL).await;
                }
            }
        }
    }

    pub async fn close(self) -> Result<()> {
        let _ = self.shutdown_tx.send(());

        {
            let mut request_map = self.request_map.lock().await;
            for (_, tracker) in request_map.drain() {
                let _ = tracker
                    .ch
                    .send(ResponseUpdate {
                        response: String::from("{\"error\":\"connection closed\"}"),
                    })
                    .await;
            }
        }

        let mut stream = self.stream.lock().await;
        if let Err(e) = stream.close(None).await {
            println!("Error during WebSocket close: {}", e);
        }
        drop(stream);

        tokio::time::sleep(Duration::from_millis(100)).await;

        println!("WebSocket shutdown complete");
        Ok(())
    }
}
