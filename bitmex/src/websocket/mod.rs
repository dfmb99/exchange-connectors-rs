use crate::rest::BitmexRest;
use crate::utils::auth::{generate_signature, AuthData};
use crate::utils::data::execution::Execution;
use crate::utils::data::instrument::Instrument;
use crate::utils::data::order::Order;
use crate::utils::data::order_book_l2::OrderBookL2;
use crate::utils::data::position::Position;
use crate::utils::data::trade_bucketed::TradeBucketed;
use crate::utils::data::user_margin::Margin;
use crate::utils::enums::Subscriptions;
use crate::utils::thread_pool::ThreadPool;
use actix_rt::System;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, error};
use rayon::prelude::*;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tungstenite::client::AutoStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

const URL_MAIN: &str = "wss://www.bitmex.com/realtime";
const URL_TEST: &str = "wss://testnet.bitmex.com/realtime";
const LEN_ORDERS: usize = 40;
const LEN_TRADE_BUCKETED: usize = 10080;
const MAX_TRADE_BUCKETED_API_SIZE: usize = 1000;
const TICKER_SNAPS_SIZE_DEFAULT: usize = 720;
const LEN_DEFAULT: usize = 100;
const CONNECT_ERR_SLEEP_MS: u64 = 1000;
const ERR_SYS_TIME: &str = "Time went backwards";

type Socket = Arc<Mutex<WebSocket<AutoStream>>>;
type Keys = Arc<RwLock<HashMap<String, Vec<String>>>>;
type Data = Arc<RwLock<HashMap<String, Vec<Map<String, Value>>>>>;
type InstrumentSnaps = Arc<RwLock<Vec<Instrument>>>;
type L2Data = Arc<RwLock<Vec<Map<String, Value>>>>;
// vec that stores all the ordered ids of the table "orderBookL2"
type L2ids = Arc<RwLock<Vec<u64>>>;

#[derive(Clone)]
pub struct BitmexWs {
    testnet: bool,
    symbol: String,
    thread_pool_size: usize,
    subscriptions: Vec<Subscriptions>,
    socket: Socket,
    auth_data: AuthData,
    keys: Keys,
    data: Data,
    instrument_snaps: InstrumentSnaps,
    l2_data: L2Data,
    l2_ids: L2ids,
}

impl BitmexWs {
    pub async fn new(
        testnet: bool,
        symbol: String,
        thread_pool_size: usize,
        subscriptions: Vec<Subscriptions>,
        auth_data: AuthData,
    ) -> BitmexWs {
        let mut ws = BitmexWs {
            socket: Arc::new(Mutex::new(connect_ws(
                if testnet { URL_TEST } else { URL_MAIN },
                symbol.to_string(),
                &auth_data,
                &subscriptions,
            ))),
            keys: Arc::new(RwLock::new(HashMap::new())),
            data: Arc::new(RwLock::new(HashMap::new())),
            l2_data: Arc::new(RwLock::new(Vec::new())),
            l2_ids: Arc::new(RwLock::new(Vec::new())),
            instrument_snaps: Arc::new(RwLock::new(Vec::new())),
            thread_pool_size,
            symbol,
            testnet,
            subscriptions,
            auth_data,
        };
        ws.run().await;
        ws.wait_for_data();
        let inst_snaps = Arc::clone(&ws.instrument_snaps);
        let data_clone = Arc::clone(&ws.data);

        thread::spawn(move || loop {
            let data_clone = data_clone.clone();
            {
                let data = data_clone.read().unwrap();
                let instrument_data = data.get("instrument");
                assert!(instrument_data.is_some());
                let instrument_data = instrument_data.unwrap().clone();
                let instrument_data: Vec<Instrument> =
                    serde_json::from_value(json!(instrument_data)).unwrap();
                update_ticker_snaps(
                    inst_snaps.to_owned(),
                    instrument_data.first().unwrap().to_owned(),
                );
                drop(data);
            }
            sleep(Duration::from_secs(5));
        });
        ws
    }

    fn wait_for_data(&self) {
        while !self.is_data_available() {
            thread::yield_now();
        }
    }

    fn is_data_available(&self) -> bool {
        self.data.read().unwrap().len() == self.subscriptions.len()
    }

    /// Starts websocket connection
    async fn run(&mut self) {
        let thread_pool = ThreadPool::new(self.thread_pool_size);
        let socket_clone = Arc::clone(&self.socket);
        let data_clone = Arc::clone(&self.data);
        let keys_clone = Arc::clone(&self.keys);
        let l2_data_clone = Arc::clone(&self.l2_data);
        let l2_ids_clone = Arc::clone(&self.l2_ids);
        let symbol = self.symbol.to_string();
        let auth_data = self.auth_data.to_owned();
        let testnet = self.testnet;
        let subscriptions = self.subscriptions.clone();

        // data updates are managed by a thread pool asynchronously
        thread::spawn(move || loop {
            let mut socket = socket_clone.lock().unwrap();
            match socket.read_message() {
                Ok(msg) => {
                    debug!("Received ws message: {:?}", msg);
                    let map_msg: Map<String, Value> =
                        serde_json::from_str(&msg.to_string()[..]).unwrap_or_default();
                    if let Some(data) = map_msg.get("data") {
                        let keys_memory = Arc::clone(&keys_clone);
                        let data_memory = Arc::clone(&data_clone);
                        let l2_data_memory = Arc::clone(&l2_data_clone);
                        let l2_ids_memory = Arc::clone(&l2_ids_clone);
                        let table = map_msg.get("table").unwrap().as_str().unwrap().to_string();
                        let action = map_msg.get("action").unwrap().as_str().unwrap().to_string();
                        let symbol2 = symbol.to_string();
                        let authdata2 = auth_data.to_owned();
                        let data_rec = data.to_owned();
                        thread_pool.execute(move || {
                            if action == "partial" {
                                let keys_rec = map_msg.get("keys").unwrap().to_owned();
                                System::new("env").block_on(async move {
                                    handle_partial(
                                        symbol2,
                                        testnet,
                                        authdata2,
                                        &table[..],
                                        keys_rec,
                                        data_rec,
                                        data_memory,
                                        keys_memory,
                                        l2_ids_memory,
                                        l2_data_memory,
                                    )
                                    .await;
                                });
                            } else if data_memory.read().unwrap().contains_key(&table)
                                || (l2_data_memory.read().unwrap().len() > 0
                                    && table == "orderBookL2")
                            {
                                handle_data_msg(
                                    table,
                                    action,
                                    data_rec,
                                    data_memory,
                                    keys_memory,
                                    l2_ids_memory,
                                    l2_data_memory,
                                );
                            }
                        });
                    } else if let (Some(_), Some(subscription)) =
                        (map_msg.get("success"), map_msg.get("subscribe"))
                    {
                        debug!("Subscribed to: {}", subscription.as_str().unwrap());
                    } else if let Some(info) = map_msg.get("info") {
                        debug!("{}", info.as_str().unwrap());
                    } else if let Some(error) = map_msg.get("error") {
                        let status = map_msg.get("status").unwrap().as_i64().unwrap();
                        error!("Code: {}, Error: {}", status, error);
                    } else {
                        debug!("Unknown message: {:?}", msg);
                    }
                }
                Err(err) => {
                    error!("Error reading ws message: {}", err);
                    *socket = connect_ws(
                        if testnet { URL_TEST } else { URL_MAIN },
                        symbol.to_string(),
                        &auth_data,
                        &subscriptions,
                    );
                }
            };
        });
    }

    /// Returns size of orders at a given order book level
    pub fn get_order_book_size(&self, price: f64) -> u64 {
        let book_data = self.l2_data.read().unwrap().to_vec();
        let mut book_data: Vec<OrderBookL2> = serde_json::from_value(json!(book_data)).unwrap();
        book_data.reverse();
        let location = book_data.binary_search_by(|v| v.price.partial_cmp(&price).unwrap());
        return match location {
            Ok(pos) => book_data.get(pos).unwrap().size,
            _ => 0,
        };
    }

    /// Returns orderbook impact price of a given order size
    pub fn get_order_book_impact_price(&self, contracts: i64) -> f64 {
        let mut location = Err(0);
        let book_data = self.l2_data.read().unwrap().to_vec();
        let mut book_data: Vec<OrderBookL2> = serde_json::from_value(json!(book_data)).unwrap();
        book_data.reverse();
        let instrument = &self.get_instrument()[0];
        let mut price = instrument.mid_price.round();
        let mut sum = 0_u64;

        // find best bid or best ask
        while location.is_err() {
            location = book_data.binary_search_by(|v| v.price.partial_cmp(&price).unwrap());
            price += if contracts > 0 {
                -instrument.tick_size
            } else {
                instrument.tick_size
            };
        }
        assert!(location.is_ok());
        let mut pos = location.unwrap();
        // find impact price
        while sum <= contracts.unsigned_abs() && pos > 0 && pos < book_data.len() {
            let entry = book_data.get(pos).unwrap();
            sum += entry.size;
            price = entry.price;
            if contracts > 0 {
                pos += 1;
            } else {
                pos -= 1;
            }
        }
        price
    }

    /// Returns depth of book (bids_sum, asks_sum)
    pub fn get_order_book_depth(
        &self,
        mid_price: f64,
        mut low: f64,
        high: f64,
        tick_size: f64,
    ) -> (u64, u64) {
        let mut price = low;
        let mut location = Err(0);
        let mut bids_size = 0;
        let mut asks_size = 0;
        let book_data = self.l2_data.read().unwrap().to_vec();
        let mut book_data: Vec<OrderBookL2> = serde_json::from_value(json!(book_data)).unwrap();
        book_data.reverse();
        while location.is_err() {
            location = book_data.binary_search_by(|v| v.price.partial_cmp(&low).unwrap());
            low += tick_size;
        }
        assert!(location.is_ok());
        let mut pos = location.unwrap();
        while price <= high && pos < book_data.len() {
            let entry = book_data.get(pos).unwrap();
            if entry.price < mid_price {
                bids_size += entry.size;
            }
            if entry.price > mid_price {
                asks_size += entry.size;
            }
            price += tick_size;
            pos += 1;
        }
        (bids_size, asks_size)
    }

    /// Returns open orders
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `order` stream
    pub fn get_open_orders(&self) -> Vec<Order> {
        let data = self.data.read().unwrap();
        let order_data = data.get("order");
        assert!(order_data.is_some());
        let order_data = order_data.unwrap().clone();
        let order_data: Vec<Order> = serde_json::from_value(json!(order_data)).unwrap();
        order_data
            .into_iter()
            .filter(|item| item.ord_status == "New" || item.ord_status == "PartiallyFilled")
            .collect()
    }

    /// Returns filled orders
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `order` stream
    pub fn get_filled_orders(&self) -> Vec<Order> {
        let data = self.data.read().unwrap();
        let order_data = data.get("order");
        assert!(order_data.is_some());
        let order_data = order_data.unwrap().clone();
        let order_data: Vec<Order> = serde_json::from_value(json!(order_data)).unwrap();
        order_data
            .into_iter()
            .filter(|item| item.ord_status == "Filled")
            .collect()
    }

    /// Returns canceled orders
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `order` stream
    pub fn get_canceled_orders(&self) -> Vec<Order> {
        let data = self.data.read().unwrap();
        let order_data = data.get("order");
        assert!(order_data.is_some());
        let order_data = order_data.unwrap().clone();
        let order_data: Vec<Order> = serde_json::from_value(json!(order_data)).unwrap();
        order_data
            .into_iter()
            .filter(|item| item.ord_status == "Canceled")
            .collect()
    }

    /// Returns position data
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `position` stream
    pub fn get_position(&self) -> Vec<Position> {
        let data = self.data.read().unwrap();
        let position_data = data.get("position");
        assert!(position_data.is_some());
        let position_data = position_data.unwrap().clone();
        serde_json::from_value(json!(position_data)).unwrap()
    }

    /// Returns instrument data
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `instrument` stream
    pub fn get_instrument(&self) -> Vec<Instrument> {
        let data = self.data.read().unwrap();
        let instrument_data = data.get("instrument");
        assert!(instrument_data.is_some());
        let instrument_data = instrument_data.unwrap().clone();
        serde_json::from_value(json!(instrument_data)).unwrap()
    }

    /// Returns tradeBin1m data
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `tradeBin1m` stream
    pub fn get_trade_bin_1m(&self) -> Vec<TradeBucketed> {
        let data = self.data.read().unwrap();
        let trade_data = data.get("tradeBin1m");
        assert!(trade_data.is_some());
        let trade_data = trade_data.unwrap().clone();
        serde_json::from_value(json!(trade_data)).unwrap()
    }

    /// Returns tradeBin5m data
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `tradeBin5m` stream
    pub fn get_trade_bin_5m(&self) -> Vec<TradeBucketed> {
        let data = self.data.read().unwrap();
        let trade_data = data.get("tradeBin5m");
        assert!(trade_data.is_some());
        let trade_data = trade_data.unwrap().clone();
        serde_json::from_value(json!(trade_data)).unwrap()
    }

    /// Returns tradeBin1h data
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `tradeBin1h` stream
    pub fn get_trade_bin_1h(&self) -> Vec<TradeBucketed> {
        let data = self.data.read().unwrap();
        let trade_data = data.get("tradeBin1h");
        assert!(trade_data.is_some());
        let trade_data = trade_data.unwrap().clone();
        serde_json::from_value(json!(trade_data)).unwrap()
    }

    /// Returns tradeBin1d data
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `tradeBin1d` stream
    pub fn get_trade_bin_1d(&self) -> Vec<TradeBucketed> {
        let data = self.data.read().unwrap();
        let trade_data = data.get("tradeBin1d");
        assert!(trade_data.is_some());
        let trade_data = trade_data.unwrap().clone();
        serde_json::from_value(json!(trade_data)).unwrap()
    }

    /// Returns execution data
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `execution` stream
    pub fn get_execution(&self) -> Vec<Execution> {
        let data = self.data.read().unwrap();
        let execution_data = data.get("execution");
        assert!(execution_data.is_some());
        let execution_data = execution_data.unwrap().clone();
        serde_json::from_value(json!(execution_data)).unwrap()
    }

    /// Returns margin data
    ///
    /// # Panics
    /// Will panic if the user did not subscribe to the `margin` stream
    pub fn get_margin(&self) -> Vec<Margin> {
        let data = self.data.read().unwrap();
        let margin_data = data.get("margin");
        assert!(margin_data.is_some());
        let margin_data = margin_data.unwrap().clone();
        serde_json::from_value(json!(margin_data)).unwrap()
    }

    pub fn get_ticker_snaps(&self) -> Vec<Instrument> {
        self.instrument_snaps.read().unwrap().to_owned()
    }
}

// Updates ticker snaps vec with current instrument data
pub fn update_ticker_snaps(instrument_snaps: InstrumentSnaps, instrument: Instrument) {
    let mut ticker_snaps = instrument_snaps.write().unwrap();
    if ticker_snaps.len() >= TICKER_SNAPS_SIZE_DEFAULT {
        ticker_snaps.pop();
    }
    ticker_snaps.insert(0, instrument);
}

/// Handles BitMex websocket 'partial' message.
#[allow(clippy::too_many_arguments)]
async fn handle_partial(
    symbol: String,
    testnet: bool,
    auth_data: AuthData,
    table: &str,
    keys: Value,
    data: Value,
    data_memory: Data,
    keys_memory: Keys,
    l2_ids: L2ids,
    l2_data: L2Data,
) {
    let data_rec: Vec<Map<String, Value>> = serde_json::from_value(data).unwrap();
    let keys: Vec<String> = serde_json::from_value(keys).unwrap();
    let rest = BitmexRest::new(testnet, auth_data.to_owned()).await;

    keys_memory
        .write()
        .unwrap()
        .insert(String::from(table), keys);
    if table == "order" {
        let mut filter: HashMap<&str, Value> = HashMap::with_capacity(1);
        filter.insert("open", json!(true));

        let mut params: HashMap<&str, Value> = HashMap::with_capacity(4);
        params.insert("symbol", json!(symbol.to_owned()));
        params.insert("reverse", json!(true));
        params.insert("count", json!(LEN_ORDERS));
        params.insert("filter", json!(filter));

        let mut response = rest.get_order(params.clone()).await;
        while response.is_err() {
            response = rest.get_trades_bucketed(params.clone()).await;
        }
        let mut response_vec: Vec<Map<String, Value>> =
            serde_json::from_value(response.unwrap()).unwrap();
        response_vec.reverse();

        data_memory
            .write()
            .unwrap()
            .insert(String::from(table), response_vec);
    } else if table == "tradeBin1m" {
        let response_vec =
            get_trade_bin_1m_api(symbol.to_owned(), testnet, auth_data.to_owned()).await;
        data_memory
            .write()
            .unwrap()
            .insert(String::from(table), response_vec);
    } else if table == "orderBookL2" {
        let mut l2_ids_w = l2_ids.write().unwrap();
        let mut l2_data_w = l2_data.write().unwrap();
        let ids: Vec<u64> = data_rec
            .par_iter()
            .map(|x| x.get("id").unwrap().as_u64().unwrap())
            .collect();
        *l2_ids_w = ids;
        *l2_data_w = data_rec;
        let empty: Vec<Map<String, Value>> = Vec::new();
        data_memory
            .write()
            .unwrap()
            .insert(String::from(table), empty);
    } else {
        data_memory
            .write()
            .unwrap()
            .insert(String::from(table), data_rec);
    }
}

async fn get_trade_bin_1m_api(
    symbol: String,
    testnet: bool,
    auth_data: AuthData,
) -> Vec<Map<String, Value>> {
    let rest = BitmexRest::new(testnet, auth_data.to_owned()).await;
    let mut data: Vec<Map<String, Value>> = Vec::with_capacity(LEN_TRADE_BUCKETED);
    let mut count = 0;
    let mut end_time = String::new();
    while count < LEN_TRADE_BUCKETED {
        let count_param = if LEN_TRADE_BUCKETED - count < MAX_TRADE_BUCKETED_API_SIZE {
            LEN_TRADE_BUCKETED - count
        } else {
            MAX_TRADE_BUCKETED_API_SIZE
        };
        let mut params: HashMap<&str, Value> = HashMap::with_capacity(4);
        params.insert("symbol", json!(symbol.to_owned()));
        params.insert("reverse", json!(true));
        params.insert("binSize", json!("1m"));
        params.insert("count", json!(count_param));
        if !end_time.is_empty() {
            params.insert("endTime", json!(end_time));
        }
        count += count_param;

        let mut response = rest.get_trades_bucketed(params.clone()).await;
        while response.is_err() {
            response = rest.get_trades_bucketed(params.clone()).await;
        }
        let mut response_vec: Vec<Map<String, Value>> =
            serde_json::from_value(response.unwrap()).unwrap();
        end_time = String::from(
            response_vec
                .last()
                .unwrap()
                .get("timestamp")
                .unwrap()
                .as_str()
                .unwrap(),
        );
        let naive_datetime =
            NaiveDateTime::parse_from_str(&end_time, "%Y-%m-%dT%H:%M:%S.000Z").unwrap();
        let datetime_utc: DateTime<Utc> =
            DateTime::from_naive_utc_and_offset(naive_datetime, Utc) - chrono::Duration::minutes(1);
        end_time = datetime_utc.to_string();
        data.append(&mut response_vec);
    }
    data.reverse();
    data
}

/// Handles BitMex websocket `insert`, `update`, `delete` message.
fn handle_data_msg(
    table: String,
    action: String,
    data_rec: Value,
    data_memory: Data,
    keys: Keys,
    l2_ids: L2ids,
    l2_data: L2Data,
) {
    let mut data_rec: Vec<Map<String, Value>> = serde_json::from_value(data_rec).unwrap();
    let mut data_memory = data_memory.write().unwrap();
    let mut l2_data_memory = l2_data.write().unwrap();

    if &action[..] == "insert" {
        let data_memory: &mut Vec<Map<String, Value>> = data_memory.get_mut(&table[..]).unwrap();

        // vec cannot grow forever, need to trim data
        let len = match &table[..] {
            "order" => LEN_ORDERS,
            table if table.contains("orderBookL2") => usize::MAX,
            table if table.contains("tradeBin") => LEN_TRADE_BUCKETED,
            _ => LEN_DEFAULT,
        };

        if data_memory.len() + data_rec.len() > len {
            // rmv_count >=1
            let rmv_count = data_memory.len() + data_rec.len() - len;
            for _ in 1..=rmv_count {
                if !data_memory.is_empty() {
                    data_memory.remove(0);
                }
            }
        }
        // inserts w/ table 'orderBookL2' are not FIFO, they are sorted by price
        if table == "orderBookL2" {
            for map in data_rec {
                let id = map.get("id").unwrap().as_u64().unwrap();
                let location = l2_ids.read().unwrap().binary_search(&id);
                if let Err(pos) = location {
                    l2_ids.write().unwrap().insert(pos, id);
                    l2_data_memory.insert(pos, map);
                }
            }
        } else {
            data_memory.append(&mut data_rec);
        }
    } else {
        for update_data in data_rec {
            let data_memory: &mut Vec<Map<String, Value>> =
                data_memory.get_mut(&table[..]).unwrap();
            #[allow(unused_assignments)]
            let mut item = None;
            if table == "orderBookL2" {
                item = find_item_order_bookl2(&l2_data_memory, &update_data);
            } else {
                item = find_item_by_keys(
                    keys.read().unwrap().get(&table[..]).unwrap(),
                    data_memory,
                    &update_data,
                )
            }
            if action == "update" {
                if let Some((index, mut item)) = item {
                    item.extend(update_data.clone());
                    if table == "orderBookL2" {
                        l2_data_memory.remove(index);
                        l2_data_memory.insert(index, item);
                    } else {
                        data_memory.remove(index);
                        data_memory.insert(index, item);
                    }
                }
            } else if action == "delete" {
                if let Some((index, _)) = item {
                    if table == "orderBookL2" {
                        l2_ids.write().unwrap().remove(index);
                        l2_data_memory.remove(index);
                    } else {
                        data_memory.remove(index);
                    }
                }
            }
        }
    }
}

/// Tries to connect to BitMex websocket and returns socket, if any error is thrown retries after a timeout
fn connect_ws(
    base_uri: &str,
    symbol: String,
    auth_data: &AuthData,
    subscriptions: &Vec<Subscriptions>,
) -> WebSocket<AutoStream> {
    loop {
        if let Ok(ws_stream) = connect(Url::parse(base_uri).unwrap()) {
            let mut socket = ws_stream.0;
            let mut args: Vec<Value> = Vec::with_capacity(subscriptions.len());
            for x in subscriptions {
                let sub = x.value().to_string();
                if sub == "margin"
                    || sub == "wallet"
                    || sub == "transact"
                    || sub.contains("Notifications")
                    || sub == "chat"
                    || sub == "announcement"
                    || sub == "connected"
                {
                    args.push(json!(sub));
                } else {
                    args.push(json!(sub + ":" + &symbol));
                }
            }
            authenticate(&mut socket, auth_data);
            send_command(&mut socket, "subscribe", &args);
            return socket;
        }
        sleep(Duration::from_millis(CONNECT_ERR_SLEEP_MS))
    }
}

/// Send command to BitMex websocket to authenticate account, if authData has API key and secret
fn authenticate(socket: &mut WebSocket<AutoStream>, auth_data: &AuthData) {
    match auth_data {
        AuthData::Data { key, secret } => {
            let expires = get_unix_time_secs() + 60;
            let sig = generate_signature(secret, "GET", "/realtime", &expires.to_string()[..], "");
            let args: Vec<Value> = vec![json!(key), json!(expires), json!(sig)];
            send_command(socket, "authKeyExpires", &args);
        }
        AuthData::None => {}
    }
}

/// Sends raw command.
#[allow(unused_must_use)]
fn send_command(socket: &mut WebSocket<AutoStream>, command: &str, args: &Vec<Value>) {
    let mut params: HashMap<&str, Value> = HashMap::with_capacity(2);
    params.insert("op", json!(command));
    params.insert("args", json!(args));
    let json = serde_json::to_string(&params);
    assert!(json.is_ok());
    socket.write_message(Message::Text(json.unwrap()));
}

fn find_item_by_keys(
    keys: &Vec<String>,
    table: &[Map<String, Value>],
    match_data: &Map<String, Value>,
) -> Option<(usize, Map<String, Value>)> {
    for (index, item) in table.iter().enumerate() {
        let mut matched = true;
        for key in keys {
            if item.get(key).unwrap() != match_data.get(key).unwrap() {
                matched = false;
            }
        }
        if matched {
            return Some((index, item.clone()));
        }
    }
    None
}

fn find_item_order_bookl2(
    table: &[Map<String, Value>],
    match_data: &Map<String, Value>,
) -> Option<(usize, Map<String, Value>)> {
    let id = match_data.get("id").unwrap().as_u64().unwrap();
    let index = table.binary_search_by(|v| {
        v.get("id")
            .unwrap()
            .as_u64()
            .unwrap()
            .partial_cmp(&id)
            .unwrap()
    });
    return match index {
        Ok(pos) => Some((pos, table.get(pos).unwrap().clone())),
        _ => None,
    };
}

fn get_unix_time_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(ERR_SYS_TIME)
        .as_secs()
}

#[cfg(test)]
mod tests {

    use crate::utils::auth::AuthData;
    use crate::utils::enums::Subscriptions;
    use crate::websocket::{get_trade_bin_1m_api, BitmexWs, LEN_TRADE_BUCKETED};
    use actix_rt::System;
    use chrono::{DateTime, NaiveDateTime, Utc};

    #[test]
    fn test_ws_trade_bin_1m() {
        System::new("test").block_on(async {
            let _ = env_logger::try_init();
            let sub = vec![Subscriptions::TradeBin1m];
            let mut ws = BitmexWs::new(true, "XBTUSD".to_string(), 1, sub, AuthData::None).await;
            ws.run().await;
            println!("{:?}", ws.get_trade_bin_1m());
        });
    }

    #[test]
    fn test_ws_trade_bin_api() {
        System::new("test").block_on(async {
            let _ = env_logger::try_init();
            let result = get_trade_bin_1m_api(
                "XBTUSD".to_string(),
                true,
                AuthData::Data {
                    key: "hvcfpTU1oyvDkSD9eeGkLUyg".to_string(),
                    secret: "ReYIfVgcDRBIZcPFXAE464lTHq-v4RW6MoJay-sXmOfmgMjc".to_string(),
                },
            )
            .await;
            assert_eq!(result.len(), LEN_TRADE_BUCKETED);
            let start_time = result
                .first()
                .unwrap()
                .get("timestamp")
                .unwrap()
                .as_str()
                .unwrap();
            let naive_datetime =
                NaiveDateTime::parse_from_str(start_time, "%Y-%m-%dT%H:%M:%S.000Z").unwrap();
            let mut datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);
            for x in result {
                let timestamp = String::from(x.get("timestamp").unwrap().as_str().unwrap());
                let timestamp =
                    NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%dT%H:%M:%S.000Z").unwrap();
                let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc);
                assert_eq!(datetime_utc, timestamp);
                datetime_utc += chrono::Duration::minutes(1);
            }
        });
    }

    #[test]
    fn test_ws_get_order_book_impact_price() {
        System::new("test").block_on(async {
            let _ = env_logger::try_init();
            let sub = vec![Subscriptions::Instrument, Subscriptions::OrderBookL2];
            let mut ws = BitmexWs::new(false, "XBTUSD".to_string(), 1, sub, AuthData::None).await;
            ws.run().await;
            println!("{:?}", ws.get_order_book_impact_price(10000000));
        });
    }
}
