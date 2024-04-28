use binance::commons::config::Config;
use binance::commons::errors::ErrorKind as BinanceLibErrorKind;
use binance::rest::api::Binance;
use binance::rest::futures::account::FuturesAccount;
use binance::rest::futures::general::FuturesGeneral;
use binance::rest::futures::market::FuturesMarket;
use binance::rest::futures::model::{AggTrades, LiquidationOrders, MarkPrices, Trades};
use binance::rest::model::{BookTickers, KlineSummaries};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    general();
    account();
    market_data();
}

fn general() {
    let general: FuturesGeneral = Binance::new(None, None);

    match general.ping() {
        Ok(answer) => println!("{answer:?}"),
        Err(err) => {
            match err.0 {
                BinanceLibErrorKind::BinanceError(response) => match response.code {
                    -1000_i16 => println!("An unknown error occured while processing the request"),
                    _ => println!("Non-catched code {}: {}", response.code, response.msg),
                },
                BinanceLibErrorKind::Msg(msg) => println!("Binancelib error msg: {msg}"),
                _ => println!("Other errors: {}.", err.0),
            };
        }
    }

    match general.get_server_time() {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {e}"),
    }

    match general.exchange_info() {
        Ok(answer) => println!("Exchange information: {answer:?}"),
        Err(e) => println!("Error: {e}"),
    }

    match general.get_symbol_info("btcusdt") {
        Ok(answer) => println!("Symbol information: {answer:?}"),
        Err(e) => println!("Error: {e}"),
    }
}

fn account() {
    let api_key_user =
        Some("f7349ef10fed52e0282e9c66d7269acfb046d70d8b48f0ca34733e67322471c9".into());
    let api_secret_user =
        Some("7dedd32206a93e7d86f84372940a74e762711cd0800833a1e5fe56e6ed059cc1".into());

    let config = Config::testnet();
    let account: FuturesAccount = Binance::new_with_config(api_key_user, api_secret_user, &config);

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    match account.get_comission_rate("btcusdt", &since_the_epoch.as_millis().to_string()) {
        Ok(comission_rate) => println!(
            "Maker fee: {:?} Taker fee: {:?}",
            comission_rate.maker_commission_rate, comission_rate.taker_commission_rate
        ),
        Err(e) => println!("Error: {e}"),
    }
}

fn market_data() {
    let market: FuturesMarket = Binance::new(None, None);

    match market.get_depth("btcusdt") {
        Ok(answer) => println!("Depth update ID: {:?}", answer.last_update_id),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_trades("btcusdt") {
        Ok(Trades::AllTrades(answer)) => println!("First trade: {:?}", answer[0]),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_agg_trades("btcusdt", None, None, None, None) {
        Ok(AggTrades::AllAggTrades(answer)) => println!("First aggregated trade: {:?}", answer[0]),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_klines("btcusdt", "5m", 10, None, None) {
        Ok(KlineSummaries::AllKlineSummaries(answer)) => println!("First kline: {:?}", answer[0]),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_24h_price_stats("btcusdt") {
        Ok(answer) => println!("24hr price stats: {answer:?}"),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_price("btcusdt") {
        Ok(answer) => println!("Price: {answer:?}"),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_all_book_tickers() {
        Ok(BookTickers::AllBookTickers(answer)) => println!("First book ticker: {:?}", answer[0]),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_book_ticker("btcusdt") {
        Ok(answer) => println!("Book ticker: {answer:?}"),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_mark_prices() {
        Ok(MarkPrices::AllMarkPrices(answer)) => println!("First mark Prices: {:?}", answer[0]),
        Err(e) => println!("Error: {e}"),
    }

    match market.get_all_liquidation_orders() {
        Ok(LiquidationOrders::AllLiquidationOrders(answer)) => {
            println!("First liquidation order: {:?}", answer[0])
        }
        Err(e) => println!("Error: {e}"),
    }

    match market.open_interest("btcusdt") {
        Ok(answer) => println!("Open interest: {answer:?}"),
        Err(e) => println!("Error: {e}"),
    }

    match market.funding_rate_history(Some("btcusdt".to_string()), Some(3), None, None) {
        Ok(funding_rates) => {
            for f in &funding_rates {
                println!(
                    "Funding rate: symbol {:?} funding rate {:?}",
                    f.symbol, f.funding_rate
                );
            }
        }
        Err(e) => println!("Error: {e}"),
    }
}
