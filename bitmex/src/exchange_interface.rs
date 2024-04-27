use crate::rest::BitmexRest;
use crate::utils::auth::AuthData;
use crate::utils::data::instrument::Instrument;
use crate::utils::data::order::Order;
use crate::utils::data::position::Position;
use crate::utils::data::trade_bucketed::TradeBucketed;
use crate::utils::enums::{RequestResponseErr, Subscriptions};
use crate::websocket::BitmexWs;
use actix_rt::System;
use rayon::prelude::*;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

const ONE_HOUR_AS_SECS: u64 = 86400;

type HistoricalPrices = Arc<RwLock<Vec<TradeBucketed>>>;

#[derive(Clone)]
pub struct BitmexInterface {
    pub rest: BitmexRest,
    pub ws: BitmexWs,
    last_month_hist_prices: HistoricalPrices,
}

impl BitmexInterface {
    pub async fn new(
        testnet: bool,
        auth_data: AuthData,
        symbol: String,
        subscriptions: Vec<Subscriptions>,
    ) -> BitmexInterface {
        let ws = BitmexWs::new(
            testnet,
            symbol.to_owned(),
            num_cpus::get(),
            subscriptions,
            auth_data.to_owned(),
        )
        .await;
        let interface = BitmexInterface {
            rest: BitmexRest::new(testnet, auth_data.to_owned()).await,
            ws,
            last_month_hist_prices: Arc::new(Default::default()),
        };
        let last_month_hist_prices = Arc::clone(&interface.last_month_hist_prices);
        let auth_data_clone = auth_data.clone();
        let symbol_clone = symbol.clone();
        thread::spawn(move || loop {
            let last_month_hist_prices = Arc::clone(&last_month_hist_prices);
            let auth_data_clone = auth_data_clone.clone();
            let symbol_clone = symbol_clone.clone();
            System::new("main").block_on(async move {
                let rest = BitmexRest::new(testnet, auth_data_clone.clone()).await;
                update_last_month_historical_prices(
                    rest,
                    Arc::clone(&last_month_hist_prices),
                    symbol_clone.clone(),
                )
                .await;
                thread::sleep(Duration::from_secs(ONE_HOUR_AS_SECS));
            });
        });
        interface
    }

    /// Gets fair price
    pub fn get_fair_price(&self) -> f64 {
        self.ws.get_instrument().get(0).unwrap().mark_price
    }

    /// Gets current mid price
    pub fn get_mid_price(&self) -> f64 {
        self.ws.get_instrument().get(0).unwrap().mid_price
    }

    /// Gets current ask price
    pub fn get_ask_price(&self) -> f64 {
        self.ws.get_instrument().get(0).unwrap().ask_price
    }

    /// Gets current bid price
    pub fn get_bid_price(&self) -> f64 {
        self.ws.get_instrument().get(0).unwrap().bid_price
    }

    /// Gets L2 orderbook size for specific price
    pub fn get_order_book_size(&self, price: f64) -> u64 {
        self.ws.get_order_book_size(price)
    }

    /// Gets tick size of instrument
    pub fn get_tick_size(&self) -> f64 {
        self.ws.get_instrument().get(0).unwrap().tick_size
    }

    /// Gets margin balance XBT
    pub fn get_margin_balance_xbt(&self) -> u64 {
        for x in self.ws.get_margin() {
            if x.currency == "XBt" {
                return x.margin_balance;
            }
        }
        0
    }

    /// Gets margin balance USDt
    pub fn get_margin_balance_usdt(&self) -> u64 {
        for x in self.ws.get_margin() {
            if x.currency == "USDt" {
                return x.margin_balance;
            }
        }
        0
    }

    /// Gets position
    pub fn get_position_ws(&self) -> Option<Position> {
        self.ws.get_position().get(0).cloned()
    }

    /// Gets margin balance
    pub fn get_position_qty(&self) -> i64 {
        if let Some(position) = self.ws.get_position().get(0) {
            return position.current_qty;
        }
        0_i64
    }

    /// Gets position entry
    pub fn get_position_entry(&self) -> f64 {
        if let Some(position) = self.ws.get_position().get(0) {
            return position.avg_entry_price;
        }
        0.0
    }

    /// Get mark delta for current instrument
    pub fn get_mark_delta(&self) -> f64 {
        let instrument = self.ws.get_instrument();
        if instrument.get(0).unwrap().is_inverse {
            -(instrument.get(0).unwrap().multiplier as f64 / instrument.get(0).unwrap().mark_price)
                * self.get_position_qty() as f64
        } else {
            instrument.get(0).unwrap().multiplier as f64
                * instrument.get(0).unwrap().mark_price
                * self.get_position_qty() as f64
        }
    }

    /// Returns open orders data on contract == symbol, None if error ocurrs
    pub async fn get_open_orders_rest(&self, symbol: String) -> Option<Vec<Order>> {
        let mut params: HashMap<&str, Value> = HashMap::new();
        params.insert("symbol", Value::from(symbol));

        let res = self.rest.get_order(params).await;
        if let Ok(data) = res {
            let orders: Vec<Order> = serde_json::from_value(data).unwrap();
            return Some(
                orders
                    .into_iter()
                    .filter(|item| item.ord_status == "New" || item.ord_status == "PartiallyFilled")
                    .collect(),
            );
        }
        None
    }

    /// Returns order data with order_id == order_id, None if error ocurrs
    pub async fn get_order_id_rest(&self, order_id: &str) -> Option<Order> {
        let mut value: Map<String, Value> = Map::new();
        value.insert(String::from("orderID"), Value::from(order_id));

        let mut params: HashMap<&str, Value> = HashMap::new();
        params.insert("filter", Value::Object(value));
        let res = self.rest.get_order(params).await;
        if let Ok(data) = res {
            let orders: Vec<Order> = serde_json::from_value(data).unwrap();
            if !orders.is_empty() {
                return Some(orders.get(0).unwrap().to_owned());
            }
        }
        None
    }

    /// Returns order data with order_id == order_id, None if error ocurrs
    #[allow(dead_code)]
    pub fn get_order_id_ws(&self, order_id: &str) -> Option<Order> {
        let mut orders = self.ws.get_open_orders();
        orders.append(&mut self.ws.get_filled_orders());
        orders.append(&mut self.ws.get_canceled_orders());

        let orders: Vec<Order> = orders
            .into_iter()
            .filter(|ord| ord.order_id == order_id)
            .collect();
        if !orders.is_empty() {
            return Some(orders.get(0).unwrap().to_owned());
        }
        None
    }

    /// Returns open buy order that is closest to mid price
    #[allow(dead_code)]
    pub fn get_highest_bid(&self) -> Order {
        self.ws
            .get_open_orders()
            .par_iter()
            .filter(|ord| ord.side == "Buy")
            .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
            .unwrap()
            .to_owned()
    }

    /// Returns open sell order that is closest to mid price
    #[allow(dead_code)]
    pub fn get_lowest_sell(&self) -> Order {
        self.ws
            .get_open_orders()
            .par_iter()
            .filter(|ord| ord.side == "Sell")
            .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
            .unwrap()
            .to_owned()
    }

    /// Returns true if order is filled, false otherwise
    ///
    /// * `order_id` - orderID of order
    pub fn is_order_filled(&self, order_id: String) -> bool {
        let orders: Vec<Order> = self
            .ws
            .get_filled_orders()
            .into_iter()
            .filter(|ord| ord.order_id == order_id)
            .collect();
        !orders.is_empty()
    }

    /// Returns true if order is canceled, false otherwise
    ///
    /// * `order_id` - orderID of order
    pub fn is_order_canceled(&self, order_id: String) -> bool {
        let orders: Vec<Order> = self
            .ws
            .get_canceled_orders()
            .into_iter()
            .filter(|ord| ord.order_id == order_id)
            .collect();
        !orders.is_empty()
    }

    /// Place single order and return response
    ///
    /// * `params` - hashmap of parameters of order
    pub async fn place_order(
        &self,
        params: HashMap<&str, Value>,
    ) -> Result<Value, RequestResponseErr> {
        self.rest.place_order(params).await
    }

    /// Amend single order and return response
    ///
    /// * `params` - hashmap of parameters of order
    pub async fn amend_order(
        &self,
        params: HashMap<&str, Value>,
    ) -> Result<Value, RequestResponseErr> {
        self.rest.amend_order(params).await
    }

    /// Cancel multiple orders
    ///
    /// * `order_ids` - list of ids of the orders to cancel
    #[allow(dead_code)]
    pub async fn cancel_orders(&self, order_ids: Vec<String>) -> Result<Value, RequestResponseErr> {
        let mut params: HashMap<&str, Value> = HashMap::with_capacity(1);
        params.insert("orderID", json!(order_ids));
        self.rest.cancel_orders(params).await
    }

    /// Cancel all orders
    ///
    /// * `symbol` - contract to cancel all orders
    pub async fn cancel_all_orders(&self, symbol: String) -> Result<Value, RequestResponseErr> {
        let mut params: HashMap<&str, Value> = HashMap::with_capacity(1);
        params.insert("symbol", json!(symbol));
        self.rest.cancel_all_orders(params).await
    }

    /// Gets tradeBin1m data
    ///
    /// * `count` - count of candles to retrieve
    pub fn get_trade_bin_1m(&self, count: usize) -> Vec<TradeBucketed> {
        let data = self.ws.get_trade_bin_1m();
        data[data.len() - count..data.len()].to_vec()
    }

    /// Gets last month hourly trade bucketed data
    #[allow(dead_code)]
    pub fn get_last_month_historical_prices(&self) -> Vec<TradeBucketed> {
        self.last_month_hist_prices.read().unwrap().to_owned()
    }

    /// Gets last hourly instrument data
    pub fn get_last_hour_historical_prices(&self) -> Vec<Instrument> {
        self.ws.get_ticker_snaps()
    }

    /// Gets usdt pairs info
    pub async fn get_instruments_usdt(&self) -> Option<Vec<Instrument>> {
        let mut value: Map<String, Value> = Map::new();
        value.insert(String::from("settlCurrency"), Value::from("USDt"));
        value.insert(String::from("state"), Value::from("Open"));
        value.insert(String::from("typ"), Value::from("FFWCSX"));

        let mut params: HashMap<&str, Value> = HashMap::new();
        params.insert("filter", Value::Object(value));

        if let Ok(response) = self.rest.get_instrument(params).await {
            return Some(serde_json::from_value(response).unwrap());
        }
        None
    }
}
async fn update_last_month_historical_prices(
    rest: BitmexRest,
    last_month_hist_prices: HistoricalPrices,
    symbol: String,
) {
    let mut params: HashMap<&str, Value> = HashMap::with_capacity(4);
    params.insert("symbol", json!(symbol));
    params.insert("reverse", json!(true));
    params.insert("binSize", json!("1h"));
    params.insert("count", json!(720)); // 1month worth of data

    let result = rest.get_trades_bucketed(params).await;
    if let Ok(trade_data) = result {
        let trade_data: Vec<TradeBucketed> = serde_json::from_value(json!(trade_data)).unwrap();
        *last_month_hist_prices.write().unwrap() = trade_data;
    }
}
