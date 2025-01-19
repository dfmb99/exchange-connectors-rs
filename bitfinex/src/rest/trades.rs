use crate::commons::errors::BitfinexError;
use crate::rest::client::*;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub id: i64,
    pub pair: String,
    pub execution_timestamp: i64,
    pub order_id: i32,
    pub execution_amount: f64,
    pub execution_price: f64,
    pub order_type: String,
    pub order_price: f64,
    pub maker: i32,
    pub fee: f64,
    pub fee_currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingPair {
    pub id: i64,
    pub mts: i64,
    pub amount: f64,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency {
    pub mts: i64,
    pub amount: f64,
    pub price: f64,
    pub rate: f64,
    pub period: i64,
}

#[derive(Clone)]
pub struct Trades {
    client: Client,
}

impl Default for Trades {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl Trades {
    pub fn new() -> Result<Self, BitfinexError> {
        Ok(Trades {
            client: Client::new(None, None),
        })
    }

    pub fn funding_currency<S>(&self, symbol: S) -> Result<Vec<FundingCurrency>, BitfinexError>
    where
        S: Into<String>,
    {
        let endpoint = format!("trades/{}/hist", symbol.into());
        let data = self.client.get(endpoint, String::new())?;
        let trades = from_str(&data).map_err(BitfinexError::JsonError)?;
        Ok(trades)
    }

    pub fn trading_pair<S>(&self, symbol: S) -> Result<Vec<TradingPair>, BitfinexError>
    where
        S: Into<String>,
    {
        let endpoint = format!("trades/{}/hist", symbol.into());
        let data = self.client.get(endpoint, String::new())?;
        let trades = from_str(&data).map_err(BitfinexError::JsonError)?;
        Ok(trades)
    }

    pub fn history<S>(&self, symbol: S) -> Result<Vec<Trade>, BitfinexError>
    where
        S: Into<String>,
    {
        let payload = "{}".to_string();
        let request = format!("trades/{}/hist", symbol.into());
        self.trades(request, payload)
    }

    pub fn generated_by_order<S>(&self, symbol: S, order_id: S) -> Result<Vec<Trade>, BitfinexError>
    where
        S: Into<String>,
    {
        let payload = "{}".to_string();
        let request = format!("order/{}:{}/trades", symbol.into(), order_id.into());
        self.trades(request, payload)
    }

    fn trades<S>(&self, request: S, payload: S) -> Result<Vec<Trade>, BitfinexError>
    where
        S: Into<String>,
    {
        let data = self
            .client
            .post_signed_read(request.into(), payload.into())?;
        let trades = from_str(&data).map_err(BitfinexError::JsonError)?;
        Ok(trades)
    }
}
