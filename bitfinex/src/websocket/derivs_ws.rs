use super::derivs_ws_data::DerivsWsData;
use crate::commons::errors::Error;
use crate::rest::account::Position;
use crate::rest::candles::Candle;
use crate::rest::orders::OrderData;
use crate::rest::ticker::TradingPair;
use crate::rest::trades::TradingPair as TradesTradingPair;
use crate::websocket::events::{DataEvent, NotificationEvent};
use crate::websocket::model::{BalanceInfo, Wallet};
use crate::websocket::websockets::{EventHandler, WebSockets};
use log::{debug, error, warn};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct DerivsWs {
    ws_data: DerivsWsData,
}

impl DerivsWs {
    /// Bitfinex derivs interface,
    pub fn new(symbol: String, api_key: String, api_secret: String) -> DerivsWs {
        let ws_data = DerivsWsData::default();
        let ws_int = DerivsWs {
            ws_data: ws_data.clone(),
        };
        ws_int.run(symbol, api_key, api_secret);
        ws_int.wait_for_data();
        fill_price_snaps(ws_data);
        ws_int
    }

    fn wait_for_data(&self) {
        while self.ws_data.get_trading_pair().is_none() {
            debug!("Waiting for data");
            thread::yield_now();
        }
    }

    pub fn run(&self, symbol: String, api_key: String, api_secret: String) {
        let ws_data = self.ws_data.clone();
        thread::spawn(move || loop {
            let mut web_socket: WebSockets = WebSockets::default();

            web_socket.add_event_handler(WebSocketHandler {
                ws_data: ws_data.clone(),
            });
            if web_socket.connect().is_ok() {
                web_socket.subscribe_ticker(symbol.to_owned());
                web_socket.subscribe_trades(symbol.to_owned());
                web_socket.subscribe_candles(symbol.to_owned(), "1m".into());
                web_socket
                    .auth(api_key.to_owned(), api_secret.to_owned(), false, &[])
                    .unwrap();

                if let Err(err) = web_socket.event_loop() {
                    error!("Websocket error: {}", err);
                }
            }
        });
    }

    pub fn get_trading_pair(&self) -> Option<TradingPair> {
        self.ws_data.get_trading_pair()
    }

    pub fn get_candles(&self) -> VecDeque<Candle> {
        self.ws_data.get_candles()
    }

    pub fn get_trades(&self) -> VecDeque<TradesTradingPair> {
        self.ws_data.get_trades()
    }

    pub fn get_positions(&self) -> VecDeque<Position> {
        self.ws_data.get_positions()
    }

    pub fn get_balance(&self) -> Option<BalanceInfo> {
        self.ws_data.get_balance()
    }

    pub fn get_wallet(&self) -> VecDeque<Wallet> {
        self.ws_data.get_wallet()
    }

    pub fn get_filled_orders(&self) -> VecDeque<OrderData> {
        self.ws_data.get_filled_orders()
    }

    pub fn get_open_orders(&self) -> VecDeque<OrderData> {
        self.ws_data.get_open_orders()
    }

    pub fn get_canceled_orders(&self) -> VecDeque<OrderData> {
        self.ws_data.get_canceled_orders()
    }

    pub fn get_filled_order(&self, order_id: i64) -> Option<OrderData> {
        self.ws_data.get_filled_order(order_id)
    }

    pub fn get_open_order(&self, order_id: i64) -> Option<OrderData> {
        self.ws_data.get_open_order(order_id)
    }

    pub fn get_canceled_order(&self, order_id: i64) -> Option<OrderData> {
        self.ws_data.get_canceled_order(order_id)
    }
}

fn fill_price_snaps(ws_data: DerivsWsData) {
    thread::spawn(move || loop {
        match ws_data.get_trading_pair() {
            Some(trading_pair) => {
                ws_data.add_price_snap(trading_pair.clone());
                debug!("Added price snap {:?}", trading_pair);
                thread::sleep(Duration::from_millis(5000));
            }
            None => {
                warn!("Unable to add price snap")
            }
        }
    });
}

struct WebSocketHandler {
    ws_data: DerivsWsData,
}

impl EventHandler for WebSocketHandler {
    fn on_connect(&mut self, event: NotificationEvent) {
        if let NotificationEvent::Info(info) = event {
            debug!(
                "Platform status: {:?}, Version {}",
                info.platform, info.version
            );
        }
    }

    fn on_auth(&mut self, _event: NotificationEvent) {}

    fn on_subscribed(&mut self, _event: NotificationEvent) {}

    fn on_data_event(&mut self, event: DataEvent) {
        match event {
            DataEvent::TickerTradingEvent(_, trading_pair) => {
                debug!("Ticker event => {:?}", trading_pair);
                self.ws_data.update_trading_pair(trading_pair);
            }
            DataEvent::TickerFundingEvent(_, _) => {}
            DataEvent::TradesTradingSnapshotEvent(_, trades) => {
                debug!("Trades snapshot event => {:?}", trades);
                for t in &trades {
                    self.ws_data.add_trade(t.clone());
                }
            }
            DataEvent::TradesTradingUpdateEvent(_, _, trade) => {
                debug!("Trades update event => {:?}", trade);
                self.ws_data.add_trade(trade);
            }
            DataEvent::TradesFundingSnapshotEvent(_, _) => {}
            DataEvent::TradesFundingUpdateEvent(_, _, _) => {}
            DataEvent::BookTradingSnapshotEvent(_, _) => {}
            DataEvent::BookTradingUpdateEvent(_, _) => {}
            DataEvent::BookFundingSnapshotEvent(_, _) => {}
            DataEvent::BookFundingUpdateEvent(_, _) => {}
            DataEvent::RawBookEvent(_, _) => {}
            DataEvent::RawBookUpdateEvent(_, _) => {}
            DataEvent::CandlesSnapshotEvent(_, candles) => {
                debug!("Candles snapshot event => {:?}", candles);
                for c in &candles {
                    self.ws_data.add_candle(c.clone());
                }
            }
            DataEvent::CandlesUpdateEvent(_, candle) => {
                debug!("Candles update event => {:?}", candle);
                self.ws_data.add_candle(candle);
            }
            DataEvent::HeartbeatEvent(_, _) => {}
            DataEvent::OrdersSnapshotEvent(_, _, orders) => {
                debug!("Orders snapshot event => {:?}", orders);
                for o in &orders {
                    self.ws_data.add_order(o.clone());
                }
            }
            DataEvent::OrdersUpdateEvent(_, _, order) => {
                debug!("Orders update event => {:?}", order);
                self.ws_data.add_order(order);
            }
            DataEvent::PositionsSnapshotEvent(_, _, positions) => {
                debug!("Positions snapshot event => {:?}", positions);
                for p in &positions {
                    self.ws_data.add_position(p.clone());
                }
            }
            DataEvent::PositionsUpdateEvent(_, _, position) => {
                debug!("Positions update event => {:?}", position);
                self.ws_data.add_position(position);
            }
            DataEvent::TradesUpdateEvent(_, _, _) => {}
            DataEvent::WalletsSnapshotEvent(_, _, wallets) => {
                debug!("Wallets snapshot event => {:?}", wallets);
                for w in &wallets {
                    self.ws_data.add_wallet(w.clone());
                }
            }
            DataEvent::WalletsUpdateEvent(_, _, wallet) => {
                debug!("Wallets update event => {:?}", wallet);
                self.ws_data.add_wallet(wallet);
            }
            DataEvent::BalanceInfoUpdateEvent(_, _, balance) => {
                debug!("Balance info update event => {:?}", balance);
                self.ws_data.update_balance(balance);
            }
            DataEvent::Other(_) => {}
        }
    }

    fn on_error(&mut self, message: Error) {
        error!("{:?}", message);
    }
}
