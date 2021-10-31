use algotrader::types::{
    Ccy::*,
    TradingPair,
    trading_pairs::*,
    // buy,
    // sell
};


// tokio main
fn main() {
    println!("Hello, world!");

    // starting point (bootstrap): if can buy and can sell == 0 -> buy

    for _ in tick(1sec) {
        let (market_data, virtual_wallets, virtual_books) = (MarketData::fetch().await?, VirtualWallet::fetch().await?, VirtualBook::fetch().await?);
        for pair in pairs { // pair = (buy_token, sell_token)
            if (virtual_wallets[buy_token].amount >= TRADE_SIZE && virtual_books[pair].last_buy_time - now > 1_hour) && market_data[pair].should_buy() {
                buy(pair, TRADE_SIZE, ACCOUNT_KEY) // send http request without reading response ? or maybe handle response elswhere in async. logs ? wrapper around bitpanda mod or direct use ?
            }
            //if virtual_books[pair].can_sell > 0 && virtual_wallets[sell_token].amount > 0 && market_data[pair].should_sell() { // should sell depends on trade too
            for trade in trade_history.get_buys_not_sold() { // instead of iterating on all trades, iterate on trades sorted by buying price and break as soon as one is not sold
                if should_sell(trade, market_data) {
                    sell(pair, virtual_books[pair].amount / virtual_books[pair].can_sell, ACCOUNT_KEY)
                }
            }
        }
    }
}

struct MarketData {
    // data (ccy -> bid, ask, avg)
    // algotrader’s virtual wallet {mapping ccy -> amount, no of trades to buy, no to sell}, make it in a separate struct ?
    // hm ccy -> Data {market_data: MarketData, wallet: Wallet}, no, 2 separate hashmaps is better
}

// need real wallet too ?
struct VirtualWallet { 
    amount: f64 // could be derived from trade history I guess ? i recompute it every second anyway
}

struct VirtualBook { // trade history ?
    can_buy: uint, // can be derived from trades ?
    can_sell: uint,
    trades: Vec<Trade> // put trades in pair buy sell, or buy not_yet_sold
}  

struct Trade {
    pair: TradingPair,
    price_per_unit: f64, // useless if 2 amounts ?
    amount_ccy1: f64,
    amount_ccy2: f64,
    // buy or sell trade ? derived from trading pair ? bool ? enum ?
}

impl MarketData {
    async fn fetch() -> HashMap<TradingPair, Self> {
        todo!()
    }

    fn should_buy(&self) -> bool {
        self.ask <= 0.975 * self.avg
        // && if I am at most recent min I dont buy, If it has gone up of X% from most recent min I buy -> only buy when it looks like it’s going up again
    }

    fn should_sell(&self) -> bool {
        self.bid >= 1.05 * this_trades_buying_price 
        // && if it has gone down of X% from max on last Y minutes, I sell -> only sell when it looks like it’s gonna go down
    }
}

impl VirtualWallet {
    async fn fetch() -> HashMap<TradingPair, Self> {
        todo!()
    }
}

impl VirtualBook {
    async fn fetch() -> HashMap<TradingPair, Self> {
        todo!()
    }
}