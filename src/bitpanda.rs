use anyhow;
use async_trait::async_trait;

use crate::types::*;

const BASE_URL: &str = "https://api.exchange.bitpanda.com/public/v1/";

struct Bitpanda;

impl BitpandaBackend for Bitpanda {
}

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
    use tokio;
    use super::{*, trading_pairs::*};

    #[test]
    fn trading_pair_serde() {
        assert_eq!(
            r#""DOGE_EUR""#,
            serde_json::to_string(&DOGE_EUR).as_deref().unwrap()
        );
        assert_eq!(
            DOGE_EUR,
            serde_json::from_str::<TradingPair>(r#""DOGE_EUR""#).unwrap()
        );
    }

    #[test]
    fn market_tickers() {
        assert_eq!(
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(Bitpanda::market_tickers())
                .unwrap()
                .len(),
            24
        );
    }
}
