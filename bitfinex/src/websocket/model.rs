use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    pub id: i64,
    pub symbol: String,
    pub execution_timestamp: i64,
    pub order_id: i32,
    pub execution_amount: f64,
    pub execution_price: f64,
    pub order_type: String,
    pub order_price: f64,
    pub maker: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub wallet_type: String,
    pub currency: String,
    pub balance: f64,
    pub unsettled_interest: f64,
    pub balance_available: Option<f64>,
    pub description: Option<String>,
    pub meta: Option<Map<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceInfo {
    pub aum: f64,
    pub aum_net: f64,
}
