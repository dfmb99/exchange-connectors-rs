use std::sync::atomic::AtomicBool;
use binance::api::*;
use binance::config::Config;
use binance::futures::userstream::*;
use binance::futures::websockets::{FuturesMarket, FuturesWebsocketEvent, FuturesWebSockets};

fn main() {
    //user_stream();
    user_stream_websocket();
}

fn user_stream() {
    let api_key_user = Some("YOUR_API_KEY".into());
    let user_stream: FuturesUserStream = Binance::new(api_key_user, None);

    if let Ok(answer) = user_stream.start() {
        println!("Data Stream Started ...");
        let listen_key = answer.listen_key;

        match user_stream.keep_alive(&listen_key) {
            Ok(msg) => println!("Keepalive user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }

        match user_stream.close(&listen_key) {
            Ok(msg) => println!("Close user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

fn user_stream_websocket() {
    let api_key_user =
        Some("f7349ef10fed52e0282e9c66d7269acfb046d70d8b48f0ca34733e67322471c9".into());
    let api_secret_user =
        Some("7dedd32206a93e7d86f84372940a74e762711cd0800833a1e5fe56e6ed059cc1".into());
    let config = Config::testnet();

    let user_stream: FuturesUserStream =
        Binance::new_with_config(api_key_user, api_secret_user, &config);

    let keep_running = AtomicBool::new(true); // Used to control the event loop

    if let Ok(answer) = user_stream.start() {
        let listen_key = answer.listen_key;

        let mut web_socket: FuturesWebSockets<'_> =
            FuturesWebSockets::new(|event: FuturesWebsocketEvent| {
                match event {
                    FuturesWebsocketEvent::AccountUpdate(account_update) => {
                        for balance in &account_update.data.balances {
                            println!("{:?}", balance);
                        }
                        for positions in &account_update.data.positions {
                            println!("{:?}", positions);
                        }
                    }
                    FuturesWebsocketEvent::OrderTrade(trade) => {
                        println!("{:?}", trade);
                    }
                    _ => (),
                };

                Ok(())
            });

        web_socket
            .connect_with_config(FuturesMarket::USDM, &listen_key, &config)
            .unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            println!("Error: {}", e);
        }
        user_stream.close(&listen_key).unwrap();
        web_socket.disconnect().unwrap();
        println!("User stream closed and disconnected");
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}
