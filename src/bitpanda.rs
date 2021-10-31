use anyhow;
use async_trait::async_trait;

use crate::types::*;

const BASE_URL: &str = "https://api.exchange.bitpanda.com/public/v1/";

struct Bitpanda;
impl BitpandaBackend for Bitpanda {}

#[async_trait]
pub trait BitpandaBackend {
    async fn market_tickers() -> anyhow::Result<Vec<MarketTicker>> {
        Ok(reqwest::Client::new()
            .get(format!("{}{}", BASE_URL, "market-ticker"))
            .header("Accept", "application/json")
            .send()
            .await?
            .json::<Vec<MarketTicker>>()
            .await?)
    }
}

#[cfg(test)]
mod test {
    use super::{Bitpanda, BitpandaBackend};
    use tokio;

    #[tokio::test]
    async fn market_tickers() {
        assert_eq!(Bitpanda::market_tickers().await.unwrap().len(), 24);
    }
}
