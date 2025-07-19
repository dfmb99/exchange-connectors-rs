use crate::commons::config::Config;
use crate::commons::errors::*;
use crate::commons::util::{build_request, build_signed_request};
use crate::interfaces::usdm_data::{UsdmConfig, UsdmData};
use crate::rest::api::{Futures, API};
use crate::rest::client::Client;
use crate::rest::futures::account::{CustomOrderRequest, OrderRequest, OrderType};
use crate::rest::futures::model::{
    AccountBalance, AccountInformation, AggTrades, CanceledOrder, ChangeLeverageResponse,
    ComissionRate, ExchangeInformation, FundingRateHist, LiquidationOrders, MarkPrice, MarkPrices,
    OpenInterest, OpenInterestHist, Order, OrderBook, OrderUpdate, PositionRisk, PriceStats,
    Symbol, Trades, Transaction,
};
use crate::rest::model::KlineSummaries::AllKlineSummaries;
use crate::rest::model::{
    AggrTradesEvent, BookTickers, Empty, EventBalance, EventPosition, IndexPriceEvent,
    KlineSummaries, KlineSummary, LiquidationOrder, Prices, ServerTime, SymbolPrice, Tickers,
};
use crate::rest::spot::account::{OrderSide, TimeInForce};
use crate::websocket::futures::usdm::WsInterface;
use log::error;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::{BTreeMap, VecDeque};
use std::convert::TryInto;
use std::thread;
use std::time::Duration;

enum RequestType {
    Get,
    GetSigned,
    PostSigned,
    DeleteSigned,
}

#[derive(Clone)]
pub struct UsdmInterface {
    symbol: String,
    pub api: Client,
    recv_window: u64,
    pub ws: WsInterface,
    data: UsdmData,
    config: UsdmConfig,
}

impl UsdmInterface {
    /// Binance USDM futures interface,
    /// subscribes to @aggTrade, @markPrice@1s and @forceOrder and user data stream
    /// * `symbol` - String
    /// * `api_key` - Option<String>
    /// * `api_secret` - Option<String>
    /// * `config` - Config
    pub fn new(
        symbol: String,
        api_key: Option<String>,
        api_secret: Option<String>,
        client_config: &Config,
        config: UsdmConfig,
    ) -> UsdmInterface {
        let client = Client::new(
            api_key.to_owned(),
            api_secret.to_owned(),
            client_config.futures_rest_api_endpoint.clone(),
        );
        let usdm_int = UsdmInterface {
            symbol: symbol.to_owned(),
            api: client,
            recv_window: client_config.recv_window,
            ws: WsInterface::new(symbol, api_key, api_secret, client_config),
            data: UsdmData::default(),
            config,
        };
        update_usdm_data(usdm_int.to_owned());
        usdm_int.wait_for_data();
        usdm_int
    }

    fn wait_for_data(&self) {
        loop {
            let AllKlineSummaries(k_lines) = self.get_last_day_klines();
            if !k_lines.is_empty() {
                break;
            }
            thread::yield_now();
        }
    }

    /// Get last day "1m" klines
    pub fn get_last_day_klines(&self) -> KlineSummaries {
        self.data.get_last_day_klines()
    }

    /// Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        self.api_request(Futures::Time, RequestType::Get, None)
    }

    /// Obtain exchange information
    /// - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.api_request(Futures::ExchangeInfo, RequestType::Get, None)
    }

    /// Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        let info = self.exchange_info()?;
        info.symbols
            .into_iter()
            .find(|item| item.symbol == upper_symbol)
            .ok_or(BinanceError::SymbolNotFound)
    }

    /// Order book (Default 100; max 1000)
    pub fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.api_request(Futures::Depth, RequestType::Get, Some(request))
    }

    /// Order book at a custom depth. Currently supported values
    /// are 5, 10, 20, 50, 100, 500, 1000
    pub fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("limit".into(), depth.to_string());
        let request = build_request(parameters);
        self.api_request(Futures::Depth, RequestType::Get, Some(request))
    }

    /// Get trades
    pub fn get_trades<S>(&self, symbol: S) -> Result<Trades>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.api_request(Futures::Trades, RequestType::Get, Some(request))
    }

    /// Get historical trades
    pub fn get_historical_trades<S1, S2, S3>(
        &self,
        symbol: S1,
        from_id: S2,
        limit: S3,
    ) -> Result<Trades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{lt}"));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{fi}"));
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(
            Futures::HistoricalTrades,
            RequestType::GetSigned,
            Some(request),
        )
    }

    /// Get aggr trades
    pub fn get_agg_trades<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        from_id: S2,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<AggTrades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{lt}"));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{st}"));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{et}"));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{fi}"));
        }

        let request = build_request(parameters);
        self.api_request(Futures::AggTrades, RequestType::Get, Some(request))
    }

    /// Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    /// https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub fn get_klines<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{lt}"));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{st}"));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{et}"));
        }

        let request = build_request(parameters);
        let data: Vec<Vec<Value>> =
            self.api_request(Futures::Klines, RequestType::Get, Some(request))?;

        let klines = AllKlineSummaries(
            data.iter()
                .map(|row| row.try_into())
                .collect::<Result<Vec<KlineSummary>>>()?,
        );

        Ok(klines)
    }

    /// 24hr ticker price change statistics
    pub fn get_24h_price_stats<S>(&self, symbol: S) -> Result<PriceStats>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.api_request(Futures::Ticker24hr, RequestType::Get, Some(request))
    }

    /// 24hr ticker price change statistics for all symbols
    pub fn get_all_24h_price_stats(&self) -> Result<Vec<PriceStats>> {
        self.api_request(Futures::Ticker24hr, RequestType::Get, None)
    }

    /// Latest price for ONE symbol.
    pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.api_request(Futures::TickerPrice, RequestType::Get, Some(request))
    }

    /// Latest price for all symbols.
    pub fn get_all_prices(&self) -> Result<Prices> {
        self.api_request(Futures::TickerPrice, RequestType::Get, None)
    }

    /// Symbols order book ticker
    /// -> Best price/qty on the order book for ALL symbols.
    pub fn get_all_book_tickers(&self) -> Result<BookTickers> {
        self.api_request(Futures::BookTicker, RequestType::Get, None)
    }

    /// -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.api_request(Futures::BookTicker, RequestType::Get, Some(request))
    }

    /// Get mark prices
    pub fn get_mark_prices(&self) -> Result<MarkPrices> {
        self.api_request(Futures::PremiumIndex, RequestType::Get, None)
    }

    /// Get mark price
    pub fn get_mark_price<S>(&self, symbol: S) -> Result<MarkPrice>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.api_request(Futures::PremiumIndex, RequestType::Get, Some(request))
    }

    /// Get all liquidation orders
    pub fn get_all_liquidation_orders(&self) -> Result<LiquidationOrders> {
        self.api_request(Futures::AllForceOrders, RequestType::Get, None)
    }

    /// Get open interest data
    pub fn open_interest<S>(&self, symbol: S) -> Result<OpenInterest>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.api_request(Futures::OpenInterest, RequestType::Get, Some(request))
    }

    /// Get open interest statistics
    pub fn open_interest_statistics<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        period: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<Vec<OpenInterestHist>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("period".into(), period.into());

        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{lt}"));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{st}"));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{et}"));
        }

        let request = build_request(parameters);
        self.api_request(Futures::OpenInterestHist, RequestType::Get, Some(request))
    }

    /// Get funding rate history
    pub fn funding_rate_history<S1, S2, S3, S4>(
        &self,
        symbol: S1,
        limit: S2,
        start_time: S3,
        end_time: S4,
    ) -> Result<Vec<FundingRateHist>>
    where
        S1: Into<Option<String>>,
        S2: Into<Option<i32>>,
        S3: Into<Option<i64>>,
        S4: Into<Option<i64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(lt) = symbol.into() {
            parameters.insert("symbol".into(), lt);
        }
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), lt.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }

        let request = build_request(parameters);
        self.api_request(Futures::FundingRate, RequestType::Get, Some(request))
    }

    /// Get comission rate
    pub fn get_comission_rate<S>(&self, symbol: S, timestamp: S) -> Result<ComissionRate>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("timestamp".into(), timestamp.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(Futures::ComissionRate, RequestType::Get, Some(request))
    }

    /// Place limit buy order
    pub fn limit_buy(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<f64>,
        price: f64,
    ) -> Result<Transaction> {
        let buy = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            qty: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::PostSigned, Some(request))
    }

    /// Place limit sell order
    pub fn limit_sell(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<f64>,
        price: f64,
    ) -> Result<Transaction> {
        let sell = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            qty: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::PostSigned, Some(request))
    }

    /// Place a MARKET order - BUY
    pub fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            qty: Some(qty.into()),
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::PostSigned, Some(request))
    }

    /// Place a MARKET order - SELL
    pub fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            qty: Some(qty.into()),
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::PostSigned, Some(request))
    }

    /// Cancel an order
    pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<CanceledOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::DeleteSigned, Some(request))
    }

    /// Cancel an order with a given client id
    pub fn cancel_order_with_client_id<S>(
        &self,
        symbol: S,
        orig_client_order_id: String,
    ) -> Result<CanceledOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), orig_client_order_id);

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::DeleteSigned, Some(request))
    }

    /// Place a STOP_MARKET close - BUY
    pub fn stop_market_close_buy<S, F>(&self, symbol: S, stop_price: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::StopMarket,
            time_in_force: None,
            qty: None,
            reduce_only: None,
            price: None,
            stop_price: Some(stop_price.into()),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::PostSigned, Some(request))
    }

    /// Place a STOP_MARKET close - SELL
    pub fn stop_market_close_sell<S, F>(&self, symbol: S, stop_price: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::StopMarket,
            time_in_force: None,
            qty: None,
            reduce_only: None,
            price: None,
            stop_price: Some(stop_price.into()),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::PostSigned, Some(request))
    }

    /// Custom order for for professional traders
    pub fn custom_order(&self, order_request: CustomOrderRequest) -> Result<Transaction> {
        let order: OrderRequest = OrderRequest {
            symbol: order_request.symbol,
            side: order_request.side,
            position_side: order_request.position_side,
            order_type: order_request.order_type,
            time_in_force: order_request.time_in_force,
            qty: order_request.qty,
            reduce_only: order_request.reduce_only,
            price: order_request.price,
            stop_price: order_request.stop_price,
            close_position: order_request.close_position,
            activation_price: order_request.activation_price,
            callback_rate: order_request.callback_rate,
            working_type: order_request.working_type,
            price_protect: order_request.price_protect,
        };
        let order = self.build_order(order);
        let request = build_signed_request(order, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::PostSigned, Some(request))
    }

    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), order.symbol);
        parameters.insert("side".into(), order.side.to_string());
        parameters.insert("type".into(), order.order_type.to_string());

        if let Some(position_side) = order.position_side {
            parameters.insert("positionSide".into(), position_side.to_string());
        }
        if let Some(time_in_force) = order.time_in_force {
            parameters.insert("timeInForce".into(), time_in_force.to_string());
        }
        if let Some(qty) = order.qty {
            parameters.insert("quantity".into(), qty.to_string());
        }
        if let Some(reduce_only) = order.reduce_only {
            parameters.insert("reduceOnly".into(), reduce_only.to_string().to_uppercase());
        }
        if let Some(price) = order.price {
            parameters.insert("price".into(), price.to_string());
        }
        if let Some(stop_price) = order.stop_price {
            parameters.insert("stopPrice".into(), stop_price.to_string());
        }
        if let Some(close_position) = order.close_position {
            parameters.insert(
                "closePosition".into(),
                close_position.to_string().to_uppercase(),
            );
        }
        if let Some(activation_price) = order.activation_price {
            parameters.insert("activationPrice".into(), activation_price.to_string());
        }
        if let Some(callback_rate) = order.callback_rate {
            parameters.insert("callbackRate".into(), callback_rate.to_string());
        }
        if let Some(working_type) = order.working_type {
            parameters.insert("workingType".into(), working_type.to_string());
        }
        if let Some(price_protect) = order.price_protect {
            parameters.insert(
                "priceProtect".into(),
                price_protect.to_string().to_uppercase(),
            );
        }

        parameters
    }

    /// Get position_information
    pub fn position_information<S>(&self, symbol: S) -> Result<Vec<PositionRisk>>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(Futures::PositionRisk, RequestType::GetSigned, Some(request))
    }

    /// Get account information
    pub fn account_information(&self) -> Result<AccountInformation> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(Futures::Account, RequestType::GetSigned, Some(request))
    }

    /// Get account balance
    pub fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(Futures::Balance, RequestType::GetSigned, Some(request))
    }

    /// Change initial leverage
    pub fn change_initial_leverage<S>(
        &self,
        symbol: S,
        leverage: u8,
    ) -> Result<ChangeLeverageResponse>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(
            Futures::ChangeInitialLeverage,
            RequestType::PostSigned,
            Some(request),
        )
    }

    /// Change position mode
    pub fn change_position_mode(&self, dual_side_position: bool) -> Result<()> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let dual_side = if dual_side_position { "true" } else { "false" };
        parameters.insert("dualSidePosition".into(), dual_side.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request::<Empty>(
            Futures::PositionSide,
            RequestType::PostSigned,
            Some(request),
        )
        .map(|_| ())
    }

    /// Cancel all orders
    pub fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request::<Empty>(
            Futures::AllOpenOrders,
            RequestType::DeleteSigned,
            Some(request),
        )
        .map(|_| ())
    }

    /// Get an order
    pub fn get_order<S>(&self, symbol: S, order_id: u64) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(Futures::Order, RequestType::GetSigned, Some(request))
    }

    /// Get all open orders
    pub fn get_all_open_orders<S>(&self, symbol: S) -> Result<Vec<Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.api_request(Futures::OpenOrders, RequestType::GetSigned, Some(request))
    }

    /// Get mark price from websocket
    pub fn get_mark_price_ws(&self) -> Option<f64> {
        if let Some(event) = self.ws.get_mark_price() {
            return Some(event.price.parse().unwrap());
        }
        None
    }

    /// Get mark price from websocket
    pub fn get_mark_price_snaps_ws(&self) -> VecDeque<IndexPriceEvent> {
        self.ws.get_mark_price_snaps()
    }

    /// Get liquidations
    pub fn get_liquidations_ws(&self) -> VecDeque<LiquidationOrder> {
        self.ws.get_liquidations()
    }

    /// Get aggr_trades
    pub fn get_aggr_trades_ws(&self) -> VecDeque<AggrTradesEvent> {
        self.ws.get_aggr_trades()
    }

    /// Get last price from websocket
    pub fn get_last_price_ws(&self) -> Option<f64> {
        let aggr_trades = self.ws.get_aggr_trades();
        if !aggr_trades.is_empty() {
            return Some(aggr_trades[0].price.parse().unwrap());
        }
        None
    }

    /// Get position
    pub fn get_position_ws(&self) -> Option<EventPosition> {
        self.ws.get_position()
    }

    /// Get balance
    pub fn get_balance_ws(&self) -> Option<EventBalance> {
        self.ws.get_balance()
    }

    /// Get open orders
    pub fn get_open_orders_ws(&self) -> VecDeque<OrderUpdate> {
        self.ws.get_open_orders()
    }

    /// Get filled orders
    pub fn get_filled_orders_ws(&self) -> VecDeque<OrderUpdate> {
        self.ws.get_filled_orders()
    }

    /// Get canceled orders
    pub fn get_canceled_orders_ws(&self) -> VecDeque<OrderUpdate> {
        self.ws.get_canceled_orders()
    }

    /// Returns true of order is open, false otherwise
    pub fn is_open_orders_ws(&self, order_id: u64) -> bool {
        self.get_open_orders_ws()
            .into_iter()
            .filter(|ord| ord.order_id == order_id)
            .count()
            > 0
    }

    /// Returns true if order is filled, false otherwise
    pub fn is_filled_orders_ws(&self, order_id: u64) -> bool {
        self.get_filled_orders_ws()
            .into_iter()
            .filter(|ord| ord.order_id == order_id)
            .count()
            > 0
    }

    /// Get canceled orders, false otherwise
    pub fn is_canceled_orders_ws(&self, order_id: u64) -> bool {
        self.get_canceled_orders_ws()
            .into_iter()
            .filter(|ord| ord.order_id == order_id)
            .count()
            > 0
    }

    /// Get order
    ///
    /// * `order_id` - id of order
    pub fn get_order_ws(&self, order_id: u64) -> Option<OrderUpdate> {
        self.ws.get_order(order_id)
    }

    /// Get last order filled
    pub fn get_last_filled_order(&self) -> Option<OrderUpdate> {
        let filled_orders = self.ws.get_filled_orders();
        if !filled_orders.is_empty() {
            return Some(filled_orders[0].to_owned());
        }
        None
    }

    /// Get last order canceled
    pub fn get_last_canceled_order_ws(&self) -> Option<OrderUpdate> {
        let canceled_orders = self.ws.get_canceled_orders();
        if !canceled_orders.is_empty() {
            return Some(canceled_orders[0].to_owned());
        }
        None
    }

    /// Get last open order
    pub fn get_last_open_order_ws(&self) -> Option<OrderUpdate> {
        let open_orders = self.ws.get_open_orders();
        if !open_orders.is_empty() {
            return Some(open_orders[0].to_owned());
        }
        None
    }

    /// Get position size
    pub fn get_position_size_ws(&self) -> Option<f64> {
        if let Some(position) = self.get_position_ws() {
            return Some(position.position_amount.parse().unwrap());
        }
        None
    }

    /// Get position entry price
    pub fn get_position_entry_ws(&self) -> Option<f64> {
        if let Some(position) = self.get_position_ws() {
            return Some(position.entry_price.parse().unwrap());
        }
        None
    }

    /// Get position entry price
    pub fn get_position_upnl_ws(&self) -> Option<f64> {
        if let Some(position) = self.get_position_ws() {
            return Some(position.unrealized_pnl.parse().unwrap());
        }
        None
    }

    /// Get balance asset
    pub fn get_balance_asset_ws(&self) -> Option<String> {
        if let Some(balance) = self.get_balance_ws() {
            return Some(balance.asset);
        }
        None
    }

    /// Get balance wallet
    pub fn get_balance_wallet_ws(&self) -> Option<f64> {
        if let Some(balance) = self.get_balance_ws() {
            return Some(balance.wallet_balance.parse().unwrap());
        }
        None
    }

    /// Get balance cross wallet
    pub fn get_balance_cross_wallet_ws(&self) -> Option<f64> {
        if let Some(balance) = self.get_balance_ws() {
            return Some(balance.cross_wallet_balance.parse().unwrap());
        }
        None
    }

    fn api_request<T: DeserializeOwned>(
        &self,
        endpoint: Futures,
        req_type: RequestType,
        req: Option<String>,
    ) -> Result<T> {
        let result: Result<T> = match req_type {
            RequestType::Get => self.api.get(API::Futures(endpoint), req.to_owned()),
            RequestType::GetSigned => self.api.get_signed(API::Futures(endpoint), req.to_owned()),
            RequestType::PostSigned => self
                .api
                .post_signed(API::Futures(endpoint), req.to_owned().unwrap()),
            RequestType::DeleteSigned => self
                .api
                .delete_signed(API::Futures(endpoint), req.to_owned()),
        };

        if result.is_err() && self.config.retry_on_err {
            self.err_handler(endpoint, req_type, req, result)
        } else {
            result
        }
    }

    fn err_handler<T: DeserializeOwned>(
        &self,
        endpoint: Futures,
        req_type: RequestType,
        req: Option<String>,
        result: Result<T>,
    ) -> Result<T> {
        match &result {
            Err(BinanceError::BinanceError { response: err }) => {
                if err.msg == "Request occur unknown error."
                    || err.msg == "Service Unavailable."
                    || err.msg
                        == "Internal error; unable to process your request. Please try again."
                {
                    thread::sleep(Duration::from_millis(self.config.retry_timeout));
                    return self.api_request(endpoint, req_type, req);
                }
                result
            }
            Err(BinanceError::RequestError(_)) => {
                thread::sleep(Duration::from_millis(self.config.retry_timeout));
                self.api_request(endpoint, req_type, req)
            }
            _ => result,
        }
    }
}

fn update_usdm_data(mut usdm_int: UsdmInterface) {
    thread::spawn(move || loop {
        match usdm_int.get_klines(usdm_int.symbol.to_owned(), "1m", 1440, None, None) {
            Ok(kline_data) => {
                usdm_int.data.set_last_day_klines(kline_data);
                thread::sleep(Duration::from_millis(usdm_int.config.rest_update_interval));
            }
            Err(err) => {
                error!("{err:?}");
                thread::sleep(Duration::from_millis(usdm_int.config.retry_timeout));
            }
        }
    });
}
