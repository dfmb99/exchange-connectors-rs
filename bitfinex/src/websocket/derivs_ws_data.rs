use crate::rest::account::Position;
use crate::rest::candles::Candle;
use crate::rest::orders::OrderData;
use crate::rest::ticker::TradingPair;
use crate::rest::trades::TradingPair as TradesTradingPair;
use crate::websocket::model::{BalanceInfo, Wallet};
use indexmap::IndexMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

type TradingPairWs = Arc<RwLock<Option<TradingPair>>>;
type CandlesWs = Arc<RwLock<VecDeque<Candle>>>;
type TradesWs = Arc<RwLock<VecDeque<TradesTradingPair>>>;
type PositionsWs = Arc<RwLock<IndexMap<String, Position>>>;
type BalanceWs = Arc<RwLock<Option<BalanceInfo>>>;
type WalletWs = Arc<RwLock<IndexMap<String, Wallet>>>;
type OrdersWs = Arc<RwLock<IndexMap<i64, OrderData>>>;
type PriceSnapsWs = Arc<RwLock<VecDeque<TradingPair>>>;

const DATA_SIZE: usize = 1000;

#[derive(Debug)]
pub struct DerivsWsData {
    trading_pair: TradingPairWs,
    candles: CandlesWs,
    trades: TradesWs,
    positions: PositionsWs,
    balance: BalanceWs,
    wallets: WalletWs,
    filled_orders: OrdersWs,
    open_orders: OrdersWs,
    canceled_orders: OrdersWs,
    price_snaps: PriceSnapsWs,
}

impl Default for DerivsWsData {
    fn default() -> DerivsWsData {
        DerivsWsData {
            trading_pair: Arc::new(RwLock::new(None)),
            candles: Arc::new(RwLock::new(VecDeque::with_capacity(DATA_SIZE))),
            trades: Arc::new(RwLock::new(VecDeque::with_capacity(DATA_SIZE))),
            positions: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            balance: Arc::new(RwLock::new(None)),
            wallets: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            filled_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            open_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            canceled_orders: Arc::new(RwLock::new(IndexMap::with_capacity(DATA_SIZE))),
            price_snaps: Arc::new(RwLock::new(VecDeque::with_capacity(DATA_SIZE))),
        }
    }
}

impl Clone for DerivsWsData {
    fn clone(&self) -> DerivsWsData {
        DerivsWsData {
            trading_pair: Arc::clone(&self.trading_pair),
            candles: Arc::clone(&self.candles),
            trades: Arc::clone(&self.trades),
            positions: Arc::clone(&self.positions),
            balance: Arc::clone(&self.balance),
            wallets: Arc::clone(&self.wallets),
            filled_orders: Arc::clone(&self.filled_orders),
            open_orders: Arc::clone(&self.open_orders),
            canceled_orders: Arc::clone(&self.canceled_orders),
            price_snaps: Arc::clone(&self.price_snaps),
        }
    }
}

impl DerivsWsData {
    pub fn get_trading_pair(&self) -> Option<TradingPair> {
        self.trading_pair.read().unwrap().clone()
    }

    pub fn get_candles(&self) -> VecDeque<Candle> {
        self.candles.read().unwrap().clone()
    }

    pub fn get_trades(&self) -> VecDeque<TradesTradingPair> {
        self.trades.read().unwrap().clone()
    }

    pub fn get_positions(&self) -> VecDeque<Position> {
        self.positions.read().unwrap().values().cloned().collect()
    }

    pub fn get_balance(&self) -> Option<BalanceInfo> {
        self.balance.read().unwrap().clone()
    }

    pub fn get_wallet(&self) -> VecDeque<Wallet> {
        self.wallets.read().unwrap().values().cloned().collect()
    }

    pub fn get_filled_orders(&self) -> VecDeque<OrderData> {
        self.filled_orders
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    pub fn get_open_orders(&self) -> VecDeque<OrderData> {
        self.open_orders.read().unwrap().values().cloned().collect()
    }

    pub fn get_canceled_orders(&self) -> VecDeque<OrderData> {
        self.canceled_orders
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }
    pub fn get_filled_order(&self, order_id: i64) -> Option<OrderData> {
        let filled_orders = self.filled_orders.read().unwrap();
        get_value_index_map(filled_orders, order_id)
    }

    pub fn get_open_order(&self, order_id: i64) -> Option<OrderData> {
        let open_orders = self.open_orders.read().unwrap();
        get_value_index_map(open_orders, order_id)
    }

    pub fn get_canceled_order(&self, order_id: i64) -> Option<OrderData> {
        let canceled_orders: RwLockReadGuard<IndexMap<i64, OrderData>> =
            self.canceled_orders.read().unwrap();
        get_value_index_map(canceled_orders, order_id)
    }

    pub fn update_trading_pair(&self, event: TradingPair) {
        let mut trading_pair: RwLockWriteGuard<Option<TradingPair>> =
            self.trading_pair.write().unwrap();
        *trading_pair = Some(event);
    }

    pub fn add_candle(&self, event: Candle) {
        insert_vec(self.candles.write().unwrap(), event);
    }

    pub fn add_trade(&self, event: TradesTradingPair) {
        insert_vec(self.trades.write().unwrap(), event);
    }

    pub fn add_position(&self, event: Position) {
        let symbol = &event.symbol;
        insert_value_index_map(self.positions.write().unwrap(), symbol.to_owned(), event);
    }

    pub fn update_balance(&self, event: BalanceInfo) {
        let mut balance: RwLockWriteGuard<Option<BalanceInfo>> = self.balance.write().unwrap();
        *balance = Some(event);
    }

    pub fn add_wallet(&self, event: Wallet) {
        let currency = &event.currency;
        insert_value_index_map(self.wallets.write().unwrap(), currency.to_owned(), event);
    }

    pub fn add_price_snap(&self, event: TradingPair) {
        insert_vec(self.price_snaps.write().unwrap(), event);
    }

    pub fn add_order(&self, order: OrderData) {
        let order_id = order.id;
        let order_status = order.clone().order_status;

        if order_status == "ACTIVE" {
            insert_value_index_map(self.open_orders.write().unwrap(), order_id, order);
        } else if order_status.contains("EXECUTED") {
            insert_value_index_map(self.filled_orders.write().unwrap(), order_id, order);
            // this order could be previously open so needs to be removed from open orders
            remove_value_index_map(self.open_orders.write().unwrap(), order_id);
        } else if order_status == "CANCELED" {
            insert_value_index_map(self.canceled_orders.write().unwrap(), order_id, order);
            // this order could be previously open so needs to be removed from open orders
            remove_value_index_map(self.open_orders.write().unwrap(), order_id);
        }
    }
}

fn insert_value_index_map<T, V>(mut index_map: RwLockWriteGuard<IndexMap<T, V>>, key: T, value: V)
where
    T: Eq,
    T: Hash,
{
    if index_map.insert(key, value).is_none() && index_map.len() == DATA_SIZE + 1 {
        index_map.pop();
    }
}

fn remove_value_index_map<T, V>(mut index_map: RwLockWriteGuard<IndexMap<T, V>>, key: T)
where
    T: Eq,
    T: Hash,
{
    index_map.shift_remove(&key);
}

fn insert_vec<T>(mut vec: RwLockWriteGuard<VecDeque<T>>, value: T) {
    vec.push_back(value);
    if vec.len() > DATA_SIZE {
        vec.pop_front();
    }
}

fn get_value_index_map<T, V>(index_map: RwLockReadGuard<IndexMap<T, V>>, key: T) -> Option<V>
where
    T: Eq,
    T: Hash,
    V: Clone,
{
    let result: Option<&V> = index_map.get(&key);
    result.cloned()
}

#[cfg(test)]
mod tests {}
