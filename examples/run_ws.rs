use binance::config::Config;
use binance::ws_usdt_futures::WsInterface;
use dotenv::dotenv;
use std::thread;

fn main() {
    dotenv().ok();
    env_logger::init();
    let api_key_user =
        Some("f7349ef10fed52e0282e9c66d7269acfb046d70d8b48f0ca34733e67322471c9".into());
    let api_secret_user =
        Some("7dedd32206a93e7d86f84372940a74e762711cd0800833a1e5fe56e6ed059cc1".into());
    let config = Config::testnet();
    let _ws = WsInterface::new(
        "BTCUSDT".to_string(),
        api_key_user,
        api_secret_user,
        config.to_owned(),
    );
    loop {
        thread::yield_now();
    }
}
