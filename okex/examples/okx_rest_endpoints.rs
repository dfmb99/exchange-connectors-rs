use dotenv::dotenv;
use okx::commons::errors::*;
use okx::commons::utils::{Side, TradeMode};
use okx::rest::api::Okx;
use okx::rest::market_data::{MarketData, TickerParams};
use okx::rest::trade::{Trade, PlaceOrderParams};
use std::env::var;
use okx::commons::config::Config;

fn main() {
    market_data();
    //trade();
}

#[allow(dead_code)]
fn market_data() {
    let market_data: MarketData = Okx::new(None, None, None);

    match market_data.get_ticker(&TickerParams {
        inst_id: "BTC-USD-SWAP".to_string(),
    }) {
        Ok(answer) => println!("{:?}", answer.data),
        Err(e) => match e.0 {
            ErrorKind::OkxError(e) => println!("API error: {} {}", e.code, e.msg),
            _ => println!("Error: {:?}", e),
        },
    }
}

#[allow(dead_code)]
fn trade() {
    dotenv().ok();
    let _ = env_logger::try_init();
    let api_key = var("API_KEY").expect("API_KEY is not defined.");
    let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
    let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");
    let config = Config::testnet();
    let trade: Trade =
        Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

    match trade.place_order(&PlaceOrderParams {
        inst_id: "BTC-USDT".to_string(),
        td_mode: TradeMode::Cash,
        side: Side::Buy,
        sz: "10".into(),
        ..Default::default()
    }) {
        Ok(answer) => println!("{:?}", answer.data),
        Err(e) => match e.0 {
            ErrorKind::OkxError(e) => println!("API error: {} {}", e.code, e.msg),
            _ => println!("Error: {:?}", e),
        },
    }
}
