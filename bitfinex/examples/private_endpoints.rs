extern crate bitfinex;

use std::time::SystemTime;
use bitfinex::rest::account::{AvailableBalanceParams, MovementParams, TransferWalletParams};
use bitfinex::rest::api::*;
use bitfinex::commons::pairs::*;
use bitfinex::commons::currency::*;
use bitfinex::rest::derivs::{DerivsPosCollaterallLimitsParams, DerivsPosCollaterallParams};
use bitfinex::rest::funding::{CancelAllOffersParams, CancelOfferParams, SubmitOfferParams};
use bitfinex::rest::orders::{
    OrderCancelParams, OrderMultiCancelParams, OrderSubmitParams, OrderType, OrderUpdateParams,
    TradeParams,
};

fn main() {
    // TESTNET
    let api_key = Some("5QytTTlYGhLHzo1nT17O2baW3A12DBaPzydzu3aWvEy".into());
    let secret_key = Some("LYrjDqa7TOvxDjlViaku3Ux6Ci7j7qfrAV1lp8vo9DZ".into());
    let api = Bitfinex::new(api_key, secret_key);

    // ORDERS
    let params = OrderSubmitParams {
        order_type: OrderType::ExchangeLimit.to_string(),
        symbol: TESTBTCUSDT.to_string(),
        price: Some("5000.0".into()),
        amount: "0.0001".into(),
        ..Default::default()
    };

    #[allow(unused_assignments)]
    let mut id = 0;

    match api.orders.submit_order(&params) {
        Ok(order) => {
            id = order.order_data[0].id;
            println!(
                "Order submitted => Symbol: {:?} amount: {:?} price: {:?}",
                order.order_data[0].symbol, order.order_data[0].amount, order.order_data[0].price
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    let params = OrderUpdateParams {
        id,
        amount: "0.0002".to_string(),
        price: "6000.0".to_string(),
        ..Default::default()
    };

    match api.orders.update_order(&params) {
        Ok(order) => {
            println!(
                "Order updated => Symbol: {:?} amount: {:?} price: {:?}",
                order.order_data.symbol, order.order_data.amount, order.order_data.price
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.orders.active_orders() {
        Ok(orders) => {
            for order in &orders {
                println!(
                    "Active orders => Symbol: {:?} amount: {:?} price: {:?}",
                    order.symbol, order.amount, order.price
                );
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    let params = OrderCancelParams {
        id: Some(id),
        ..Default::default()
    };

    match api.orders.cancel_order(&params) {
        Ok(order) => {
            println!("Order canceled => Id: {:?}", order.order_data.id);
        }
        Err(e) => panic!("Error: {}", e),
    }

    let params = OrderSubmitParams {
        order_type: OrderType::ExchangeLimit.to_string(),
        symbol: TESTBTCUSDT.to_string(),
        price: Some("5000.0".into()),
        amount: "0.0001".into(),
        ..Default::default()
    };

    match api.orders.submit_order(&params) {
        Ok(order) => {
            id = order.order_data[0].id;
            println!(
                "Order submitted => Symbol: {:?} amount: {:?} price: {:?}",
                order.order_data[0].symbol, order.order_data[0].amount, order.order_data[0].price
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    let params = OrderMultiCancelParams {
        id: Some(vec![id]),
        ..Default::default()
    };

    match api.orders.cancel_multi_orders(&params) {
        Ok(order) => {
            println!("Order canceled => Id: {:?}", order.order_data[0].id);
        }
        Err(e) => panic!("Error: {}", e),
    }

    let order_history = api.orders.history(BTCUSD.to_owned()); // Use None if you don't want a pair
    match order_history {
        Ok(orders) => {
            for order in &orders {
                println!(
                    "Order History => Symbol: {:?} amount: {:?} price: {:?}",
                    order.symbol, order.amount, order.price
                );
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    // WALLET
    match api.account.get_wallets() {
        Ok(wallets) => {
            for wallet in &wallets {
                println!(
                    "Wallet => Currency: {:?} Balance: {:?}",
                    wallet.currency, wallet.balance
                );
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.account.get_active_positions() {
        Ok(positions) => {
            for position in &positions {
                println!(
                    "Position => Symbol: {:?} Amount: {:?} Price liq: {:?}",
                    position.symbol, position.amount, position.price_liq
                );
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.account.margin_symbol(ETHUSD) {
        Ok(info) => {
            println!(
                "Margin Symbol Info => Gross Balance: {:?}",
                info.margin.gross_balance
            );
        }
        Err(e) => println!("Error: {}", e),
    }

    // FUNDING INFO
    match api.account.funding_info(USD) {
        Ok(info) => {
            println!(
                "Funding Info => Yield Loan: {:?} Yield Lend: {:?}",
                info.funding.yield_loan, info.funding.yield_lend
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    // LEDGER
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    match api.ledger.get_history(USD, now - 3600000, now, 5) {
        Ok(entries) => {
            for entry in &entries {
                println!(
                    "Ledger Entry => {}{} => {}: {}",
                    entry.amount, entry.currency, entry.balance, entry.description
                );
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    let params = OrderSubmitParams {
        order_type: OrderType::Market.to_string(),
        symbol: TESTBTCPERP.to_string(),
        amount: "0.0001".into(),
        ..Default::default()
    };

    match api.orders.submit_order(&params) {
        Ok(order) => {
            println!(
                "Order submitted => Symbol: {:?} amount: {:?} price: {:?}",
                order.order_data[0].symbol, order.order_data[0].amount, order.order_data[0].price
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    // DERIVS
    match api
        .derivs
        .derivs_pos_collateral_limits(&DerivsPosCollaterallLimitsParams {
            symbol: TESTBTCPERP.to_string(),
        }) {
        Ok(limits) => {
            println!(
                "Derivs pos collaterall limits => min {:?} max {:?}",
                limits.min_collateral, limits.max_collateral
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api
        .derivs
        .derivs_pos_collateral(&DerivsPosCollaterallParams {
            symbol: TESTBTCPERP.to_string(),
            collateral: 100.0,
        }) {
        Ok(status) => {
            for s in &status {
                println!("Derivs pos collaterall => {}", s.status);
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    let params = OrderSubmitParams {
        order_type: OrderType::Market.to_string(),
        symbol: TESTBTCPERP.to_string(),
        amount: "-0.0001".into(),
        ..Default::default()
    };

    match api.orders.submit_order(&params) {
        Ok(order) => {
            println!(
                "Order submitted => Symbol: {:?} amount: {:?} price: {:?}",
                order.order_data[0].symbol, order.order_data[0].amount, order.order_data[0].price
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.orders.trades(
        BTCPERP,
        &TradeParams {
            limit: Some(5),
            sort: Some(-1),
            ..Default::default()
        },
    ) {
        Ok(trade) => {
            for t in &trade {
                println!(
                    "Trades => pair {} exec amount {} exec price {} fee {}",
                    t.pair, t.exec_amount, t.exec_price, t.fee
                );
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    // FUNDING
    #[allow(unused_assignments)]
    let mut id = 0;

    match api.funding.submit_offer(&SubmitOfferParams {
        offer_type: "LIMIT".to_string(),
        symbol: FTESTUSDT.to_string(),
        amount: "200".to_string(),
        rate: "0.01".to_string(),
        period: 2,
        flags: None,
    }) {
        Ok(offer) => {
            id = offer.funding_offer_data.id;
            println!(
                "Submit funding offer => symbol {} amount {} rate {} period {}",
                offer.funding_offer_data.symbol,
                offer.funding_offer_data.amount,
                offer.funding_offer_data.rate,
                offer.funding_offer_data.period
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.funding.active_offers(None) {
        Ok(offers) => {
            for o in &offers {
                println!(
                    "Active funding offers => symbol {} amount {} rate {} period {}",
                    o.symbol, o.amount, o.rate, o.period
                );
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.funding.cancel_offer(&CancelOfferParams { id }) {
        Ok(offer) => {
            println!("Cancel funding offer => id {}", offer.funding_offer_data.id);
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.funding.submit_offer(&SubmitOfferParams {
        offer_type: "LIMIT".to_string(),
        symbol: FTESTUSDT.to_string(),
        amount: "200".to_string(),
        rate: "0.01".to_string(),
        period: 2,
        flags: None,
    }) {
        Ok(offer) => {
            println!(
                "Submit funding offer => symbol {} amount {} rate {} period {}",
                offer.funding_offer_data.symbol,
                offer.funding_offer_data.amount,
                offer.funding_offer_data.rate,
                offer.funding_offer_data.period
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.funding.cancel_all_offers(&CancelAllOffersParams {
        currency: Some(FTESTUSDT.to_string()),
    }) {
        Ok(offer) => {
            println!("Cancel all funding offers => status {}", offer.status);
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.account.transfer_between_wallets(&TransferWalletParams {
        from: "exchange".to_string(),
        to: "margin".to_string(),
        currency: TESTUSDT.to_string(),
        currency_to: None,
        amount: "100".to_string(),
        email_dst: None,
    }) {
        Ok(response) => {
            println!("Transfer between wallets => status {}", response.status);
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.account.movements(TESTUSDT, &MovementParams::default()) {
        Ok(movements) => {
            for m in &movements {
                println!(
                    "Movements =>  currency {} amount {} fees {}",
                    m.currency, m.amount, m.fees
                );
            }
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.account.available_balance(&AvailableBalanceParams {
        symbol: TESTBTCUSDT.to_string(),
        order_type: "EXCHANGE".to_string(),
        dir: Some(1),
        rate: Some("5000".into()),
        ..Default::default()
    }) {
        Ok(balance) => {
            println!(
                "Available balances => amount available {}",
                balance.amount_available
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.account.fee_summary() {
        Ok(fee_summary) => {
            println!(
                "Fee summary => deriv rebate {} deriv taker fee {}",
                fee_summary.data.0.deriv_rebate, fee_summary.data.1.deriv_taker_fee
            );
        }
        Err(e) => panic!("Error: {}", e),
    }

    match api.derivs.list_derivs_pairs() {
        Ok(pairs) => {
            for p in &pairs {
                println!("List derivs pairs => symbol {}", p);
            }
        }
        Err(e) => panic!("Error: {}", e),
    }
}
