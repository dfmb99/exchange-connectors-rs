use crate::rest::futures::model::OrderUpdate;
use crate::rest::model::{
    AggrTradesEvent, EventBalance, EventPosition, IndexPriceEvent, LiquidationOrder,
};
use indexmap::IndexMap;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

type MarkPriceWs = Arc<RwLock<Option<IndexPriceEvent>>>;
type MarkPriceSnapsWs = Arc<RwLock<VecDeque<IndexPriceEvent>>>;
type AggrTradesWs = Arc<RwLock<VecDeque<AggrTradesEvent>>>;
type LiquidationsWs = Arc<RwLock<VecDeque<LiquidationOrder>>>;
type PositionsWs = Arc<RwLock<Option<EventPosition>>>;
type BalanceWs = Arc<RwLock<Option<EventBalance>>>;
type OrdersWs = Arc<RwLock<IndexMap<u64, OrderUpdate>>>;

const DATA_SIZE: usize = 1000;

#[derive(Debug)]
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

impl Clone for WsData {
    fn clone(&self) -> WsData {
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
}

impl Default for WsData {
    fn default() -> WsData {
        WsData {
            mark_price: Arc::new(RwLock::new(None)),
            mark_price_snaps: Arc::new(RwLock::new(VecDeque::with_capacity(DATA_SIZE))),
            aggr_trades: Arc::new(RwLock::new(VecDeque::with_capacity(DATA_SIZE))),
            liquidations: Arc::new(RwLock::new(VecDeque::with_capacity(DATA_SIZE))),
            position: Arc::new(RwLock::new(None)),
            balance: Arc::new(RwLock::new(None)),
            filled_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            open_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            canceled_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
        }
    }
}

impl WsData {
    pub fn get_mark_price_event(&self) -> Option<IndexPriceEvent> {
        self.mark_price.read().unwrap().clone()
    }

    pub fn get_mark_price_event_snaps(&self) -> VecDeque<IndexPriceEvent> {
        self.mark_price_snaps.read().unwrap().clone()
    }

    pub fn get_aggr_trades(&self) -> VecDeque<AggrTradesEvent> {
        self.aggr_trades.read().unwrap().clone()
    }

    pub fn get_liquidations(&self) -> VecDeque<LiquidationOrder> {
        self.liquidations.read().unwrap().clone()
    }

    pub fn get_position_event(&self) -> Option<EventPosition> {
        self.position.read().unwrap().clone()
    }

    pub fn get_balance_event(&self) -> Option<EventBalance> {
        self.balance.read().unwrap().clone()
    }

    pub fn get_filled_orders(&self) -> VecDeque<OrderUpdate> {
        self.filled_orders
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    pub fn get_open_orders(&self) -> VecDeque<OrderUpdate> {
        self.open_orders.read().unwrap().values().cloned().collect()
    }

    pub fn get_canceled_orders(&self) -> VecDeque<OrderUpdate> {
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

    pub fn update_mark_price(&self, event: IndexPriceEvent) {
        let mut mark_price: RwLockWriteGuard<Option<IndexPriceEvent>> =
            self.mark_price.write().unwrap();
        *mark_price = Some(event);
    }

    pub fn add_mark_price_snap(&self, event: IndexPriceEvent) {
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
        let order_id = order.order_id;
        let order_status = order.clone().order_status;

        if order_status == "NEW" {
            insert_order_index_map(self.open_orders.write().unwrap(), order_id, order);
        } else if order_status == "FILLED" {
            insert_order_index_map(self.filled_orders.write().unwrap(), order_id, order);
            // this order could be previously open so needs to be removed from open orders
            remove_order_index_map(self.open_orders.write().unwrap(), order_id);
        } else if order_status == "CANCELED" {
            insert_order_index_map(self.canceled_orders.write().unwrap(), order_id, order);
            // this order could be previously open so needs to be removed from open orders
            remove_order_index_map(self.open_orders.write().unwrap(), order_id);
        }
    }
}

fn insert_order_index_map(
    mut index_map: RwLockWriteGuard<IndexMap<u64, OrderUpdate>>,
    order_id: u64,
    order: OrderUpdate,
) {
    if index_map.insert(order_id, order).is_none() && index_map.len() == DATA_SIZE + 1 {
        index_map.shift_remove_index(0);
    }
}

fn remove_order_index_map(
    mut index_map: RwLockWriteGuard<IndexMap<u64, OrderUpdate>>,
    order_id: u64,
) {
    index_map.shift_remove(&order_id);
}

fn insert_vec<T>(mut vec: RwLockWriteGuard<VecDeque<T>>, value: T) {
    vec.push_back(value);
    if vec.len() > DATA_SIZE {
        vec.pop_front();
    }
}

fn get_order(
    index_map: RwLockReadGuard<IndexMap<u64, OrderUpdate>>,
    order_id: u64,
) -> Option<OrderUpdate> {
    let result: Option<&OrderUpdate> = index_map.get(&order_id);
    result.cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::model::{AccountUpdateEvent, LiquidationEvent};

    #[test]
    fn test_order_update() {
        let mut json = r#"{
        "s": "BTCUSDT",
        "c": "web_HWhZes7Aql5iv5R6dEaa",
        "S": "BUY",
        "o": "LIMIT",
        "f": "GTC",
        "q": "0.010",
        "p": "15000",
        "ap": "0",
        "sp": "0",
        "x": "NEW",
        "X": "NEW",
        "i": 3252769662,
        "l": "0",
        "z": "0",
        "L": "0",
        "N": "",
        "n": "",
        "T": 1668814069559,
        "t": 0,
        "b": "150",
        "a": "0",
        "m": false,
        "R": false,
        "wt": "CONTRACT_PRICE",
        "ot": "LIMIT",
        "ps": "LONG",
        "cp": false,
        "AP": "0",
        "cr": "",
        "pP": false,
        "si": 0,
        "ss": 0,
        "rp": "0" }"#;
        let v: OrderUpdate = serde_json::from_str(json).unwrap();
        let ws_data = WsData::default();
        ws_data.add_order(v);
        assert_eq!(ws_data.get_open_orders().len(), 1);
        assert_eq!(ws_data.get_filled_orders().len(), 0);
        assert_eq!(ws_data.get_canceled_orders().len(), 0);

        json = r#"{
        "s": "BTCUSDT",
        "c": "web_HWhZes7Aql5iv5R6dEaa",
        "S": "BUY",
        "o": "LIMIT",
        "f": "GTC",
        "q": "0.010",
        "p": "15000",
        "ap": "0",
        "sp": "0",
        "x": "FILLED",
        "X": "FILLED",
        "i": 3252769662,
        "l": "0",
        "z": "0",
        "L": "0",
        "N": "",
        "n": "",
        "T": 1668814069559,
        "t": 0,
        "b": "150",
        "a": "0",
        "m": false,
        "R": false,
        "wt": "CONTRACT_PRICE",
        "ot": "LIMIT",
        "ps": "LONG",
        "cp": false,
        "AP": "0",
        "cr": "",
        "pP": false,
        "si": 0,
        "ss": 0,
        "rp": "0"}"#;
        let v: OrderUpdate = serde_json::from_str(json).unwrap();
        ws_data.add_order(v);
        assert_eq!(ws_data.get_open_orders().len(), 0);
        assert_eq!(ws_data.get_filled_orders().len(), 1);
        assert_eq!(ws_data.get_canceled_orders().len(), 0);

        json = r#"{
        "s": "BTCUSDT",
        "c": "web_HWhZes7Aql5iv5R6dEaa",
        "S": "BUY",
        "o": "LIMIT",
        "f": "GTC",
        "q": "0.010",
        "p": "15000",
        "ap": "0",
        "sp": "0",
        "x": "CANCELED",
        "X": "CANCELED",
        "i": 3252769662,
        "l": "0",
        "z": "0",
        "L": "0",
        "N": "",
        "n": "",
        "T": 1668814069559,
        "t": 0,
        "b": "150",
        "a": "0",
        "m": false,
        "R": false,
        "wt": "CONTRACT_PRICE",
        "ot": "LIMIT",
        "ps": "LONG",
        "cp": false,
        "AP": "0",
        "cr": "",
        "pP": false,
        "si": 0,
        "ss": 0,
        "rp": "0" }"#;
        let v: OrderUpdate = serde_json::from_str(json).unwrap();
        ws_data.add_order(v);
        assert_eq!(ws_data.get_open_orders().len(), 0);
        assert_eq!(ws_data.get_filled_orders().len(), 1);
        assert_eq!(ws_data.get_canceled_orders().len(), 1);
    }

    #[test]
    fn test_mark_price_update() {
        let json = r#"  {
        "e": "markPriceUpdate",
        "E": 1562305380000,
        "s": "BTCUSDT",
        "p": "11794.15000000",
        "i": "11784.62659091",
        "P": "11784.25641265",
        "r": "0.00038167",
        "T": 1562306400000
    }"#;
        let ws_data = WsData::default();
        let v: IndexPriceEvent = serde_json::from_str(json).unwrap();
        ws_data.update_mark_price(v);
        assert!(ws_data.get_mark_price_event().is_some());
        assert_eq!(
            ws_data.get_mark_price_event().unwrap().price,
            "11794.15000000"
        );
    }

    #[test]
    fn test_mark_price_snaps_update() {
        let json = r#"  {
        "e": "markPriceUpdate",
        "E": 1562305380000,
        "s": "BTCUSDT",
        "p": "11794.15000000",
        "i": "11784.62659091",
        "P": "11784.25641265",
        "r": "0.00038167",
        "T": 1562306400000
    }"#;
        let ws_data = WsData::default();
        let v: IndexPriceEvent = serde_json::from_str(json).unwrap();
        ws_data.add_mark_price_snap(v.to_owned());
        assert_eq!(ws_data.get_mark_price_event_snaps().len(), 1);
        ws_data.add_mark_price_snap(v);
        assert_eq!(ws_data.get_mark_price_event_snaps().len(), 2);
    }

    #[test]
    fn test_aggr_trades_update() {
        let json = r#"  {
        "e": "aggTrade",
        "E": 123456789,
        "s": "BTCUSDT",
        "a": 5933014,
        "p": "0.001",
        "q": "100",
        "f": 100,
        "l": 105,
        "T": 123456785,
        "m": true
    }"#;
        let ws_data = WsData::default();
        let v: AggrTradesEvent = serde_json::from_str(json).unwrap();
        ws_data.add_aggr_trades(v.to_owned());
        assert_eq!(ws_data.get_aggr_trades().len(), 1);
        ws_data.add_aggr_trades(v);
        assert_eq!(ws_data.get_aggr_trades().len(), 2);
    }

    #[test]
    fn test_liquidations_update() {
        let json = r#" {
        "e":"forceOrder",
        "E":1568014460893,
        "o":{
            "s":"BTCUSDT",
            "S":"SELL",
            "o":"LIMIT",
            "f":"IOC",
            "q":"0.014",
            "p":"9910",
            "ap":"9910",
            "X":"FILLED",
            "l":"0.014",
            "z":"0.014",
            "T":1568014460893
           }
    }"#;
        let ws_data = WsData::default();
        let v: LiquidationEvent = serde_json::from_str(json).unwrap();
        ws_data.add_liquidation(v.liquidation_order.to_owned());
        assert_eq!(ws_data.get_liquidations().len(), 1);
        ws_data.add_liquidation(v.liquidation_order);
        assert_eq!(ws_data.get_liquidations().len(), 2);
    }

    #[test]
    fn test_account_update() {
        let json = r#"{
        "e": "ACCOUNT_UPDATE",
        "E": 1564745798939,
        "T": 1564745798938 ,
        "a":
        {
            "m":"ORDER",
            "B":[
            {
                "a":"USDT",
                "wb":"122624.12345678",
                "cw":"100.12345678",
                "bc":"50.12345678"
            }
            ],
            "P":[
            {
                "s":"BTCUSDT",
                "pa":"0",
                "ep":"0.00000",
                "cr":"200",
                "up":"0",
                "mt":"isolated",
                "iw":"0.00000000",
                "ps":"BOTH"
            }
            ]
        }
    }"#;
        let ws_data = WsData::default();
        let v: AccountUpdateEvent = serde_json::from_str(json).unwrap();
        ws_data.update_balance(v.data.balances.first().unwrap().to_owned());
        ws_data.update_position(v.data.positions.first().unwrap().to_owned());
        assert!(ws_data.get_balance_event().is_some());
        assert!(ws_data.get_position_event().is_some());
    }

    #[test]
    fn test_max_data_size() {
        let json = r#"  {
        "e": "aggTrade",
        "E": 123456789,
        "s": "BTCUSDT",
        "a": 5933014,
        "p": "0.001",
        "q": "100",
        "f": 100,
        "l": 105,
        "T": 123456785,
        "m": true
    }"#;
        let ws_data = WsData::default();
        let v: AggrTradesEvent = serde_json::from_str(json).unwrap();
        let mut inserts: usize = 0;
        for _ in 0..=DATA_SIZE * 2 {
            ws_data.add_aggr_trades(v.to_owned());
            inserts += 1;
        }
        assert!(inserts > DATA_SIZE);
        assert_eq!(ws_data.get_aggr_trades().len(), DATA_SIZE);
    }

    #[test]
    fn test_max_data_size_index_map() {
        let json = r#"{
        "s": "BTCUSDT",
        "c": "web_HWhZes7Aql5iv5R6dEaa",
        "S": "BUY",
        "o": "LIMIT",
        "f": "GTC",
        "q": "0.010",
        "p": "15000",
        "ap": "0",
        "sp": "0",
        "x": "NEW",
        "X": "NEW",
        "i": 3252769662,
        "l": "0",
        "z": "0",
        "L": "0",
        "N": "",
        "n": "",
        "T": 1668814069559,
        "t": 0,
        "b": "150",
        "a": "0",
        "m": false,
        "R": false,
        "wt": "CONTRACT_PRICE",
        "ot": "LIMIT",
        "ps": "LONG",
        "cp": false,
        "AP": "0",
        "cr": "",
        "pP": false,
        "si": 0,
        "ss": 0,
        "rp": "0" }"#;
        let v: OrderUpdate = serde_json::from_str(json).unwrap();
        let ws_data = WsData::default();
        let mut inserts: usize = 0;
        for _ in 0..=DATA_SIZE * 2 {
            let mut v2 = v.clone();
            v2.order_id += inserts as u64;
            ws_data.add_order(v2);
            inserts += 1;
        }
        assert!(inserts > DATA_SIZE);
        assert_eq!(ws_data.get_open_orders().len(), DATA_SIZE);
        assert_eq!(
            ws_data
                .get_open_orders()
                .get(ws_data.get_open_orders().len() - 1)
                .unwrap()
                .order_id,
            v.order_id + inserts as u64 - 1
        );
    }
}
