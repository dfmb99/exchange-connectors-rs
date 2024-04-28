use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Margin {
    pub currency: String,
    #[serde(rename = "marginBalance")]
    pub margin_balance: u64,
    #[serde(rename = "availableMargin")]
    pub available_balance: u64,
    #[serde(rename = "walletBalance")]
    pub wallet_balance: u64,
}
