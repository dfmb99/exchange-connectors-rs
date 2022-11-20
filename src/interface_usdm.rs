use std::collections::BTreeMap;
use std::convert::TryInto;
use serde_json::Value;
use crate::account::{OrderSide, TimeInForce};
use crate::api::{API, Futures};
use crate::client::Client;
use crate::config::Config;
use crate::futures::account::{CustomOrderRequest, OrderType, OrderRequest};
use crate::interface_usdm_data::UsdmData;
use crate::ws_usdm::WsInterface;
use crate::errors::*;
use crate::futures::model::{AccountBalance, AccountInformation, AggTrades, BookTickers, CanceledOrder, ChangeLeverageResponse, ExchangeInformation, KlineSummaries, KlineSummary, LiquidationOrders, MarkPrices, OpenInterest, OpenInterestHist, OrderBook, PositionRisk, PriceStats, Symbol, SymbolPrice, Tickers, Trades, Transaction};
use crate::model::{Empty, ServerTime};
use crate::util::{build_request, build_signed_request};

struct UsdmInterface {
    symbol: String,
    client: Client,
    recv_window: u64,
    ws: WsInterface,
    data: UsdmData,
}

impl UsdmInterface {
    /// Binance USDM futures interface,
    /// subscribes to @aggTrade, @markPrice@1s and @forceOrder and user data stream
    /// * `symbol` - String
    /// * `api_key` - Option<String>
    /// * `api_secret` - Option<String>
    /// * `config` - Config
    pub fn new(symbol: String, api_key: Option<String>, api_secret: Option<String>, config: &Config) -> UsdmInterface {
        let client = Client::new(
            api_key.to_owned(),
            api_secret.to_owned(),
            config.futures_rest_api_endpoint.clone(),
        );
        UsdmInterface {
            symbol: symbol.to_owned(),
            client,
            recv_window: config.recv_window,
            ws: WsInterface::new(symbol.to_owned(), api_key.to_owned(), api_secret.to_owned(), config),
            data: UsdmData::default()
        }
    }

    /// Test connectivity
    pub fn ping(&self) -> Result<String> {
        self.client.get(API::Futures(Futures::Ping), None)?;
        Ok("pong".into())
    }

    /// Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get(API::Futures(Futures::Time), None)
    }

    /// Obtain exchange information
    /// - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get(API::Futures(Futures::ExchangeInfo), None)
    }

    /// Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
        where
            S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info() {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                bail!("Symbol not found")
            }
            Err(e) => Err(e),
        }
    }

    /// Order book (Default 100; max 1000)
    pub fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
        where
            S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client.get(API::Futures(Futures::Depth), Some(request))
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
        self.client.get(API::Futures(Futures::Depth), Some(request))
    }

    /// Get trades
    pub fn get_trades<S>(&self, symbol: S) -> Result<Trades>
        where
            S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::Trades), Some(request))
    }

    /// Get historical trades
    pub fn get_historical_trades<S1, S2, S3>(
        &self, symbol: S1, from_id: S2, limit: S3,
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
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{}", fi));
        }

        let request = build_signed_request(parameters, self.recv_window)?;

        self.client
            .get_signed(API::Futures(Futures::HistoricalTrades), Some(request))
    }

    /// Get aggr trades
    pub fn get_agg_trades<S1, S2, S3, S4, S5>(
        &self, symbol: S1, from_id: S2, start_time: S3, end_time: S4, limit: S5,
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
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{}", fi));
        }

        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::AggTrades), Some(request))
    }

    /// Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    /// https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub fn get_klines<S1, S2, S3, S4, S5>(
        &self, symbol: S1, interval: S2, limit: S3, start_time: S4, end_time: S5,
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
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }

        let request = build_request(parameters);

        let data: Vec<Vec<Value>> = self
            .client
            .get(API::Futures(Futures::Klines), Some(request))?;

        let klines = KlineSummaries::AllKlineSummaries(
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

        self.client
            .get(API::Futures(Futures::Ticker24hr), Some(request))
    }

    /// 24hr ticker price change statistics for all symbols
    pub fn get_all_24h_price_stats(&self) -> Result<Vec<PriceStats>> {
        self.client.get(API::Futures(Futures::Ticker24hr), None)
    }

    /// Latest price for ONE symbol.
    pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
        where
            S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::TickerPrice), Some(request))
    }

    /// Latest price for all symbols.
    pub fn get_all_prices(&self) -> Result<crate::model::Prices> {
        self.client.get(API::Futures(Futures::TickerPrice), None)
    }

    /// Symbols order book ticker
    /// -> Best price/qty on the order book for ALL symbols.
    pub fn get_all_book_tickers(&self) -> Result<BookTickers> {
        self.client.get(API::Futures(Futures::BookTicker), None)
    }

    /// -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
        where
            S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::BookTicker), Some(request))
    }

    /// Get mark prices
    pub fn get_mark_prices(&self) -> Result<MarkPrices> {
        self.client.get(API::Futures(Futures::PremiumIndex), None)
    }

    /// Get all liquidation orders
    pub fn get_all_liquidation_orders(&self) -> Result<LiquidationOrders> {
        self.client.get(API::Futures(Futures::AllForceOrders), None)
    }

    /// Get open interest data
    pub fn open_interest<S>(&self, symbol: S) -> Result<OpenInterest>
        where
            S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::OpenInterest), Some(request))
    }

    /// Get open interest statistics
    pub fn open_interest_statistics<S1, S2, S3, S4, S5>(
        &self, symbol: S1, period: S2, limit: S3, start_time: S4, end_time: S5,
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
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }

        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::OpenInterestHist), Some(request))
    }

    /// Place limit buy order
    pub fn limit_buy(
        &self, symbol: impl Into<String>, qty: impl Into<f64>, price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let buy = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
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
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    /// Place limit sell order
    pub fn limit_sell(
        &self, symbol: impl Into<String>, qty: impl Into<f64>, price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let sell = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
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
        self.client
            .post_signed(API::Futures(Futures::Order), request)
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
        self.client
            .post_signed(API::Futures(Futures::Order), request)
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
        self.client
            .post_signed(API::Futures(Futures::Order), request)
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
        self.client
            .delete_signed(API::Futures(Futures::Order), Some(request))
    }

    /// Cancel an order with a given client id
    pub fn cancel_order_with_client_id<S>(
        &self, symbol: S, orig_client_order_id: String,
    ) -> Result<CanceledOrder>
        where
            S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), orig_client_order_id);

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::Order), Some(request))
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
        self.client
            .post_signed(API::Futures(Futures::Order), request)
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
        self.client
            .post_signed(API::Futures(Futures::Order), request)
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
        self.client
            .post_signed(API::Futures(Futures::Order), request)
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
        self.client
            .get_signed(API::Futures(Futures::PositionRisk), Some(request))
    }

    /// Get account information
    pub fn account_information(&self) -> Result<AccountInformation> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Account), Some(request))
    }

    /// Get account balance
    pub fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Balance), Some(request))
    }

    /// Change initial leverage
    pub fn change_initial_leverage<S>(
        &self, symbol: S, leverage: u8,
    ) -> Result<ChangeLeverageResponse>
        where
            S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::ChangeInitialLeverage), request)
    }

    /// Change position mode
    pub fn change_position_mode(&self, dual_side_position: bool) -> Result<()> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let dual_side = if dual_side_position { "true" } else { "false" };
        parameters.insert("dualSidePosition".into(), dual_side.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Futures(Futures::PositionSide), request)
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
        self.client
            .delete_signed::<Empty>(API::Futures(Futures::AllOpenOrders), Some(request))
            .map(|_| ())
    }

    /// Get all open orders
    pub fn get_all_open_orders<S>(&self, symbol: S) -> Result<Vec<crate::futures::model::Order>>
        where
            S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::OpenOrders), Some(request))
    }

}

fn update_last_day_klines(data: UsdmData) {

}
