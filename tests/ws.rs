#[cfg(test)]
mod tests {
    use futures_util::StreamExt;
    use solana_trader_client_rust::{
        common::{USDC, WRAPPED_SOL},
        provider::ws::WebSocketClient,
    };
    use solana_trader_proto::api;
    use std::{error::Error, time::Duration};
    use test_case::test_case;
    use tokio::time::timeout;

    #[test_case(
        WRAPPED_SOL,
        USDC,
        0.01,
        5.0 ;
        "SOL to USDC quote"
    )]
    #[tokio::test]
    #[ignore]
    async fn test_raydium_quotes_ws(
        in_token: &str,
        out_token: &str,
        in_amount: f64,
        slippage: f64,
    ) -> Result<(), Box<dyn Error>> {
        let client = WebSocketClient::new(None).await?;

        let request = api::GetRaydiumQuotesRequest {
            in_token: in_token.to_string(),
            out_token: out_token.to_string(),
            in_amount,
            slippage,
        };

        let result = timeout(Duration::from_secs(3), client.get_raydium_quotes(&request)).await??;

        assert!(
            !result.routes.is_empty(),
            "Expected at least one route in response"
        );

        client.close().await?;
        Ok(())
    }

    // NOTE: trade stream is very low in activity, so this will run for a while and may expire.
    #[test_case(
        "SOL/USDC",
        0,
        api::Project::POpenbook ;
        "SOL/USDC trades stream"
    )]
    #[tokio::test]
    #[ignore]
    async fn test_trades_stream_ws(
        market: &str,
        limit: i32,
        project: api::Project,
    ) -> Result<(), Box<dyn Error>> {
        let ws = WebSocketClient::new(None).await?;

        let mut trade_stream = ws
            .get_trades_stream(market.to_string(), limit.try_into().unwrap(), project)
            .await?;

        if let Some(update) = trade_stream.next().await {
            println!("trade stream received: {:?}", update);
            // assert!(trade.trades.unwrap().trades[0]. > 0.0, "Trade price should be positive");
        }

        ws.close().await?;
        Ok(())
    }

    #[test_case(
        vec![api::Project::PRaydium],
        vec![WRAPPED_SOL.to_string()] ;
        "raydium SOL price stream"
    )]
    #[tokio::test]
    #[ignore]
    async fn test_prices_stream_ws(
        projects: Vec<api::Project>,
        tokens: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let ws = WebSocketClient::new(None).await?;

        let stream = ws.get_prices_stream(projects, tokens).await?;

        match stream.take(1).next().await {
            Some(Ok(response)) => {
                println!("response recieved: {:?}", response);
                assert!(
                    response.price.unwrap().buy > 0.0,
                    "Price should be positive"
                );
            }
            Some(Err(e)) => return Err(Box::new(e)),
            None => panic!("Stream ended unexpectedly"),
        }

        ws.close().await?;
        Ok(())
    }
}
