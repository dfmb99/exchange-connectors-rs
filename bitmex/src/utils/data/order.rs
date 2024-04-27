use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(rename = "clOrdID", default)]
    pub clord_id: String,
    pub symbol: String,
    pub side: String,
    #[serde(deserialize_with = "deserialize_null_default", rename = "orderQty")]
    pub order_qty: u64,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub price: f64,
    #[serde(rename = "ordType")]
    pub ord_type: String,
    #[serde(rename = "ordStatus")]
    pub ord_status: String,
    #[serde(deserialize_with = "deserialize_null_default", rename = "leavesQty")]
    pub leaves_qty: u64,
    #[serde(deserialize_with = "deserialize_null_default", rename = "cumQty")]
    pub cum_qty: u64,
    #[serde(
        deserialize_with = "deserialize_null_default",
        rename = "avgPx",
        default
    )]
    pub avg_price: f64,
    pub timestamp: String,
    pub text: String,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
