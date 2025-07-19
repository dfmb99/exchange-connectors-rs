extern crate bitfinex;

use bitfinex::commons::currency::*;
use bitfinex::commons::pairs::*;
use bitfinex::commons::precision::*;
use bitfinex::rest::api::*;
use bitfinex::rest::candles::*;
use bitfinex::rest::derivs::*;
use bitfinex::rest::ticker::*;

fn main() {
    let api = Bitfinex::new(None, None);

    // TICKER
    let trading_pair = api.ticker.trading_pair(ETHUSD);
    match trading_pair {
        Ok(answer) => println!(
            "Trading pair => bid: {:?}  ask: {:?}",
            answer.bid, answer.ask
        ),
        Err(e) => panic!("Error: {e}"),
    }

    let funding_currency = api.ticker.funding_currency(USD);
    match funding_currency {
        Ok(answer) => println!(
            "Funding currency => bid: {:?}  ask: {:?}",
            answer.bid, answer.ask
        ),
        Err(e) => panic!("Error: {e}"),
    }

    let funding_stats = api.ticker.funding_stats(
        USD,
        &FundingStatusParams {
            limit: Some(5),
            start: None,
            end: None,
        },
    );
    match funding_stats {
        Ok(funding_stats) => {
            for stats in &funding_stats {
                println!(
                    "Funding Stats => flash return rate: {:?} amount: {:?}",
                    stats.frr, stats.funding_amount
                );
            }
        }
        Err(e) => panic!("Error: {e}"),
    }

    // TRADES
    let trading_pairs = api.trades.trading_pair(ETHUSD);
    match trading_pairs {
        Ok(trades) => {
            for trade in &trades {
                println!(
                    "Trading => amount: {:?}  price: {:?}",
                    trade.amount, trade.price
                );
            }
        }
        Err(e) => panic!("Error: {e}"),
    }

    let funding_currency = api.trades.funding_currency(USD);
    match funding_currency {
        Ok(trades) => {
            for trade in &trades {
                println!(
                    "Funding => amount: {:?}  price: {:?}",
                    trade.amount, trade.price
                );
            }
        }
        Err(e) => panic!("Error: {e}"),
    }

    // BOOK
    let trading_pairs = api.book.trading_pair(ETHUSD, P0);
    match trading_pairs {
        Ok(books) => {
            for book in &books {
                println!(
                    "Trading => price: {:?} amount: {:?}",
                    book.price, book.amount
                );
            }
        }
        Err(e) => panic!("Error: {e}"),
    }

    let funding_currency = api.book.funding_currency(USD, P0);
    match funding_currency {
        Ok(books) => {
            for book in &books {
                println!("Funding => rate: {:?} amount: {:?}", book.rate, book.amount);
            }
        }
        Err(e) => panic!("Error: {e}"),
    }

    // CANDLES
    let last = api.candles.last(ETHUSD, "1m");
    match last {
        Ok(answer) => println!(
            "Candle Last => High: {:?} low: {:?}",
            answer.high, answer.low
        ),
        Err(e) => panic!("Error: {e}"),
    }

    let history = api
        .candles
        .history(ETHUSD, "12h", &CandleHistoryParams::new());
    match history {
        Ok(candles) => {
            for candle in &candles {
                println!(
                    "Candle History => High: {:?} Low: {:?}",
                    candle.high, candle.low
                );
            }
        }
        Err(e) => panic!("Error: {e}"),
    }

    // DERIVS
    let derivs_status = api.derivs.derivs_status(vec![BTCPERP, ETHPERP, DOGEPERP]);
    match derivs_status {
        Ok(derivs) => {
            for deriv in &derivs {
                println!(
                    "Derivs Status => Key: {:?} Mark price: {:?} Funding rate: {:?}",
                    deriv.key, deriv.mark_price, deriv.current_funding
                );
            }
        }
        Err(e) => panic!("Error: {e}"),
    }

    let derivs_status = api.derivs.derivs_status(vec!["ALL"]);
    match derivs_status {
        Ok(derivs) => {
            for deriv in &derivs {
                println!(
                    "Derivs Status => Key: {:?} Mark price: {:?} Funding rate: {:?}",
                    deriv.key, deriv.mark_price, deriv.current_funding
                );
            }
        }
        Err(e) => panic!("Error: {e}"),
    }

    let derivs_hist = api.derivs.derivs_status_hist(
        BTCPERP,
        &DerivStatusHistParams {
            start: Some("157057800000".into()),
            end: Some("1573566992000".into()),
            sort: Some(-1),
            limit: Some("5".into()),
        },
    );

    match derivs_hist {
        Ok(derivs) => {
            for deriv in &derivs {
                println!(
                    "Derivs Status Hist => Symbol: {:?} Mark price: {:?} Funding rate: {:?}",
                    BTCPERP, deriv.mark_price, deriv.current_funding
                );
            }
        }
        Err(e) => panic!("Error: {e}"),
    }
}
