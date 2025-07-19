use crate::rest::account::Position;
use crate::rest::book::{
    FundingCurrency as BookFundingCurrency, RawBook, TradingPair as BookTradingPair,
};
use crate::rest::candles::Candle;
use crate::rest::orders::OrderData;
use crate::rest::ticker::*;
use crate::rest::trades::{
    FundingCurrency as TradesFundingCurrency, TradingPair as TradesTradingPair,
};
use crate::websocket::model::{BalanceInfo, Trade, Wallet};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum NotificationEvent {
    Auth(AuthMessage),
    Info(InfoMessage),
    TradingSubscribed(TradingSubscriptionMessage),
    FundingSubscribed(FundingSubscriptionMessage),
    CandlesSubscribed(CandlesSubscriptionMessage),
    RawBookSubscribed(RawBookSubscriptionMessage),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DataEvent {
    TickerTradingEvent(i32, TradingPair),
    TickerFundingEvent(i32, FundingCurrency),
    TradesTradingSnapshotEvent(i32, Vec<TradesTradingPair>),
    TradesTradingUpdateEvent(i32, String, TradesTradingPair),
    TradesFundingSnapshotEvent(i32, Vec<TradesFundingCurrency>),
    TradesFundingUpdateEvent(i32, String, TradesFundingCurrency),
    BookTradingSnapshotEvent(i32, Vec<BookTradingPair>),
    BookTradingUpdateEvent(i32, BookTradingPair),
    BookFundingSnapshotEvent(i32, Vec<BookFundingCurrency>),
    BookFundingUpdateEvent(i32, BookFundingCurrency),
    RawBookEvent(i32, RawBook),
    RawBookUpdateEvent(i32, Vec<RawBook>),
    CandlesSnapshotEvent(i32, Vec<Candle>),
    CandlesUpdateEvent(i32, Candle),
    HeartbeatEvent(i32, String),
    OrdersSnapshotEvent(i32, String, Vec<OrderData>),
    OrdersUpdateEvent(i32, String, Box<OrderData>),
    PositionsSnapshotEvent(i32, String, Vec<Position>),
    PositionsUpdateEvent(i32, String, Position),
    TradesUpdateEvent(i32, String, Trade),
    WalletsSnapshotEvent(i32, String, Vec<Wallet>),
    WalletsUpdateEvent(i32, String, Wallet),
    BalanceInfoUpdateEvent(i32, String, BalanceInfo),
    Other(Value),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthMessage {
    pub event: String,
    pub status: String,
    pub chan_id: u32,
    pub code: Option<u32>,
    pub msg: Option<String>,
    pub user_id: Option<u32>,
    pub auth_id: Option<String>,
}

impl AuthMessage {
    pub fn is_ok(&self) -> bool {
        self.status == "OK"
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoMessage {
    pub event: String,
    pub version: u16,
    pub server_id: String,
    pub platform: Platform,
}

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub status: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingSubscriptionMessage {
    pub event: String,
    pub channel: String,
    pub chan_id: u32,
    pub symbol: String,
    pub pair: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingSubscriptionMessage {
    pub event: String,
    pub channel: String,
    pub chan_id: u32,
    pub symbol: String,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlesSubscriptionMessage {
    pub event: String,
    pub channel: String,
    pub chan_id: u32,
    pub key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawBookSubscriptionMessage {
    pub event: String,
    pub channel: String,
    pub chan_id: u32,
    pub symbol: String,
    pub prec: String,
    pub freq: String,
    pub len: String,
    pub pair: String,
}
