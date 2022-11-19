use crate::ws_fut_data::WsData;
use binance::api::Binance;
use binance::config::Config;
use binance::futures::userstream::FuturesUserStream;
use binance::futures::websockets::{FuturesMarket, FuturesWebSockets, FuturesWebsocketEvent};
use binance::model::{EventBalance, EventPosition};
use log::{debug, error, info, trace, warn};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub struct WsInterface {
    symbol: String,
    config: Config,
    ws_data: WsData,
}

impl WsInterface {
    pub fn new(
        symbol: String,
        api_key: Option<String>,
        api_secret: Option<String>,
        config: Config,
    ) -> WsInterface {
        let ws_data = WsData::default();
        user_stream_websocket(
            symbol.to_owned(),
            api_key,
            api_secret,
            config.to_owned(),
            ws_data.clone(),
        );
        market_websocket(symbol.to_owned(), config.to_owned(), ws_data.clone());
        let ws_int = WsInterface {
            symbol,
            config,
            ws_data,
        };
        ws_int.wait_for_data();
        ws_int
    }

    fn wait_for_data(&self) {
        while self.ws_data.get_mark_price().is_none() {
            debug!("Waiting for data");
            thread::yield_now();
        }
    }
}

fn user_stream_websocket(
    symbol: String,
    api_key: Option<String>,
    api_secret: Option<String>,
    config: Config,
    ws_data: WsData,
) {
    // TODO: deal with UserDataStreamExpiredEvent events
    thread::spawn(move || {
        loop {
            let user_stream: FuturesUserStream =
                Binance::new_with_config(api_key.to_owned(), api_secret.to_owned(), &config);
            let keep_running = AtomicBool::new(true); // Used to control the event loop

            if let Ok(answer) = user_stream.start() {
                let listen_key = answer.listen_key;
                let (tx, rx) = mpsc::channel();

                // luanches thread to keep alive user streamn
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
                                    .filter(|event| event.symbol == symbol)
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
                                ws_data.add_order(trade.order);
                            }
                            _ => {
                                debug!("Received unhandled event : {:?}", event)
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
                user_stream.close(&listen_key);
                web_socket.disconnect();
                let _ = tx.send(());
                warn!("User stream closed and disconnected");
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

            match rx.recv_timeout(Duration::from_millis(300)) {
                Ok(_) => {
                    debug!("Terminating.");
                    break;
                }
                Err(_) => {}
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
                            debug!("Received unhandled event : {:?}", event)
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
            web_socket.disconnect();
            debug!("Market websocket disconnected");
        }
    });
}
