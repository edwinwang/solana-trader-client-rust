# solana-trader-client-rust

With the `solana-trader-proto` in the right path:

``solana-trader-proto = { path = "../../services/solana-trader-proto/rust" }``

**this will be updated to the published crate**

# Running tests
Since these tests are networked, they have the ignore flag on by default:

```rust
    #[tokio::test]
    #[ignore]
```

So each test must be called individually:

```bash
cargo test test_raydium_quotes_grpc -- --ignored 
```

## Adding new test cases
Using the `test_case` crate tests are parametrized:

```rust
// old test
#[test_case("BTC", "USDC", 0.1, 0.1 ; "BTC to USDC with 0.1% slippage")]
// new test case
#[test_case("BTC", "USDC", 0.1, 10 ; "BTC to USDC with 10% slippage")]
```

## Vscode 
Tests can also be ran/debugged on click with vscode. 
Just add a `settings.json` inside the `.vscode` folder, paste this snippet, and fill in the auth key:

```json
{
    "rust-analyzer.runnables.extraEnv": {
        "AUTH_HEADER": "<AUTH_HEADER>"
    },
    "rust-analyzer.runnables.extraArgs": [
        "--",
        "--ignored"
    ],
}
```