use std::collections::VecDeque;
use log::{debug, error, warn};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use crate::commons::config::Config;
use crate::commons::errors::{BinanceContentError, Error};
use crate::commons::errors::ErrorKind::BinanceError;
use crate::rest::api::Binance;
use crate::rest::futures::model::OrderUpdate;
use crate::rest::model::{
    AggrTradesEvent, EventBalance, EventPosition, IndexPriceEvent, LiquidationOrder,
};
use crate::websocket::futures::{FuturesMarket, FuturesWebsocketEvent, FuturesWebSockets};
use crate::websocket::futures::usdm_data::WsData;
use crate::websocket::futures::userstream::FuturesUserStream;

#[derive(Clone)]
pub struct WsInterface {
    ws_data: WsData,
}

impl WsInterface {
    /// Binance USDM futures interface,
    /// subscribes to @aggTrade, @markPrice@1s and @forceOrder and user data stream
    /// * `symbol` - String
    /// * `api_key` - Option<String>
    /// * `api_secret` - Option<String>
    /// * `config` - Config
    pub fn new(
        symbol: String, api_key: Option<String>, api_secret: Option<String>, config: &Config,
    ) -> WsInterface {
        let ws_data = WsData::default();
        user_stream_websocket(
            symbol.to_lowercase(),
            api_key,
            api_secret,
            config.to_owned(),
            ws_data.clone(),
        );
        market_websocket(symbol, config.to_owned(), ws_data.clone());
        let ws_int = WsInterface {
            ws_data: ws_data.clone(),
        };
        ws_int.wait_for_data();
        fill_mark_price_snaps(ws_data);
        ws_int
    }

    fn wait_for_data(&self) {
        debug!("Waiting for data");
        while self.ws_data.get_mark_price_event().is_none() {
            thread::yield_now();
        }
        debug!("Finished waiting for data");
    }

    /// Get mark price
    pub fn get_mark_price(&self) -> Option<IndexPriceEvent> {
        self.ws_data.get_mark_price_event()
    }

    /// Get mark price snaps
    pub fn get_mark_price_snaps(&self) -> VecDeque<IndexPriceEvent> {
        self.ws_data.get_mark_price_event_snaps()
    }

    /// Get aggr_trades
    pub fn get_aggr_trades(&self) -> VecDeque<AggrTradesEvent> {
        self.ws_data.get_aggr_trades()
    }

    /// Get liquidations
    pub fn get_liquidations(&self) -> VecDeque<LiquidationOrder> {
        self.ws_data.get_liquidations()
    }

    /// Get position
    pub fn get_position(&self) -> Option<EventPosition> {
        self.ws_data.get_position_event()
    }

    /// Get balance
    pub fn get_balance(&self) -> Option<EventBalance> {
        self.ws_data.get_balance_event()
    }

    /// Get open orders
    pub fn get_open_orders(&self) -> VecDeque<OrderUpdate> {
        self.ws_data.get_open_orders()
    }

    /// Get filled orders
    pub fn get_filled_orders(&self) -> VecDeque<OrderUpdate> {
        self.ws_data.get_filled_orders()
    }

    /// Get canceled orders
    pub fn get_canceled_orders(&self) -> VecDeque<OrderUpdate> {
        self.ws_data.get_canceled_orders()
    }

    /// Get order
    ///
    /// * `order_id` - id of order
    pub fn get_order(&self, order_id: u64) -> Option<OrderUpdate> {
        if let Some(order) = self.ws_data.get_open_order(order_id) {
            return Some(order);
        } else if let Some(order) = self.ws_data.get_filled_order(order_id) {
            return Some(order);
        } else if let Some(order) = self.ws_data.get_canceled_order(order_id) {
            return Some(order);
        }
        None
    }
}

fn user_stream_websocket(
    symbol: String, api_key: Option<String>, api_secret: Option<String>, config: Config,
    ws_data: WsData,
) {
    thread::spawn(move || {
        loop {
            let user_stream: FuturesUserStream =
                Binance::new_with_config(api_key.to_owned(), api_secret.to_owned(), &config);
            let keep_running = AtomicBool::new(true); // Used to control the event loop

            if let Ok(answer) = user_stream.start() {
                let listen_key = answer.listen_key;
                let (tx, rx) = mpsc::channel();

                // launches thread to keep alive user streamn
                user_stream_keep_alive(rx, user_stream.to_owned(), listen_key.to_owned());

                let mut web_socket: FuturesWebSockets<'_> =
                    FuturesWebSockets::new(|event: FuturesWebsocketEvent| {
                        match event {
                            FuturesWebsocketEvent::AccountUpdate(account_update) => {
                                debug!("Received AccountUpdateEvent : {:?}", account_update);
                                let positions: Vec<EventPosition> = account_update
                                    .data
                                    .positions
                                    .into_iter()
                                    .filter(|event| event.symbol.to_lowercase() == symbol)
                                    .collect();
                                if !positions.is_empty() {
                                    ws_data.update_position(positions.get(0).unwrap().to_owned())
                                }

                                let balances: Vec<EventBalance> = account_update
                                    .data
                                    .balances
                                    .into_iter()
                                    .filter(|event| event.asset == "USDT")
                                    .collect();
                                if !balances.is_empty() {
                                    ws_data.update_balance(balances.get(0).unwrap().to_owned())
                                }
                            }
                            FuturesWebsocketEvent::OrderTrade(trade) => {
                                debug!("Received OrderTradeEvent : {:?}", trade);
                                ws_data.add_order(trade.order);
                            }
                            FuturesWebsocketEvent::UserDataStreamExpiredEvent(
                                user_stream_expired,
                            ) => {
                                debug!(
                                    "Received UserDataStreamExpiredEvent : {:?}",
                                    user_stream_expired
                                );
                                let err = BinanceContentError {
                                    code: -32768,
                                    msg: "User data listen key is expired".to_string(),
                                };
                                return Err(Error(BinanceError(err), Default::default()));
                            }
                            _ => {
                                warn!("Received unhandled event : {:?}", event)
                            }
                        };

                        Ok(())
                    });

                web_socket
                    .connect_with_config(FuturesMarket::USDM, &listen_key, &config)
                    .unwrap(); // check error
                if let Err(e) = web_socket.event_loop(&keep_running) {
                    error!("Error: {}", e);
                }
                if let Err(e) = user_stream.close(&listen_key) {
                    error!("Error closing user stream: {}", e);
                }
                if let Err(e) = web_socket.disconnect() {
                    error!("Error disconnecting from websocket: {}", e);
                }
                let _ = tx.send(());
                debug!("User stream closed and disconnected");
            } else {
                panic!("Not able to start an User Stream (Check your API_KEY)");
            }
        }
    });
}

fn user_stream_keep_alive(rx: Receiver<()>, user_stream: FuturesUserStream, listen_key: String) {
    thread::spawn(move || {
        loop {
            // Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes. Loops every 50 minutes
            thread::sleep(Duration::from_secs(3000));

            if rx.recv_timeout(Duration::from_millis(300)).is_ok() {
                debug!("Terminating.");
                break;
            }

            match user_stream.keep_alive(&listen_key) {
                Ok(msg) => debug!("Keepalive user data stream: {:?}", msg),
                Err(e) => warn!("Error: {}", e),
            }
        }
    });
}

fn market_websocket(symbol: String, config: Config, ws_data: WsData) {
    thread::spawn(move || {
        loop {
            let keep_running = AtomicBool::new(true);
            let streams = vec![
                // taken from https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
                symbol.to_owned().to_lowercase() + "@aggTrade",
                symbol.to_owned().to_lowercase() + "@markPrice@1s",
                symbol.to_owned().to_lowercase() + "@forceOrder",
            ];

            let mut web_socket: FuturesWebSockets<'_> =
                FuturesWebSockets::new(|event: FuturesWebsocketEvent| {
                    match event {
                        FuturesWebsocketEvent::AggrTrades(trade) => {
                            debug!("Received AggrTradesEvent : {:?}", trade);
                            ws_data.add_aggr_trades(trade);
                        }
                        FuturesWebsocketEvent::IndexPrice(mark_price) => {
                            debug!("Received IndexPrice : {:?}", mark_price);
                            ws_data.update_mark_price(mark_price);
                        }
                        FuturesWebsocketEvent::Liquidation(liquidation) => {
                            debug!("Received LiquidationEvent : {:?}", liquidation);
                            ws_data.add_liquidation(liquidation.liquidation_order);
                        }
                        _ => {
                            warn!("Received unhandled event : {:?}", event)
                        }
                    };

                    Ok(())
                });
            web_socket
                .connect_multiple_streams(FuturesMarket::USDM, &streams, &config)
                .unwrap(); // check error
            if let Err(e) = web_socket.event_loop(&keep_running) {
                error!("Error: {}", e);
            }
            if let Err(e) = web_socket.disconnect() {
                error!("Error disconnecting from websocket: {}", e);
            }
            debug!("Market websocket disconnected");
        }
    });
}

fn fill_mark_price_snaps(ws_data: WsData) {
    thread::spawn(move || loop {
        match ws_data.get_mark_price_event() {
            Some(index_price) => {
                ws_data.add_mark_price_snap(index_price.clone());
                debug!("Added mark price snap {:?}", index_price);
                thread::sleep(Duration::from_millis(5000));
            }
            None => {
                warn!("Unable to add mark price snap")
            }
        }
    });
}
