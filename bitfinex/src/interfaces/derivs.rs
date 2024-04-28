use crate::commons::currency::USTF0;
use crate::commons::errors::*;
use crate::rest::account::{
    AvailableBalance, AvailableBalanceParams, FeeSummary, Position, TransferWallet,
    TransferWalletParams, Wallet,
};
use crate::rest::api::Bitfinex;
use crate::rest::candles::{Candle, CandleHistoryParams};
use crate::rest::derivs::{
    DerivStatus, DerivStatusHist, DerivStatusHistParams, DerivsPosCollaterall,
    DerivsPosCollaterallLimits, DerivsPosCollaterallLimitsParams, DerivsPosCollaterallParams,
};
use crate::rest::orders::{
    OrderCancelParams, OrderData, OrderMultiCancelParams, OrderSubmitParams, OrderUpdate,
    OrdersUpdate, Trade, TradeParams,
};
use crate::rest::ticker::TradingPair;
use crate::rest::trades::TradingPair as TradesTradingPair;
use crate::websocket::derivs_ws::DerivsWs;
use crate::websocket::model::{BalanceInfo, Wallet as WalletWs};
use log::error;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Derivs {
    pub symbol: String,
    pub api: Bitfinex,
    pub ws: DerivsWs,
    data: DerivsData,
    client_id: i64,
}

impl Derivs {
    pub fn new(symbol: String, api_key: String, api_secret: String) -> Derivs {
        let api = Bitfinex::new(Some(api_key.to_owned()), Some(api_secret.to_owned()));
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let derivs_ws = DerivsWs::new(symbol.to_owned(), api_key, api_secret);
        let derivs = Derivs {
            symbol,
            api,
            ws: derivs_ws,
            data: DerivsData::default(),
            client_id: since_the_epoch.as_millis() as i64,
        };
        update_derivs_data(derivs.to_owned());
        derivs.wait_for_data();
        derivs
    }

    fn wait_for_data(&self) {
        loop {
            if !self.get_last_day_candles_data().is_empty() {
                break;
            }
            thread::yield_now();
        }
    }

    pub fn get_last_day_candles_data(&self) -> Vec<Candle> {
        self.data.get_last_day_candles()
    }

    pub fn get_trading_pair(&self) -> Result<TradingPair> {
        self.api.ticker.trading_pair(self.symbol.to_owned())
    }

    pub fn get_candles_1m_last_day(&self) -> Result<Vec<Candle>> {
        self.api.candles.history(
            &self.symbol,
            &String::from("1m"),
            &CandleHistoryParams {
                limit: Some(1440),
                start: None,
                end: None,
                sort: Some(false),
            },
        )
    }

    pub fn get_derivs_status(&self) -> Result<Vec<DerivStatus>> {
        self.api.derivs.derivs_status(vec![&self.symbol])
    }

    pub fn get_derivs_status_history(&self) -> Result<Vec<DerivStatusHist>> {
        self.api
            .derivs
            .derivs_status_hist(&self.symbol, &DerivStatusHistParams::default())
    }

    pub fn derivs_pos_collateral(&self, collateral: f64) -> Result<Vec<DerivsPosCollaterall>> {
        self.api
            .derivs
            .derivs_pos_collateral(&DerivsPosCollaterallParams {
                symbol: self.symbol.to_owned(),
                collateral,
            })
    }

    pub fn get_derivs_pos_collateral_limits(&self) -> Result<DerivsPosCollaterallLimits> {
        self.api
            .derivs
            .derivs_pos_collateral_limits(&DerivsPosCollaterallLimitsParams {
                symbol: self.symbol.to_owned(),
            })
    }

    pub fn get_active_orders(&self) -> Result<Vec<OrderData>> {
        let mut response = self.api.orders.active_orders();
        if let Ok(orders) = response {
            response = Ok(orders
                .into_iter()
                .filter(|data| data.symbol == self.symbol)
                .collect());
        }
        response
    }

    pub fn submit_market_order(&self, amount: f64) -> Result<OrdersUpdate> {
        self.api.orders.submit_order(&OrderSubmitParams {
            client_id: Some(self.client_id),
            order_type: "MARKET".to_string(),
            symbol: self.symbol.to_string(),
            amount: amount.to_string(),
            ..Default::default()
        })
    }

    pub fn submit_market_buy_order(&self, amount: f64) -> Result<OrdersUpdate> {
        self.api.orders.submit_order(&OrderSubmitParams {
            client_id: Some(self.client_id),
            order_type: "MARKET".to_string(),
            symbol: self.symbol.to_string(),
            amount: amount.to_string(),
            ..Default::default()
        })
    }

    pub fn submit_market_sell_order(&self, amount: f64) -> Result<OrdersUpdate> {
        self.api.orders.submit_order(&OrderSubmitParams {
            client_id: Some(self.client_id),
            order_type: "MARKET".to_string(),
            symbol: self.symbol.to_string(),
            amount: (-amount).to_string(),
            ..Default::default()
        })
    }

    pub fn submit_limit_buy_order(&self, amount: f64, price: f64) -> Result<OrdersUpdate> {
        self.api.orders.submit_order(&OrderSubmitParams {
            client_id: Some(self.client_id),
            order_type: "LIMIT".to_string(),
            symbol: self.symbol.to_string(),
            amount: amount.to_string(),
            price: Some(price.to_string()),
            ..Default::default()
        })
    }

    pub fn submit_limit_sell_order(&self, amount: f64, price: f64) -> Result<OrdersUpdate> {
        self.api.orders.submit_order(&OrderSubmitParams {
            client_id: Some(self.client_id),
            order_type: "LIMIT".to_string(),
            symbol: self.symbol.to_string(),
            amount: (-amount).to_string(),
            price: Some(price.to_string()),
            ..Default::default()
        })
    }

    pub fn get_active_order(&self) -> Result<Vec<OrderData>> {
        self.api.orders.active_orders()
    }

    pub fn cancel_order(&self, id: i64) -> Result<OrderUpdate> {
        self.api.orders.cancel_order(&OrderCancelParams {
            id: Some(id),
            ..Default::default()
        })
    }

    pub fn cancel_all_orders(&self, ids: Vec<i64>) -> Result<OrdersUpdate> {
        self.api
            .orders
            .cancel_multi_orders(&OrderMultiCancelParams {
                id: Some(ids),
                ..Default::default()
            })
    }

    pub fn get_order_history(&self) -> Result<Vec<OrderData>> {
        self.api.orders.history(self.symbol.to_owned())
    }

    pub fn get_trades(&self) -> Result<Vec<Trade>> {
        self.api
            .orders
            .trades(self.symbol.to_owned(), &TradeParams::default())
    }

    pub fn get_wallets(&self) -> Result<Vec<Wallet>> {
        self.api.account.get_wallets()
    }

    pub fn get_active_positions(&self) -> Result<Vec<Position>> {
        self.api.account.get_active_positions()
    }

    pub fn transfer_between_wallets(
        &self,
        from: String,
        to: String,
        amount: f64,
    ) -> Result<TransferWallet> {
        self.api
            .account
            .transfer_between_wallets(&TransferWalletParams {
                from,
                to,
                currency: self.symbol.to_string(),
                amount: amount.to_string(),
                ..Default::default()
            })
    }

    pub fn available_balance(&self) -> Result<AvailableBalance> {
        self.api.account.available_balance(&AvailableBalanceParams {
            symbol: self.symbol.to_string(),
            dir: None,
            rate: None,
            order_type: "DERIV".to_string(),
            lev: None,
        })
    }

    pub fn fee_summary(&self) -> Result<FeeSummary> {
        self.api.account.fee_summary()
    }

    pub fn list_derivs_pairs(&self) -> Result<Vec<String>> {
        self.api.derivs.list_derivs_pairs()
    }

    pub fn get_trading_pair_ws(&self) -> TradingPair {
        self.ws.get_trading_pair().unwrap()
    }

    pub fn get_candles_ws(&self) -> VecDeque<Candle> {
        self.ws.get_candles()
    }

    pub fn get_trades_ws(&self) -> VecDeque<TradesTradingPair> {
        self.ws.get_trades()
    }

    pub fn get_position_ws(&self) -> Option<Position> {
        let positions = self.ws.get_positions();
        for p in &positions {
            if p.symbol == self.symbol {
                return Some(p.clone());
            }
        }
        None
    }

    pub fn get_balance_info_ws(&self) -> BalanceInfo {
        self.ws.get_balance().unwrap()
    }

    pub fn get_wallet_ws(&self) -> Option<WalletWs> {
        let wallets = self.ws.get_wallet();
        for w in &wallets {
            if w.currency.to_uppercase() == USTF0 {
                return Some(w.clone());
            }
        }
        None
    }

    pub fn get_filled_orders_ws(&self) -> VecDeque<OrderData> {
        self.ws
            .get_filled_orders()
            .into_iter()
            .filter(|data| data.client_id == self.client_id)
            .collect()
    }

    pub fn get_open_orders_ws(&self) -> VecDeque<OrderData> {
        self.ws
            .get_open_orders()
            .into_iter()
            .filter(|data| data.client_id == self.client_id)
            .collect()
    }

    pub fn get_canceled_orders(&self) -> VecDeque<OrderData> {
        self.ws
            .get_canceled_orders()
            .into_iter()
            .filter(|data| data.client_id == self.client_id)
            .collect()
    }

    pub fn get_filled_order(&self, order_id: i64) -> Option<OrderData> {
        self.ws.get_filled_order(order_id)
    }

    pub fn get_open_order(&self, order_id: i64) -> Option<OrderData> {
        self.ws.get_open_order(order_id)
    }

    pub fn get_canceled_order(&self, order_id: i64) -> Option<OrderData> {
        self.ws.get_canceled_order(order_id)
    }
}
#[derive(Clone)]
pub struct DerivsData {
    last_day_candles: Arc<RwLock<Vec<Candle>>>,
}

impl Default for DerivsData {
    fn default() -> DerivsData {
        DerivsData {
            last_day_candles: Arc::new(RwLock::new(Vec::default())),
        }
    }
}

impl DerivsData {
    pub fn get_last_day_candles(&self) -> Vec<Candle> {
        self.last_day_candles.read().unwrap().clone()
    }

    pub fn set_last_day_candles(&mut self, candles: Vec<Candle>) {
        *self.last_day_candles.write().unwrap() = candles;
    }
}

#[allow(unused_mut)]
fn update_derivs_data(mut derivs: Derivs) {
    thread::spawn(move || loop {
        match derivs.get_candles_1m_last_day() {
            Ok(candles) => {
                derivs.data.set_last_day_candles(candles);
            }
            Err(err) => {
                error!("{:?}", err);
            }
        }
        thread::sleep(Duration::from_millis(1000));
    });
}
