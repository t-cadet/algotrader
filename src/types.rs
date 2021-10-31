use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MarketTicker {
    instrument_code: TradingPair,
    sequence: u128,
    state: String,
    time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_u8_as_bool")]
    is_frozen: bool,
    quote_volume: Decimal,
    base_volume: Decimal,
    last_price: Decimal,
    best_bid: Decimal,
    best_ask: Decimal,
    price_change: Decimal,
    price_change_percentage: Decimal,
    high: Decimal,
    low: Decimal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TradingPair {
    pub base: Ccy,
    pub quote: Ccy,
}

impl Serialize for TradingPair {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}_{}", self.base.as_ref(), self.quote.as_ref()))
    }
}

impl<'de> Deserialize<'de> for TradingPair {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        match s.split('_').map(Ccy::from_str).collect::<Vec<_>>()[..] {
            [Ok(base), Ok(quote)] => Ok(Self { base, quote }),
            _ => Err(serde::de::Error::custom(format!(
                "Failed to deserialize `TradingPair` from `{}`",
                s
            ))),
        }
    }
}

fn deserialize_u8_as_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    u8::deserialize(deserializer).map(|number| number == 1)
}

#[rustfmt::skip]
pub mod trading_pairs {
    use super::{Ccy::*, TradingPair};

    pub const AAVE_EUR  : TradingPair = TradingPair {base: AAVE , quote: EUR};
    pub const ADA_EUR   : TradingPair = TradingPair {base: ADA  , quote: EUR};
    pub const BCH_EUR   : TradingPair = TradingPair {base: BCH  , quote: EUR};
    pub const BEST_BTC  : TradingPair = TradingPair {base: BEST , quote: BTC};
    pub const BEST_EUR  : TradingPair = TradingPair {base: BEST , quote: EUR};
    pub const BTC_CHF   : TradingPair = TradingPair {base: BTC  , quote: CHF};
    pub const BTC_EUR   : TradingPair = TradingPair {base: BTC  , quote: EUR};
    pub const BTC_GBP   : TradingPair = TradingPair {base: BTC  , quote: GBP};
    pub const CHZ_EUR   : TradingPair = TradingPair {base: CHZ  , quote: EUR};
    pub const DOGE_EUR  : TradingPair = TradingPair {base: DOGE , quote: EUR};
    pub const DOT_EUR   : TradingPair = TradingPair {base: DOT  , quote: EUR};
    pub const ETH_CHF   : TradingPair = TradingPair {base: ETH  , quote: CHF};
    pub const ETH_EUR   : TradingPair = TradingPair {base: ETH  , quote: EUR};
    pub const EOS_EUR   : TradingPair = TradingPair {base: EOS  , quote: EUR};
    pub const LINK_EUR  : TradingPair = TradingPair {base: LINK , quote: EUR};
    pub const LTC_EUR   : TradingPair = TradingPair {base: LTC  , quote: EUR};
    pub const MIOTA_EUR : TradingPair = TradingPair {base: MIOTA, quote: EUR};
    pub const PAN_EUR   : TradingPair = TradingPair {base: PAN  , quote: EUR};
    pub const USDT_EUR  : TradingPair = TradingPair {base: USDT , quote: EUR};
    pub const TRX_EUR   : TradingPair = TradingPair {base: TRX  , quote: EUR};
    pub const UNI_EUR   : TradingPair = TradingPair {base: UNI  , quote: EUR};
    pub const XLM_EUR   : TradingPair = TradingPair {base: XLM  , quote: EUR};
    pub const XRP_CHF   : TradingPair = TradingPair {base: XRP  , quote: CHF};
    pub const XRP_EUR   : TradingPair = TradingPair {base: XRP  , quote: EUR};
}

#[rustfmt::skip]
#[derive(AsRefStr, Clone, Copy, Debug, EnumString, PartialEq)]
pub enum Ccy {
    AAVE    , ADA   , BCH     , BEST    , BTC, 
    CHF     , CHZ   , DOGE    , DOT     , EOS,
    ETH     , EUR   , GBP     , LINK    , LTC,
    MIOTA   , PAN   , TRX     , TRY     , UNI,
    USDT    , XLM   , XRP     , XTZ     ,
}

#[cfg(test)]
mod test {
    use super::{trading_pairs::*, TradingPair};

    #[test]
    fn trading_pair_serde() {
        assert_eq!(
            serde_json::to_string(&DOGE_EUR).as_deref().unwrap(),
            r#""DOGE_EUR""#
        );
        assert_eq!(
            serde_json::from_str::<TradingPair>(r#""DOGE_EUR""#).unwrap(),
            DOGE_EUR
        );
    }
}
