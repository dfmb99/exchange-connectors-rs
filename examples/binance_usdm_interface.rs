use binance::commons::config::Config;
use binance::interfaces::usdm::UsdmInterface;
use binance::interfaces::usdm_data::UsdmConfig;

fn main() {
    let binance = UsdmInterface::new(
        "BTCUSDT".to_owned(),
        None,
        None,
        &Config::default(),
        UsdmConfig::default(),
    );

    let result = binance.get_mark_price("BTCUSDT");
    match result {
        Ok(mark_price) => println!("Mark price: {}", mark_price.mark_price),
        Err(e) => println!("Error: {e}"),
    }
}
