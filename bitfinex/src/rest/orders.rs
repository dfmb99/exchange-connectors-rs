use crate::commons::errors::*;
use crate::rest::client::*;
use serde_json::{from_str, to_string, Map, Value};
use std::fmt::Display;

pub enum OrderType {
    Limit,
    ExchangeLimit,
    Market,
    ExchangeMarket,
    Stop,
    ExchangeStop,
    StopLimit,
    ExchangeStopLimit,
    TrailingStop,
    ExchangeTrailingStop,
    FOK,
    ExchangeFOK,
    IOC,
    ExchangeIOC,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::ExchangeLimit => write!(f, "EXCHANGE LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::ExchangeMarket => write!(f, "EXCHANGE MARKET"),
            Self::Stop => write!(f, "STOP"),
            Self::ExchangeStop => write!(f, "EXCHANGE STOP"),
            Self::StopLimit => write!(f, "STOP LIMIT"),
            Self::ExchangeStopLimit => write!(f, "EXCHANGE STOP LIMIT"),
            Self::TrailingStop => write!(f, "TRAILING STOP"),
            Self::ExchangeTrailingStop => write!(f, "EXCHANGE TRAILING STOP"),
            Self::FOK => write!(f, "FOK"),
            Self::ExchangeFOK => write!(f, "EXCHANGE FOK"),
            Self::IOC => write!(f, "IOC"),
            Self::ExchangeIOC => write!(f, "EXCHANGE IOC"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrdersUpdate {
    pub update_timestamp: i64,
    pub order_type: String,
    pub message_id: Option<i64>,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    pub order_data: Vec<OrderData>,
    pub code: Option<i32>,
    pub status: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrderUpdate {
    pub update_timestamp: i64,
    pub order_type: String,
    pub message_id: Option<i64>,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    pub order_data: OrderData,
    pub code: Option<i32>,
    pub status: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderData {
    pub id: i64,
    pub group_id: Option<i64>,
    pub client_id: i64,
    pub symbol: String,
    pub creation_timestamp: i64,
    pub update_timestamp: i64,
    pub amount: f64,
    pub amount_original: f64,
    pub order_type: String,
    pub previous_order_type: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,

    pub flags: Option<i32>,
    pub order_status: String,

    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,

    pub price: f64,
    pub price_avg: f64,
    pub price_trailing: Option<f64>,
    pub price_aux_limit: Option<f64>,

    #[serde(skip_serializing)]
    __placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,

    pub notify: i32,
    pub hidden: i32,
    pub placed_id: Option<i32>,
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,
    pub routing: String,
    #[serde(skip_serializing)]
    _placeholder_10: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_11: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Map<String, Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct Trade {
    pub id: i64,
    pub pair: String,
    pub creation_timestamp: i64,
    pub order_id: i64,
    pub exec_amount: f64,
    pub exec_price: f64,
    pub order_type: String,
    pub order_price: f64,
    pub maker: i32,
    pub fee: f64,
    pub fee_currency: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TradeParams {
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub sort: Option<i32>,
    pub limit: Option<i64>,
}

impl TradeParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "start",
            self.start
                .map(|a| a.to_string())
                .unwrap_or_else(|| "".into()),
            "end",
            self.end.map(|a| a.to_string()).unwrap_or_else(|| "".into()),
            "sort",
            self.sort
                .map(|a| a.to_string())
                .unwrap_or_else(|| "".into()),
            "limit",
            self.limit
                .map(|a| a.to_string())
                .unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrderSubmitParams {
    #[serde(rename = "gid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(rename = "cid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<i64>,
    #[serde(rename = "type")]
    pub order_type: String,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(rename = "lev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_trailing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_aux_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_oco_stop: Option<String>,
    #[serde(rename = "tif")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrderUpdateParams {
    pub id: i64,
    #[serde(rename = "cid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<i64>,
    #[serde(rename = "cid_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_date: Option<i64>,
    #[serde(rename = "gid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    pub amount: String,
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(rename = "lev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_aux_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_trailing: Option<String>,
    #[serde(rename = "tif")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrderCancelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "cid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<i64>,
    #[serde(rename = "cid_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_date: Option<i64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrderMultiCancelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<i64>>,
    #[serde(rename = "cid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<i64>,
    #[serde(rename = "cid_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_date: Option<String>,
    #[serde(rename = "gid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<i32>,
}

#[derive(Clone)]
pub struct Orders {
    client: Client,
}

impl Orders {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Orders {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn active_orders(&self) -> Result<Vec<OrderData>> {
        let payload: String = "{}".to_string();

        self.orders_read("orders".to_owned(), payload)
    }

    pub fn submit_order(&self, params: &OrderSubmitParams) -> Result<OrdersUpdate> {
        let payload: String = to_string(params)?;

        let data = self
            .client
            .post_signed_write("order/submit".to_owned(), payload)?;

        let order: OrdersUpdate = from_str(data.as_str())?;

        Ok(order)
    }

    pub fn update_order(&self, params: &OrderUpdateParams) -> Result<OrderUpdate> {
        let payload: String = to_string(params)?;

        let data = self
            .client
            .post_signed_write("order/update".to_owned(), payload)?;

        let order: OrderUpdate = from_str(data.as_str())?;

        Ok(order)
    }

    pub fn cancel_order(&self, params: &OrderCancelParams) -> Result<OrderUpdate> {
        let payload: String = to_string(params)?;

        let data = self
            .client
            .post_signed_write("order/cancel".to_owned(), payload)?;

        let order: OrderUpdate = from_str(data.as_str())?;

        Ok(order)
    }

    pub fn cancel_multi_orders(&self, params: &OrderMultiCancelParams) -> Result<OrdersUpdate> {
        let payload: String = to_string(params)?;

        let data = self
            .client
            .post_signed_write("order/cancel/multi".to_owned(), payload)?;

        let order: OrdersUpdate = from_str(data.as_str())?;

        Ok(order)
    }

    pub fn history<T>(&self, symbol: T) -> Result<Vec<OrderData>>
    where
        T: Into<Option<String>>,
    {
        let value = symbol.into().unwrap_or_else(|| "".into());
        let payload: String = "{}".to_string();

        if value.is_empty() {
            self.orders_read("orders/hist".into(), payload)
        } else {
            let request: String = format!("orders/{}/hist", value);
            self.orders_read(request, payload)
        }
    }

    pub fn orders_read<S>(&self, request: S, payload: S) -> Result<Vec<OrderData>>
    where
        S: Into<String>,
    {
        let data = self
            .client
            .post_signed_read(request.into(), payload.into())?;

        let orders: Vec<OrderData> = from_str(data.as_str())?;

        Ok(orders)
    }

    pub fn trades<S>(&self, symbol: S, params: &TradeParams) -> Result<Vec<Trade>>
    where
        S: Into<String>,
    {
        let payload: String = to_string(params)?;
        let data = self
            .client
            .post_signed_read(format!("trades/{}/hist", symbol.into()), payload)?;

        let trades: Vec<Trade> = from_str(data.as_str())?;

        Ok(trades)
    }
}
