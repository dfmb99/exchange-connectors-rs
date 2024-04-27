use crate::rest::client::*;
use crate::commons::errors::*;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingPair {
    pub bid: f64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency {
    pub frr: f64,
    pub bid: f64,
    pub bid_period: i64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_period: i64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub frr_amnt_avail: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingStats {
    pub timestamp: u64,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub frr: f64,
    pub avg_period: f64,
    pub funding_amount: f64,
    pub funding_amount_used: f64,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    pub funding_below_threshold: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketAvgPrice {
    pub price_avg: f64,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingStatusParams {
    pub limit: Option<i32>,
    pub start: Option<String>,
    pub end: Option<String>,
}

impl FundingStatusParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}",
            "limit",
            self.limit
                .map(|a| a.to_string())
                .unwrap_or_else(|| "".into()),
            "start",
            self.start.to_owned().unwrap_or_else(|| "".into()),
            "end",
            self.end.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

impl Default for Ticker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Ticker {
    client: Client,
}

impl Ticker {
    pub fn new() -> Self {
        Ticker {
            client: Client::new(None, None),
        }
    }

    pub fn funding_currency<S>(&self, symbol: S) -> Result<FundingCurrency>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("ticker/f{}", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let ticker: FundingCurrency = from_str(data.as_str())?;

        Ok(ticker)
    }

    pub fn trading_pair<S>(&self, symbol: S) -> Result<TradingPair>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("ticker/{}", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let ticker: TradingPair = from_str(data.as_str())?;

        Ok(ticker)
    }

    pub fn funding_stats<S>(
        &self, symbol: S, params: &FundingStatusParams,
    ) -> Result<Vec<FundingStats>>
    where
        S: Into<String>,
    {
        let data = self.client.get(
            format!("funding/stats/{}/hist", symbol.into()),
            params.to_query(),
        )?;

        let ticker: Vec<FundingStats> = from_str(data.as_str())?;

        Ok(ticker)
    }
}
