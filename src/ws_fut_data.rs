use binance::futures::model::{OrderTradeEvent, OrderUpdate};
use binance::model::{
    AggrTradesEvent, BalanceUpdateEvent, EventBalance, EventPosition, LiquidationEvent,
    LiquidationOrder, MarkPriceEvent,
};
use indexmap::IndexMap;
use serde_json::{from_value, Value};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

type MarkPriceWs = Arc<RwLock<Option<MarkPriceEvent>>>;
type MarkPriceSnapsWs = Arc<RwLock<Vec<MarkPriceEvent>>>;
type AggrTradesWs = Arc<RwLock<Vec<AggrTradesEvent>>>;
type LiquidationsWs = Arc<RwLock<Vec<LiquidationOrder>>>;
type PositionsWs = Arc<RwLock<Option<EventPosition>>>;
type BalanceWs = Arc<RwLock<Option<EventBalance>>>;
type OrdersWs = Arc<RwLock<IndexMap<u64, OrderUpdate>>>;

const DATA_SIZE: usize = 500;

#[derive(Clone, Debug)]
pub struct WsData {
    mark_price: MarkPriceWs,
    mark_price_snaps: MarkPriceSnapsWs,
    aggr_trades: AggrTradesWs,
    liquidations: LiquidationsWs,
    position: PositionsWs,
    balance: BalanceWs,
    filled_orders: OrdersWs,
    open_orders: OrdersWs,
    canceled_orders: OrdersWs,
}

impl WsData {
    pub fn default() -> WsData {
        WsData {
            mark_price: Arc::new(RwLock::new(None)),
            mark_price_snaps: Arc::new(RwLock::new(Vec::with_capacity(DATA_SIZE))),
            aggr_trades: Arc::new(RwLock::new(Vec::with_capacity(DATA_SIZE))),
            liquidations: Arc::new(RwLock::new(Vec::with_capacity(DATA_SIZE))),
            position: Arc::new(RwLock::new(None)),
            balance: Arc::new(RwLock::new(None)),
            filled_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            open_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            canceled_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
        }
    }

    pub fn clone(&self) -> WsData {
        WsData {
            mark_price: Arc::clone(&self.mark_price),
            mark_price_snaps: Arc::clone(&self.mark_price_snaps),
            aggr_trades: Arc::clone(&self.aggr_trades),
            liquidations: Arc::clone(&self.liquidations),
            position: Arc::clone(&self.position),
            balance: Arc::clone(&self.balance),
            filled_orders: Arc::clone(&self.filled_orders),
            open_orders: Arc::clone(&self.open_orders),
            canceled_orders: Arc::clone(&self.canceled_orders),
        }
    }

    pub fn get_mark_price(&self) -> Option<MarkPriceEvent> {
        self.mark_price.read().unwrap().clone()
    }

    pub fn get_mark_price_snaps(&self) -> Vec<MarkPriceEvent> {
        self.mark_price_snaps.read().unwrap().clone()
    }

    pub fn get_aggr_trades(&self) -> Vec<AggrTradesEvent> {
        self.aggr_trades.read().unwrap().clone()
    }

    pub fn get_liquidations(&self) -> Vec<LiquidationOrder> {
        self.liquidations.read().unwrap().clone()
    }

    pub fn get_position(&self) -> Option<EventPosition> {
        self.position.read().unwrap().clone()
    }

    pub fn get_balance(&self) -> Option<EventBalance> {
        self.balance.read().unwrap().clone()
    }

    pub fn get_filled_orders(&self) -> Vec<OrderUpdate> {
        self.filled_orders
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    pub fn get_open_orders(&self) -> Vec<OrderUpdate> {
        self.open_orders.read().unwrap().values().cloned().collect()
    }

    pub fn get_canceled_orders(&self) -> Vec<OrderUpdate> {
        self.canceled_orders
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }
    pub fn get_filled_order(&self, order_id: u64) -> Option<OrderUpdate> {
        let filled_orders = self.filled_orders.read().unwrap();
        get_order(filled_orders, order_id)
    }

    pub fn get_open_order(&self, order_id: u64) -> Option<OrderUpdate> {
        let open_orders = self.open_orders.read().unwrap();
        get_order(open_orders, order_id)
    }

    pub fn get_canceled_order(&self, order_id: u64) -> Option<OrderUpdate> {
        let canceled_orders: RwLockReadGuard<IndexMap<u64, OrderUpdate>> =
            self.canceled_orders.read().unwrap();
        get_order(canceled_orders, order_id)
    }

    pub fn update_mark_price(&self, event: MarkPriceEvent) {
        let mut mark_price: RwLockWriteGuard<Option<MarkPriceEvent>> =
            self.mark_price.write().unwrap();
        *mark_price = Some(event);
    }

    pub fn add_mark_price_snap(&self, event: MarkPriceEvent) {
        insert_vec(self.mark_price_snaps.write().unwrap(), event);
    }

    pub fn add_aggr_trades(&self, event: AggrTradesEvent) {
        insert_vec(self.aggr_trades.write().unwrap(), event);
    }

    pub fn add_liquidation(&self, event: LiquidationOrder) {
        insert_vec(self.liquidations.write().unwrap(), event);
    }

    pub fn update_position(&self, event: EventPosition) {
        let mut position: RwLockWriteGuard<Option<EventPosition>> = self.position.write().unwrap();
        *position = Some(event);
    }

    pub fn update_balance(&self, event: EventBalance) {
        let mut balance: RwLockWriteGuard<Option<EventBalance>> = self.balance.write().unwrap();
        *balance = Some(event);
    }

    pub fn add_order(&self, order: OrderUpdate) {
        let order_id = order.clone().order_id;
        let order_status = order.clone().order_status;

        if order_status == "NEW" {
            insert_order_index_map(self.open_orders.write().unwrap(), order_id, order);
        } else if order_status == "FILLED" {
            insert_order_index_map(self.filled_orders.write().unwrap(), order_id, order);
        } else if order_status == "CANCELED" {
            insert_order_index_map(self.canceled_orders.write().unwrap(), order_id, order);
        }
    }
}

fn insert_order_index_map(
    mut index_map: RwLockWriteGuard<IndexMap<u64, OrderUpdate>>,
    order_id: u64,
    order: OrderUpdate,
) {
    if index_map.insert(order_id, order).is_none() && index_map.len() == DATA_SIZE + 1 {
        index_map.pop();
    }
}

fn insert_vec<T>(mut vec: RwLockWriteGuard<Vec<T>>, value: T) {
    vec.insert(0, value);
    if vec.len() >= DATA_SIZE {
        vec.pop();
    }
}

fn get_order(
    index_map: RwLockReadGuard<IndexMap<u64, OrderUpdate>>,
    order_id: u64,
) -> Option<OrderUpdate> {
    let result: Option<&OrderUpdate> = index_map.get(&order_id);
    return match result {
        None => None,
        Some(order) => Some(order.clone()),
    };
}

#[test]
fn test_order_update() {
    let mut json = r#"{
        "s": "BTCUSDT",
        "c": "web_HWhZes7Aql5iv5R6dEaa",
        "S": "BUY",
        "o": "LIMIT",
        "f": "GTC",
        "q": 0.010,
        "p": 15000,
        "ap": 0,
        "sp": 0,
        "x": "NEW",
        "X": "NEW",
        "i": 3252769662,
        "l": 0,
        "z": 0,
        "L": 0,
        "N": "",
        "n": "",
        "T": 1668814069559,
        "t": 0,
        "b": 150,
        "a": 0,
        "m": false,
        "R": false,
        "wt": "CONTRACT_PRICE",
        "ot": "LIMIT",
        "ps": "LONG",
        "cp": false,
        "AP": 0,
        "cr": "",
        "pP": false,
        "si": 0,
        "ss": 0,
        "rp": 0 }"#;
    let v: OrderUpdate = serde_json::from_str(json).unwrap();
    let ws_data = WsData::default();
    ws_data.add_order(v.to_owned());
    assert_eq!(ws_data.get_open_orders().len(), 1);
    assert_eq!(ws_data.get_filled_orders().len(), 0);
    assert_eq!(ws_data.get_canceled_orders().len(), 0);

    json = r#"{
        "s": "BTCUSDT",
        "c": "web_HWhZes7Aql5iv5R6dEaa",
        "S": "BUY",
        "o": "LIMIT",
        "f": "GTC",
        "q": 0.010,
        "p": 15000,
        "ap": 0,
        "sp": 0,
        "x": "FILLED",
        "X": "FILLED",
        "i": 3252769662,
        "l": 0,
        "z": 0,
        "L": 0,
        "N": "",
        "n": "",
        "T": 1668814069559,
        "t": 0,
        "b": 150,
        "a": 0,
        "m": false,
        "R": false,
        "wt": "CONTRACT_PRICE",
        "ot": "LIMIT",
        "ps": "LONG",
        "cp": false,
        "AP": 0,
        "cr": "",
        "pP": false,
        "si": 0,
        "ss": 0,
        "rp": 0 }"#;
    let v: OrderUpdate = serde_json::from_str(json).unwrap();
    ws_data.add_order(v.to_owned());
    assert_eq!(ws_data.get_open_orders().len(), 1);
    assert_eq!(ws_data.get_filled_orders().len(), 1);
    assert_eq!(ws_data.get_canceled_orders().len(), 0);

    json = r#"{
        "s": "BTCUSDT",
        "c": "web_HWhZes7Aql5iv5R6dEaa",
        "S": "BUY",
        "o": "LIMIT",
        "f": "GTC",
        "q": 0.010,
        "p": 15000,
        "ap": 0,
        "sp": 0,
        "x": "CANCELED",
        "X": "CANCELED",
        "i": 3252769662,
        "l": 0,
        "z": 0,
        "L": 0,
        "N": "",
        "n": "",
        "T": 1668814069559,
        "t": 0,
        "b": 150,
        "a": 0,
        "m": false,
        "R": false,
        "wt": "CONTRACT_PRICE",
        "ot": "LIMIT",
        "ps": "LONG",
        "cp": false,
        "AP": 0,
        "cr": "",
        "pP": false,
        "si": 0,
        "ss": 0,
        "rp": 0 }"#;
    let v: OrderUpdate = serde_json::from_str(json).unwrap();
    ws_data.add_order(v.to_owned());
    assert_eq!(ws_data.get_open_orders().len(), 1);
    assert_eq!(ws_data.get_filled_orders().len(), 1);
    assert_eq!(ws_data.get_canceled_orders().len(), 1);
}

#[test]
fn test_mark_price_update() {

}

#[test]
fn test_mark_price_snaps_update() {

}

#[test]
fn test_aggr_trades_update() {

}

#[test]
fn test_liquidations_update() {

}

#[test]
fn test_position_update() {

}

#[test]
fn test_balance_update() {

}

