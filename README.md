# Solana Trader Rust SDK

## Objective

This SDK is designed to make it easy for you to use the bloXroute Labs API in Rust. 

## Installation

``cargo add solana-trader-client-rust``

or

```toml
[dependencies]
solana-trader-client-rust = "0.1.0"
```

## Usage

The SDK provides access to Solana Trader API through:

gRPC: High-performance RPC calls
HTTP: Simple REST requests
WebSocket: Real-time streaming data


### Client Initialization

Refer to **SETUP.md** for available networks, create and populate your `.env` file with something like this:

```bash
PUBLIC_KEY="...."
PRIVATE_KEY="......."
AUTH_HEADER="......"
NETWORK=MAINNET
REGION=NY
```

```rust
use solana_trader_client_rust::provider::{
    grpc::GrpcClient, 
    http::HTTPClient,
    ws::WebSocketClient
};

// GRPC
let grpc_client = GrpcClient::new(None).await?;

// HTTP 
let http_client = HTTPClient::new(None)?;

// WebSocket
let ws_client = WebSocketClient::new(None).await?;
```

A simple example:

```rust
let request = api::GetRaydiumQuotesRequest {
    in_token: WRAPPED_SOL.to_string(),
    out_token: USDC.to_string(), 
    in_amount: 0.1,
    slippage: 0.2,
};

// Using GRPC
let response = grpc_client.get_raydium_quotes(&request).await?;

// Using HTTP
let response = http_client.get_raydium_quotes(&request).await?;

// Using WebSocket
let response = ws_client.get_raydium_quotes(&request).await?;
```

Please refer to the `tests` directory for more examples.

run `cargo clippy --tests` after adding your tests, to resolve any potential issues 
