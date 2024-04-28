use crate::commons::client::Client;
use crate::commons::errors::*;
use crate::commons::utils::InstType;
use crate::rest::api::MarketData::{
    GetCandles, GetCandlesHist, GetIndexCandles, GetIndexCandlesHist, GetIndexTickers,
    GetMarkPriceCandles, GetOrderBook, GetOrderBookLite, GetTicker, GetTickers, GetTrades,
    GetTradesHist,
};
use crate::rest::api::{ApiResponse, API};
use crate::serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MarketData {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TickersParams {
    /// Instrument type SPOT, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Underlying, e.g. BTC-USD Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family. Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
}

impl TickersParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TickerParams {
    /// Instrument ID, e.g. BTC-USD-SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,
}

impl TickerParams {
    pub fn to_query(&self) -> String {
        format!("{}={}", "instId", self.inst_id,)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickersResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    pub last: String,
    #[serde(rename = "lastSz")]
    pub last_sz: String,
    #[serde(rename = "askPx")]
    pub ask_px: String,
    #[serde(rename = "askSz")]
    pub ask_sz: String,
    #[serde(rename = "bidPx")]
    pub bid_px: String,
    #[serde(rename = "bidSz")]
    pub bid_sz: String,
    pub open24h: String,
    pub high24h: String,
    pub low24h: String,
    #[serde(rename = "volCcy24h")]
    pub vol_ccy24h: String,
    pub vol24h: String,
    #[serde(rename = "sodUtc0")]
    pub sod_utc0: String,
    #[serde(rename = "sodUtc8")]
    pub sod_utc8: String,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct IndexTickerParams {
    /// Quote currency
    /// Currently there is only an index with USD/USDT/BTC as the quote currency.
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: Option<String>,
    /// Index, e.g. BTC-USD
    /// Either quoteCcy or instId is required.
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
}

impl IndexTickerParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}",
            "quoteCcy",
            self.quote_ccy.to_owned().unwrap_or_default(),
            "instId",
            self.inst_id.to_owned().unwrap_or_default()
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrderBookParams {
    /// Instrument ID, e.g. BTC-USDT
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Order book depth per side. Maximum 400, e.g. 400 bids + 400 asks
    /// Default returns to 1 depth data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
}

impl OrderBookParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}",
            "instId",
            self.inst_id,
            "sz",
            self.sz.to_owned().unwrap_or_default()
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookResponse {
    /// Order book on sell side
    pub asks: Vec<OrderBookData>,
    /// Order book on buy side
    pub bids: Vec<OrderBookData>,
    /// Order book generation time
    pub ts: String,
}

/// An example of the array of asks and bids values: ["411.8", "10", "0", "4"]
/// - "411.8" is the depth price
/// - "10" is the quantity at the price (number of contracts for derivatives, quantity in base currency for Spot and Spot Margin)
/// - "0" is part of a deprecated feature and it is always "0"
/// - "4" is the number of orders at the price.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct OrderBookData {
    pub price: String,
    pub qty: String,
    pub placeholder: String,
    pub num_orders: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexTickerResponse {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "idxPx")]
    pub idx_px: String,
    pub high24h: String,
    pub low24h: String,
    pub open24h: String,
    #[serde(rename = "sodUtc0")]
    pub sod_utc0: String,
    #[serde(rename = "sodUtc8")]
    pub sod_utc8: String,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CandleSticksParams {
    /// Instrument ID, e.g. BTC-USD-190927-5000-C
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Bar size, the default is 1m
    /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
    /// Hong Kong time opening price k-line：[6H/12H/1D/2D/3D/1W/1M/3M]
    /// UTC time opening price k-line：[/6Hutc/12Hutc/1Dutc/2Dutc/3Dutc/1Wutc/1Mutc/3Mutc]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<String>,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. The maximum is 300. The default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl CandleSticksParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}",
            "instId",
            self.inst_id,
            "bar",
            self.bar.to_owned().unwrap_or_default(),
            "after",
            self.after.to_owned().unwrap_or_default(),
            "before",
            self.before.to_owned().unwrap_or_default(),
            "limit",
            self.limit.to_owned().unwrap_or_default(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandleSticksResponse {
    pub ts: String,
    pub o: String,
    pub h: String,
    pub l: String,
    pub c: String,
    pub vol: String,
    #[serde(rename = "volCcy")]
    pub vol_ccy: String,
    #[serde(rename = "volCcyQuote")]
    pub vol_ccy_quote: String,
    pub confirm: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandleStickNoVolResponse {
    pub ts: String,
    pub o: String,
    pub h: String,
    pub l: String,
    pub c: String,
    pub confirm: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TradesParams {
    /// Instrument ID, e.g. BTC-USDT
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Number of results per request. The maximum is 500; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl TradesParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}",
            "instId",
            self.inst_id,
            "limit",
            self.limit.to_owned().unwrap_or_default(),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TradesHistParams {
    /// Instrument ID, e.g. BTC-USDT
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Pagination Type
    /// 1: tradeId 2: timestamp
    /// The default is 1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub pagination_type: Option<String>,
    /// Pagination of data to return records earlier than the requested tradeId or ts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested tradeId.
    /// Do not support timestamp for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. The maximum and default both are 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl TradesHistParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}",
            "instId",
            self.inst_id,
            "type",
            self.pagination_type.to_owned().unwrap_or_default(),
            "after",
            self.after.to_owned().unwrap_or_default(),
            "before",
            self.before.to_owned().unwrap_or_default(),
            "limit",
            self.limit.to_owned().unwrap_or_default(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradesResponse {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub ts: String,
}

impl MarketData {
    /// Retrieve the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    pub fn get_tickers(&self, params: &TickersParams) -> Result<ApiResponse<Vec<TickersResponse>>> {
        let tickers: ApiResponse<Vec<TickersResponse>> = self
            .client
            .get(API::MarketData(GetTickers), Some(params.to_query()))?;

        Ok(tickers)
    }

    /// Retrieve the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    pub fn get_ticker(&self, params: &TickerParams) -> Result<ApiResponse<Vec<TickersResponse>>> {
        let ticker: ApiResponse<Vec<TickersResponse>> = self
            .client
            .get(API::MarketData(GetTicker), Some(params.to_query()))?;

        Ok(ticker)
    }

    /// Retrieve index tickers.
    pub fn get_index_tickers(
        &self,
        params: &IndexTickerParams,
    ) -> Result<ApiResponse<Vec<IndexTickerResponse>>> {
        let index_tickers: ApiResponse<Vec<IndexTickerResponse>> = self
            .client
            .get(API::MarketData(GetIndexTickers), Some(params.to_query()))?;

        Ok(index_tickers)
    }

    /// Retrieve order book of the instrument.
    pub fn get_order_book(
        &self,
        params: &OrderBookParams,
    ) -> Result<ApiResponse<Vec<OrderBookResponse>>> {
        let order_book: ApiResponse<Vec<OrderBookResponse>> = self
            .client
            .get(API::MarketData(GetOrderBook), Some(params.to_query()))?;

        Ok(order_book)
    }

    /// Retrieve order top 25 book of the instrument more quickly.
    pub fn get_order_book_lite(
        &self,
        params: &OrderBookParams,
    ) -> Result<ApiResponse<Vec<OrderBookResponse>>> {
        let order_book: ApiResponse<Vec<OrderBookResponse>> = self
            .client
            .get(API::MarketData(GetOrderBookLite), Some(params.to_query()))?;

        Ok(order_book)
    }

    /// Retrieve the candlestick charts. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.
    pub fn get_candles(
        &self,
        params: &CandleSticksParams,
    ) -> Result<ApiResponse<Vec<CandleSticksResponse>>> {
        let candles: ApiResponse<Vec<CandleSticksResponse>> = self
            .client
            .get(API::MarketData(GetCandles), Some(params.to_query()))?;

        Ok(candles)
    }

    /// Retrieve history candlestick charts from recent years.
    pub fn get_candles_hist(
        &self,
        params: &CandleSticksParams,
    ) -> Result<ApiResponse<Vec<CandleSticksResponse>>> {
        let candles: ApiResponse<Vec<CandleSticksResponse>> = self
            .client
            .get(API::MarketData(GetCandlesHist), Some(params.to_query()))?;

        Ok(candles)
    }

    /// Retrieve the candlestick charts of the index. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.
    pub fn get_index_candles(
        &self,
        params: &CandleSticksParams,
    ) -> Result<ApiResponse<Vec<CandleStickNoVolResponse>>> {
        let candles: ApiResponse<Vec<CandleStickNoVolResponse>> = self
            .client
            .get(API::MarketData(GetIndexCandles), Some(params.to_query()))?;

        Ok(candles)
    }

    /// Retrieve the candlestick charts of the index from recent years.
    pub fn get_index_candles_hist(
        &self,
        params: &CandleSticksParams,
    ) -> Result<ApiResponse<Vec<CandleStickNoVolResponse>>> {
        let candles: ApiResponse<Vec<CandleStickNoVolResponse>> = self.client.get(
            API::MarketData(GetIndexCandlesHist),
            Some(params.to_query()),
        )?;

        Ok(candles)
    }

    /// Retrieve the candlestick charts of mark price. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.
    pub fn get_mark_price_candles(
        &self,
        params: &CandleSticksParams,
    ) -> Result<ApiResponse<Vec<CandleStickNoVolResponse>>> {
        let candles: ApiResponse<Vec<CandleStickNoVolResponse>> = self.client.get(
            API::MarketData(GetMarkPriceCandles),
            Some(params.to_query()),
        )?;

        Ok(candles)
    }

    /// Retrieve the candlestick charts of mark price from recent years.
    pub fn get_mark_price_candles_hist(
        &self,
        params: &CandleSticksParams,
    ) -> Result<ApiResponse<Vec<CandleStickNoVolResponse>>> {
        let candles: ApiResponse<Vec<CandleStickNoVolResponse>> = self.client.get(
            API::MarketData(GetMarkPriceCandles),
            Some(params.to_query()),
        )?;

        Ok(candles)
    }

    /// Retrieve the recent transactions of an instrument.
    pub fn get_trades(&self, params: &TradesParams) -> Result<ApiResponse<Vec<TradesResponse>>> {
        let trades: ApiResponse<Vec<TradesResponse>> = self
            .client
            .get(API::MarketData(GetTrades), Some(params.to_query()))?;

        Ok(trades)
    }

    /// Retrieve the recent transactions of an instrument from the last 3 months with pagination.
    pub fn get_trades_hist(
        &self,
        params: &TradesHistParams,
    ) -> Result<ApiResponse<Vec<TradesResponse>>> {
        let trades: ApiResponse<Vec<TradesResponse>> = self
            .client
            .get(API::MarketData(GetTradesHist), Some(params.to_query()))?;

        Ok(trades)
    }
}
