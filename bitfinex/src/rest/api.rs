use crate::rest::account::*;
use crate::rest::book::*;
use crate::rest::candles::*;
use crate::rest::derivs::*;
use crate::rest::funding::Funding;
use crate::rest::ledger::*;
use crate::rest::orders::*;
use crate::rest::ticker::*;
use crate::rest::trades::*;

#[derive(Clone)]
pub struct Bitfinex {
    pub book: Book,
    pub ticker: Ticker,
    pub trades: Trades,
    pub candles: Candles,
    pub derivs: Derivs,
    pub orders: Orders,
    pub account: Account,
    pub ledger: Ledger,
    pub funding: Funding,
}

impl Bitfinex {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Bitfinex {
            book: Book::new(),
            ticker: Ticker::new(),
            trades: Trades::new().unwrap(),
            candles: Candles::new(),
            derivs: Derivs::new(api_key.clone(), secret_key.clone()),
            orders: Orders::new(api_key.clone(), secret_key.clone()),
            account: Account::new(api_key.clone(), secret_key.clone()),
            ledger: Ledger::new(api_key.clone(), secret_key.clone()),
            funding: Funding::new(api_key, secret_key),
        }
    }
}
