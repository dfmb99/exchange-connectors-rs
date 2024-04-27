use crate::rest::client::*;
use crate::commons::errors::*;
use serde_json::{from_str, to_string, Map, Value};

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub wallet_type: String,
    pub currency: String,
    pub balance: f64,
    pub unsettled_interest: f64,
    pub balance_available: f64,
    pub last_change: Option<String>,
    pub trade_details: Option<Map<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub status: String,
    pub amount: f64,
    pub base_price: f64,
    pub funding: f64,
    pub funding_type: i16,
    pub pl: f64,
    pub pl_perc: f64,
    pub price_liq: f64,
    pub leverage: f64,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    pub position_id: u64,
    pub mts_created: Option<u64>,
    pub mts_update: Option<u64>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub pos_type: i16,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    pub collaterall: f64,
    pub collaterall_min: f64,
    pub meta: Option<Map<String, Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct MarginBase {
    key: String,
    pub margin: Base,
}

#[derive(Serialize, Deserialize)]
pub struct Base {
    pub user_profit_loss: f64,
    pub user_swaps: f64,
    pub margin_balance: f64,
    pub margin_net: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MarginSymbol {
    key: String,
    symbol: String,
    pub margin: Symbol,
}

#[derive(Serialize, Deserialize)]
pub struct Symbol {
    pub tradable_balance: f64,
    pub gross_balance: f64,
    pub buy: f64,
    pub sell: f64,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FundingInfo {
    key: String,
    symbol: String,
    pub funding: FundingData,
}

#[derive(Serialize, Deserialize)]
pub struct FundingData {
    pub yield_loan: f64,
    pub yield_lend: f64,
    pub duration_loan: f64,
    pub duration_lend: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TransferWallet {
    pub update_timestamp: i64,
    pub order_type: String,
    pub message_id: Option<i64>,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    pub order_data: TransferWalletData,
    pub code: Option<i32>,
    pub status: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransferWalletData {
    pub update_timestamp: i64,
    pub wallet_from: String,
    pub wallet_to: String,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    pub currency: String,
    pub currency_to: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FeeSummary {
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    pub data: (FeeSummaryFirst, FeeSummarySecond),
    #[serde(skip_serializing)]
    _placeholder_5: Option<Value>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<Value>,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,
    pub leo_info: Value,
}

#[derive(Serialize, Deserialize)]
pub struct FeeSummaryFirst {
    pub maker_fee: f64,
    #[serde(skip_serializing)]
    _placeholder_1: Option<f64>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<f64>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    pub deriv_rebate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FeeSummarySecond {
    pub taker_fee_crypto: f64,
    pub taker_fee_stable: f64,
    pub taker_fee_fiat: f64,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub deriv_taker_fee: f64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TransferWalletParams {
    pub from: String,
    pub to: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_to: Option<String>,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_dst: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Movements {
    pub id: f64,
    pub currency: String,
    pub currency_name: String,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub create_timestamp: i64,
    pub update_timestamp: i64,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    pub status: String,
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    pub amount: f64,
    pub fees: f64,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,
    pub destination_adrr: String,
    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_10: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_11: Option<String>,
    pub transaction_id: String,
    pub withdraw_transaction_time: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MovementParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl MovementParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}",
            "start",
            self.start
                .map(|a| a.to_string())
                .unwrap_or_else(|| "".into()),
            "end",
            self.end.map(|a| a.to_string()).unwrap_or_else(|| "".into()),
            "limit",
            self.limit
                .map(|a| a.to_string())
                .unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct AvailableBalance {
    pub amount_available: f64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AvailableBalanceParams {
    pub symbol: String,
    // Direction of the order (1 for by, -1 for sell) (Mandatory for EXCHANGE and MARGIN type, not used for FUNDING)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<i32>,
    // Order price (Mandatory for EXCHANGE and MARGIN type, not used for FUNDING)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    // Type of the order/offer EXCHANGE, MARGIN, DERIV, or FUNDING
    #[serde(rename = "type")]
    pub order_type: String,
    // Leverage that you want to use in calculating the max order amount (DERIV only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lev: Option<String>,
}

#[derive(Clone)]
pub struct Account {
    client: Client,
}

impl Account {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Account {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_wallets(&self) -> Result<Vec<Wallet>> {
        let payload: String = "{}".to_string();
        let data = self.client.post_signed_read("wallets".into(), payload)?;

        let wallets: Vec<Wallet> = from_str(data.as_str())?;

        Ok(wallets)
    }

    pub fn get_active_positions(&self) -> Result<Vec<Position>> {
        let payload: String = "{}".to_string();
        let data = self.client.post_signed_read("positions".into(), payload)?;

        let wallets: Vec<Position> = from_str(data.as_str())?;

        Ok(wallets)
    }

    pub fn margin_symbol<S>(&self, key: S) -> Result<MarginSymbol>
    where
        S: Into<String>,
    {
        let payload: String = "{}".to_string();
        let request: String = format!("info/margin/{}", key.into());

        let data = self.client.post_signed_read(request, payload)?;

        let margin: MarginSymbol = from_str(data.as_str())?;

        Ok(margin)
    }

    pub fn funding_info<S>(&self, key: S) -> Result<FundingInfo>
    where
        S: Into<String>,
    {
        let payload: String = "{}".to_string();
        let request: String = format!("info/funding/f{}", key.into());

        let data = self.client.post_signed_read(request, payload)?;

        let info: FundingInfo = from_str(data.as_str())?;

        Ok(info)
    }

    pub fn transfer_between_wallets(
        &self, params: &TransferWalletParams,
    ) -> Result<TransferWallet> {
        let payload: String = to_string(params)?;

        let data = self.client.post_signed_write("transfer".into(), payload)?;

        let info: TransferWallet = from_str(data.as_str())?;

        Ok(info)
    }

    pub fn movements<S>(&self, currency: S, params: &MovementParams) -> Result<Vec<Movements>>
    where
        S: Into<String>,
    {
        let payload: String = to_string(params)?;
        let request: String = format!("movements/{}/hist", currency.into());

        let data = self.client.post_signed_read(request, payload)?;

        let movements: Vec<Movements> = from_str(data.as_str())?;

        Ok(movements)
    }

    pub fn available_balance(&self, params: &AvailableBalanceParams) -> Result<AvailableBalance> {
        let payload: String = to_string(params)?;

        let data = self
            .client
            .post_signed("calc/order/avail".into(), payload)?;

        let balance: AvailableBalance = from_str(data.as_str())?;

        Ok(balance)
    }

    pub fn fee_summary(&self) -> Result<FeeSummary> {
        let payload: String = "{}".to_string();

        let data = self.client.post_signed_read("summary".into(), payload)?;

        let fee_summaru: FeeSummary = from_str(data.as_str())?;

        Ok(fee_summaru)
    }
}
