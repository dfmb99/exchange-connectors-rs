use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeBucketed {
    pub timestamp: String,
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub trades: u64,
    pub volume: u64,
    pub vwap: Option<f64>,
}
