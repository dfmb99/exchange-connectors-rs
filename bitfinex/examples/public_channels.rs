extern crate bitfinex;

use bitfinex::commons::errors::WebSocketError;
use bitfinex::commons::{pairs::*, precision::*};
use bitfinex::websocket::events::*;
use bitfinex::websocket::websockets::*;

struct WebSocketHandler;

impl EventHandler for WebSocketHandler {
    fn on_connect(&mut self, event: NotificationEvent) {
        if let NotificationEvent::Info(info) = event {
            println!(
                "Platform status: {:?}, Version {}",
                info.platform, info.version
            );
        }
    }

    fn on_auth(&mut self, _event: NotificationEvent) {}

    fn on_subscribed(&mut self, event: NotificationEvent) {
        if let NotificationEvent::TradingSubscribed(msg) = event {
            println!("Subscribed: {msg:?}");
        } else if let NotificationEvent::CandlesSubscribed(msg) = event {
            println!("Subscribed: {msg:?}");
        } else if let NotificationEvent::RawBookSubscribed(msg) = event {
            println!("Subscribed: {msg:?}");
        }
    }

    fn on_data_event(&mut self, event: DataEvent) {
        if let DataEvent::TickerTradingEvent(channel, trading) = event {
            println!(
                "Ticker Trading ({}) - Bid {:?}, Ask: {}",
                channel, trading.bid, trading.ask
            );
        } else if let DataEvent::RawBookEvent(channel, raw_book) = event {
            println!(
                "Raw book ({}) - Price {:?}, Amount: {}",
                channel, raw_book.price, raw_book.amount
            );
        } else if let DataEvent::TradesTradingUpdateEvent(channel, _pair, trading) = event {
            println!(
                "Trade update ({}) - Id: {}, Time: {}, Price: {}, Amount: {}",
                channel, trading.id, trading.mts, trading.price, trading.amount
            );
        }
        // ... Add for all events you have subscribed (Trades, Books, ...)
    }

    fn on_error(&mut self, message: WebSocketError) {
        println!("{message:?}");
    }
}

fn main() {
    let mut web_socket: WebSockets = WebSockets::default();

    web_socket.add_event_handler(WebSocketHandler);
    web_socket.connect().unwrap(); // check error

    // TICKER
    let _ = web_socket.subscribe_ticker(BTCUSD);

    // TRADES
    let _ = web_socket.subscribe_trades(BTCUSD);

    // BOOKS
    let _ = web_socket.subscribe_books(BTCUSD, P0, "F0", 25);

    // CANDLES
    let _ = web_socket.subscribe_candles(BTCUSD, "1m");

    web_socket.event_loop().unwrap(); // check error
}
