extern crate bitfinex;

use bitfinex::commons::errors::WebSocketError;
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

    fn on_auth(&mut self, event: NotificationEvent) {
        if let NotificationEvent::Auth(auth) = event {
            println!("Auth {}: {:?}", auth.status, auth.msg);
        }
    }

    fn on_subscribed(&mut self, _event: NotificationEvent) {}

    fn on_data_event(&mut self, event: DataEvent) {
        if let DataEvent::OrdersSnapshotEvent(channel, _type, orders) = event {
            for o in &orders {
                println!(
                    "Order snapshot ({}) - Symbol {}, id {}",
                    channel, o.symbol, o.id
                );
            }
        } else if let DataEvent::OrdersUpdateEvent(channel, _type, order) = event {
            println!(
                "Order update ({}) - Symbol {}, amount {}, price {}, status {}",
                channel, order.symbol, order.amount, order.price, order.order_status
            );
        } else if let DataEvent::PositionsSnapshotEvent(channel, _type, positions) = event {
            for p in &positions {
                println!(
                    "Position snapshot ({}) - Symbol {}, amount {}, price {}",
                    channel, p.symbol, p.amount, p.base_price
                );
            }
        } else if let DataEvent::PositionsUpdateEvent(channel, _type, position) = event {
            println!(
                "Position update ({}) - Symbol {}, amount {}, price {}",
                channel, position.symbol, position.amount, position.base_price
            );
        } else if let DataEvent::PositionsSnapshotEvent(channel, _type, positions) = event {
            for p in &positions {
                println!(
                    "Position snapshot ({}) - Symbol {}, amount {}, price {}",
                    channel, p.symbol, p.amount, p.base_price
                );
            }
        } else if let DataEvent::TradesUpdateEvent(channel, _type, trade) = event {
            println!(
                "Trade update ({}) - Symbol {}, amount {}, price {}",
                channel, trade.symbol, trade.execution_amount, trade.execution_price
            );
        } else if let DataEvent::WalletsSnapshotEvent(channel, _type, wallets) = event {
            for w in &wallets {
                println!(
                    "Wallet snapshot ({}) - Currency {}, balance {}",
                    channel, w.currency, w.balance
                );
            }
        } else if let DataEvent::WalletsUpdateEvent(channel, _type, wallet) = event {
            println!(
                "Wallet update ({}) - Currency {}, balance {}",
                channel, wallet.currency, wallet.balance
            );
        } else if let DataEvent::BalanceInfoUpdateEvent(channel, _type, balance) = event {
            println!(
                "Balance update ({}) - Aum {}, Aum net {}",
                channel, balance.aum, balance.aum_net
            );
        }
    }

    fn on_error(&mut self, message: WebSocketError) {
        println!("{:?}", message);
    }
}

fn main() {
    let api_key = "5QytTTlYGhLHzo1nT17O2baW3A12DBaPzydzu3aWvEy";
    let secret_key = "LYrjDqa7TOvxDjlViaku3Ux6Ci7j7qfrAV1lp8vo9DZ";
    let mut web_socket: WebSockets = WebSockets::default();

    web_socket.add_event_handler(WebSocketHandler);
    web_socket.connect().unwrap(); // check error

    web_socket.auth(api_key, secret_key, false, &[]).unwrap();

    web_socket.event_loop().unwrap(); // check error
}
