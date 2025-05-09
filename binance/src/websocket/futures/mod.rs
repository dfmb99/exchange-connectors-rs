use crate::commons::config::Config;
use crate::commons::errors::*;
use crate::rest::futures::model::{OrderBook, OrderTradeEvent};
use crate::rest::model::{
    AccountUpdateEvent, AggrTradesEvent, BookTickerEvent, ContinuousKlineEvent, DayTickerEvent,
    DepthOrderBookEvent, IndexKlineEvent, IndexPriceEvent, KlineEvent, LiquidationEvent,
    MarkPriceEvent, MiniTickerEvent, TradeEvent, UserDataStreamExpiredEvent,
};
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message};
use url::Url;

pub mod usdm;
pub mod usdm_data;
pub mod userstream;

#[allow(clippy::all)]
enum FuturesWebsocketAPI {
    Default,
    MultiStream(String),
    Custom(String),
}

pub enum FuturesMarket {
    USDM,
    COINM,
    Vanilla,
}

impl FuturesWebsocketAPI {
    fn params(self, market: FuturesMarket, subscription: &str) -> String {
        let baseurl = match market {
            FuturesMarket::USDM => "wss://fstream.binance.com",
            FuturesMarket::COINM => "wss://dstream.binance.com",
            FuturesMarket::Vanilla => "wss://vstream.binance.com",
        };

        match self {
            FuturesWebsocketAPI::Default => {
                format!("{baseurl}/ws/{subscription}")
            }
            FuturesWebsocketAPI::MultiStream(url) => {
                format!("{url}/stream?streams={subscription}")
            }
            FuturesWebsocketAPI::Custom(url) => format!("{url}/ws/{subscription}"),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FuturesWebsocketEvent {
    AccountUpdate(AccountUpdateEvent),
    OrderTrade(OrderTradeEvent),
    AggrTrades(AggrTradesEvent),
    Trade(TradeEvent),
    OrderBook(OrderBook),
    DayTicker(DayTickerEvent),
    MiniTicker(MiniTickerEvent),
    MiniTickerAll(Vec<MiniTickerEvent>),
    IndexPrice(IndexPriceEvent),
    MarkPrice(MarkPriceEvent),
    MarkPriceAll(Vec<MarkPriceEvent>),
    DayTickerAll(Vec<DayTickerEvent>),
    Kline(KlineEvent),
    ContinuousKline(ContinuousKlineEvent),
    IndexKline(IndexKlineEvent),
    Liquidation(LiquidationEvent),
    DepthOrderBook(DepthOrderBookEvent),
    BookTicker(BookTickerEvent),
    UserDataStreamExpiredEvent(UserDataStreamExpiredEvent),
}

pub struct FuturesWebSockets<'a> {
    pub socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(FuturesWebsocketEvent) -> Result<()> + 'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum FuturesEvents {
    Vec(Vec<DayTickerEvent>),
    DayTickerEvent(DayTickerEvent),
    BookTickerEvent(BookTickerEvent),
    MiniTickerEvent(MiniTickerEvent),
    VecMiniTickerEvent(Vec<MiniTickerEvent>),
    AccountUpdateEvent(AccountUpdateEvent),
    OrderTradeEvent(OrderTradeEvent),
    AggrTradesEvent(AggrTradesEvent),
    IndexPriceEvent(IndexPriceEvent),
    MarkPriceEvent(MarkPriceEvent),
    VecMarkPriceEvent(Vec<MarkPriceEvent>),
    TradeEvent(TradeEvent),
    KlineEvent(KlineEvent),
    ContinuousKlineEvent(ContinuousKlineEvent),
    IndexKlineEvent(IndexKlineEvent),
    LiquidationEvent(LiquidationEvent),
    OrderBook(OrderBook),
    DepthOrderBookEvent(DepthOrderBookEvent),
    UserDataStreamExpiredEvent(UserDataStreamExpiredEvent),
}

impl<'a> FuturesWebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> FuturesWebSockets<'a>
    where
        Callback: FnMut(FuturesWebsocketEvent) -> Result<()> + 'a,
    {
        FuturesWebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    pub fn connect(&mut self, market: FuturesMarket, subscription: &'a str) -> Result<()> {
        self.connect_wss(FuturesWebsocketAPI::Default.params(market, subscription))
    }

    pub fn connect_with_config(
        &mut self,
        market: FuturesMarket,
        subscription: &'a str,
        config: &'a Config,
    ) -> Result<()> {
        self.connect_wss(
            FuturesWebsocketAPI::Custom(config.futures_ws_endpoint.clone())
                .params(market, subscription),
        )
    }

    pub fn connect_multiple_streams(
        &mut self,
        market: FuturesMarket,
        endpoints: &[String],
        config: &Config,
    ) -> Result<()> {
        self.connect_wss(
            FuturesWebsocketAPI::MultiStream(config.futures_ws_endpoint.clone())
                .params(market, &endpoints.join("/")),
        )
    }

    fn connect_wss(&mut self, wss: String) -> Result<()> {
        let url = Url::parse(&wss)?;
        match connect(url.as_str()) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(BinanceError::WebSocket(WebSocketError::ConnectionError(
                e.to_string(),
            ))),
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            return Ok(());
        }
        Err(BinanceError::WebSocket(WebSocketError::Disconnected))
    }

    pub fn test_handle_msg(&mut self, msg: &str) -> Result<()> {
        self.handle_msg(msg)
    }

    fn handle_msg(&mut self, msg: &str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(msg)?;

        if let Some(data) = value.get("data") {
            self.handle_msg(&data.to_string())?;
            return Ok(());
        }

        if let Ok(events) = serde_json::from_value::<FuturesEvents>(value) {
            let action = match events {
                FuturesEvents::Vec(v) => FuturesWebsocketEvent::DayTickerAll(v),
                FuturesEvents::DayTickerEvent(v) => FuturesWebsocketEvent::DayTicker(v),
                FuturesEvents::BookTickerEvent(v) => FuturesWebsocketEvent::BookTicker(v),
                FuturesEvents::MiniTickerEvent(v) => FuturesWebsocketEvent::MiniTicker(v),
                FuturesEvents::VecMiniTickerEvent(v) => FuturesWebsocketEvent::MiniTickerAll(v),
                FuturesEvents::AccountUpdateEvent(v) => FuturesWebsocketEvent::AccountUpdate(v),
                FuturesEvents::OrderTradeEvent(v) => FuturesWebsocketEvent::OrderTrade(v),
                FuturesEvents::IndexPriceEvent(v) => FuturesWebsocketEvent::IndexPrice(v),
                FuturesEvents::MarkPriceEvent(v) => FuturesWebsocketEvent::MarkPrice(v),
                FuturesEvents::VecMarkPriceEvent(v) => FuturesWebsocketEvent::MarkPriceAll(v),
                FuturesEvents::TradeEvent(v) => FuturesWebsocketEvent::Trade(v),
                FuturesEvents::ContinuousKlineEvent(v) => FuturesWebsocketEvent::ContinuousKline(v),
                FuturesEvents::IndexKlineEvent(v) => FuturesWebsocketEvent::IndexKline(v),
                FuturesEvents::LiquidationEvent(v) => FuturesWebsocketEvent::Liquidation(v),
                FuturesEvents::KlineEvent(v) => FuturesWebsocketEvent::Kline(v),
                FuturesEvents::OrderBook(v) => FuturesWebsocketEvent::OrderBook(v),
                FuturesEvents::DepthOrderBookEvent(v) => FuturesWebsocketEvent::DepthOrderBook(v),
                FuturesEvents::AggrTradesEvent(v) => FuturesWebsocketEvent::AggrTrades(v),
                FuturesEvents::UserDataStreamExpiredEvent(v) => {
                    FuturesWebsocketEvent::UserDataStreamExpiredEvent(v)
                }
            };
            (self.handler)(action)?;
        }
        Ok(())
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        let start_time = Instant::now();
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                if Instant::now().duration_since(start_time) >= Duration::from_secs(8 * 3600) {
                    socket.0.close(None)?;
                    break;
                }
                let message = socket.0.read()?;
                match message {
                    Message::Text(msg) => self.handle_msg(&msg)?,
                    Message::Ping(data) => {
                        socket.0.send(Message::Pong(data)).unwrap();
                    }
                    Message::Pong(_) | Message::Binary(_) => (),
                    Message::Close(_) => {
                        return Err(BinanceError::WebSocket(WebSocketError::Disconnected))
                    }
                    Message::Frame(_) => (),
                }
            }
        }
        Err(BinanceError::WebSocket(WebSocketError::LoopClosed))
    }
}
