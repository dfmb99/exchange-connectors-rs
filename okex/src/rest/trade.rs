use crate::commons::client::Client;
use crate::commons::errors::*;
use crate::commons::utils::{
    ExecType, InstType, OrdCategory, OrdState, OrdType, PosSide, QuickMgnType, Side, TradeMode,
    TriggerPriceType,
};
use crate::rest::api::Trade::{
    AmendMultipleOrders, AmendOrder, CancelMultipleOrders, CancelOrder, ClosePositions, GetFills,
    GetFillsHist, GetOrderDetails, GetOrderHist, GetOrderList, PlaceMultipleOrders, PlaceOrder,
};
use crate::rest::api::{ApiResponse, API};
use crate::serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Clone)]
pub struct Trade {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PlaceOrderParams {
    /// Instrument ID, e.g. BTC-USD-190927-5000-C
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Trade mode
    /// Margin mode: cross, isolated
    /// Non-Margin mode: cash
    #[serde(rename = "tdMode")]
    pub td_mode: TradeMode,
    /// Margin currency
    /// Only applicable to cross MARGIN orders in Single-currency margin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Client Order ID as assigned by the client
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
    /// Order tag
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Order side: buy, sell
    pub side: Side,
    /// Position side
    /// The default is net in the net mode
    /// It is required in the long/short mode, and can only be long or short.
    /// Only applicable to FUTURES/SWAP.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "posSide")]
    pub pos_side: Option<PosSide>,
    /// Order type
    /// market: Market order
    /// limit: Limit order
    /// post_only: Post-only order
    /// fok: Fill-or-kill order
    /// ioc: Immediate-or-cancel order
    /// optimal_limit_ioc: Market order with immediate-or-cancel order (applicable only to Futures and Perpetual swap).
    #[serde(rename = "ordType")]
    pub ord_type: OrdType,
    /// Quantity to buy or sell
    pub sz: String,
    /// Order price. Only applicable to limit, post_only, fok, ioc order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// Whether orders can only reduce in position size.
    /// Valid options: true or false. The default value is false.
    /// Only applicable to MARGIN orders, and FUTURES/SWAP orders in net mode
    /// Only applicable to Single-currency margin and Multi-currency margin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reduceOnly")]
    pub reduce_only: Option<bool>,
    /// Whether the target currency uses the quote or base currency.
    /// base_ccy: Base currency ,quote_ccy: Quote currency
    /// Only applicable to SPOT Market Orders
    /// Default is quote_ccy for buy, base_ccy for sell
    #[serde(rename = "tgtCcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,
    /// Whether to disallow the system from amending the size of the SPOT Market Order.
    /// Valid options: true or false. The default value is false.
    /// If true, system will not amend and reject the market order if user does not have sufficient funds.
    /// Only applicable to SPOT Market Orders
    #[serde(rename = "banAmend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_amend: Option<bool>,
    /// Take-profit trigger price
    /// If you fill in this parameter, you should fill in the take-profit order price as well.
    #[serde(rename = "tpTriggerPx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Take-profit order price
    /// If you fill in this parameter, you should fill in the take-profit trigger price as well.
    /// If the price is -1, take-profit will be executed at the market price.
    #[serde(rename = "tpOrdPx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    /// Stop-loss trigger price
    /// If you fill in this parameter, you should fill in the stop-loss order price.
    #[serde(rename = "slTriggerPx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Stop-loss order price
    /// If you fill in this parameter, you should fill in the stop-loss trigger price.
    /// If the price is -1, stop-loss will be executed at the market price.
    #[serde(rename = "slOrdPx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
    /// Take-profit trigger price type
    /// last: last price
    /// index: index price
    /// mark: mark price
    /// The Default is last
    #[serde(rename = "tpTriggerPxType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<TriggerPriceType>,
    /// Take-profit trigger price type
    /// last: last price
    /// index: index price
    /// mark: mark price
    /// The Default is last
    #[serde(rename = "slTriggerPxType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<TriggerPriceType>,
    /// Quick Margin type. Only applicable to Quick Margin Mode of isolated margin
    /// manual, auto_borrow, auto_repay
    /// The default value is manual
    #[serde(rename = "quickMgnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_mgn_type: Option<QuickMgnType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceOrderResponse {
    #[serde(rename = "ordId")]
    pub ord_id: String,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,
    pub tag: String,
    #[serde(rename = "sCode")]
    pub s_code: String,
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CancelOrderParams {
    /// Instrument ID, e.g. BTC-USD-190927
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Order ID
    /// Either ordId or clOrdId is required. If both are passed, ordId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ordId")]
    pub ord_id: Option<String>,
    /// Client Order ID as assigned by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrderResponse {
    #[serde(rename = "ordId")]
    pub ord_id: String,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,
    #[serde(rename = "sCode")]
    pub s_code: String,
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AmendOrderParams {
    /// Instrument ID, e.g. BTC-USD-190927
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Whether the order needs to be automatically canceled when the order amendment fails
    /// Valid options: false or true, the default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cxlOnFail")]
    pub cxl_on_fail: Option<bool>,
    /// Order ID
    /// Either ordId or clOrdId is required. If both are passed, ordId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ordId")]
    pub ord_id: Option<String>,
    /// Client Order ID as assigned by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
    /// Client Request ID as assigned by the client for order amendment
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    /// The response will include the corresponding reqId to help you identify the request if you provide it in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reqId")]
    pub req_id: Option<String>,
    /// New quantity after amendment. Either newSz or newPx is required. When amending a partially-filled order, the newSz should include the amount that has been filled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "newSz")]
    pub new_sz: Option<String>,
    /// New price after amendment. Either newSz or newPx is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "newPx")]
    pub new_px: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmendOrderResponse {
    #[serde(rename = "ordId")]
    pub ord_id: String,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,
    #[serde(rename = "reqId")]
    pub req_id: String,
    #[serde(rename = "sCode")]
    pub s_code: String,
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ClosePositionParams {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Position side
    /// This parameter can be omitted in net mode, and the default value is net. You can only fill with net.
    /// This parameter must be filled in under the long/short mode. Fill in long for close-long and short for close-short.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "posSide")]
    pub pos_side: Option<PosSide>,
    /// Margin mode
    /// cross isolated
    #[serde(rename = "mgnMode")]
    pub mgn_mode: TradeMode,
    /// Margin currency, required in the case of closing cross MARGIN position for Single-currency margin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Whether any pending orders for closing out needs to be automatically canceled when close position via a market order.
    /// false or true, the default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "autoCxl")]
    pub auto_cxl: Option<bool>,
    /// Client-supplied ID
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<bool>,
    /// Order tag
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClosePositionResponse {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "posSide")]
    pub pos_side: PosSide,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrderDetailsParams {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Order ID
    /// Either ordId or clOrdId is required, if both are passed, ordId will be used
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ordId")]
    pub ord_id: Option<String>,
    /// Client Order ID as assigned by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
}

impl OrderDetailsParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}",
            "instId",
            self.inst_id,
            "ordId",
            self.ord_id.to_owned().unwrap_or_else(|| "".into()),
            "clOrdId",
            self.cl_ord_id.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct OrderListParams {
    /// Instrument type SPOT, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: Option<String>,
    /// Underlying, e.g. BTC-USD Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family. Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Instrument ID, e.g. BTC-USD-200927
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// Order type
    /// market: Market order
    /// limit: Limit order
    /// post_only: Post-only order
    /// fok: Fill-or-kill order
    /// ioc: Immediate-or-cancel order
    /// optimal_limit_ioc: Market order with immediate-or-cancel order
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ordType")]
    pub ord_type: Option<String>,
    /// State: live, partially_filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Pagination of data to return records earlier than the requested ordId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ordId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl OrderListParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type.to_owned().unwrap_or_else(|| "".into()),
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
            "ordType",
            self.ord_type.to_owned().unwrap_or_else(|| "".into()),
            "state",
            self.state.to_owned().unwrap_or_else(|| "".into()),
            "after",
            self.after.to_owned().unwrap_or_else(|| "".into()),
            "before",
            self.before.to_owned().unwrap_or_else(|| "".into()),
            "limit",
            self.limit.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderDetailsResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "tgtCcy")]
    pub tgt_ccy: String,
    pub ccy: String,
    #[serde(rename = "ordId")]
    pub ord_id: String,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,
    pub tag: String,
    pub px: String,
    pub sz: String,
    pub pnl: String,
    #[serde(rename = "ordType")]
    pub ord_type: OrdType,
    pub side: Side,
    #[serde(rename = "posSide")]
    pub pos_side: PosSide,
    #[serde(rename = "tdMode")]
    pub td_mode: TradeMode,
    #[serde(rename = "accFillSz")]
    pub acc_fill_sz: String,
    #[serde(rename = "fillPx")]
    pub fill_px: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    #[serde(rename = "fillSz")]
    pub fill_sz: String,
    #[serde(rename = "fillTime")]
    pub fill_time: String,
    #[serde(rename = "avgPx")]
    pub avg_px: String,
    pub state: OrdState,
    pub lever: String,
    #[serde(rename = "tpTriggerPx")]
    pub tp_trigger_px: String,
    #[serde(rename = "tpTriggerPxType")]
    pub tp_trigger_px_type: TriggerPriceType,
    #[serde(rename = "slOrdPx")]
    pub sl_ord_px: String,
    #[serde(rename = "feeCcy")]
    pub fee_ccy: String,
    pub fee: String,
    #[serde(rename = "rebateCcy")]
    pub rebate_ccy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub rebate: String,
    pub category: OrdCategory,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cancelSource")]
    pub cancel_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cancelSourceReason")]
    pub cancel_source_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quickMgnType")]
    pub quick_mgn_type: Option<QuickMgnType>,
    #[serde(rename = "uTime")]
    pub u_time: String,
    #[serde(rename = "cTime")]
    pub c_time: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FillsParams {
    /// Instrument type SPOT, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: Option<String>,
    /// Underlying, e.g. BTC-USD Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family. Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Instrument ID, e.g. BTC-USD-200927
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ordId")]
    pub ord_id: Option<String>,
    /// Pagination of data to return records earlier than the requested billId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested billId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Filter with a begin timestamp. Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// Filter with an end timestamp. Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl FillsParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type.to_owned().unwrap_or_else(|| "".into()),
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
            "ordType",
            self.ord_id.to_owned().unwrap_or_else(|| "".into()),
            "after",
            self.after.to_owned().unwrap_or_else(|| "".into()),
            "before",
            self.before.to_owned().unwrap_or_else(|| "".into()),
            "begin",
            self.begin.to_owned().unwrap_or_else(|| "".into()),
            "end",
            self.end.to_owned().unwrap_or_else(|| "".into()),
            "limit",
            self.limit.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FillsResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    #[serde(rename = "ordId")]
    pub ord_id: String,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,
    #[serde(rename = "billId")]
    pub bill_id: String,
    pub tag: String,
    #[serde(rename = "fillPx")]
    pub fill_px: String,
    #[serde(rename = "fillSz")]
    pub fill_sz: String,
    pub side: Side,
    #[serde(rename = "posSide")]
    pub pos_side: PosSide,
    #[serde(rename = "execType")]
    pub exec_type: ExecType,
    #[serde(rename = "feeCcy")]
    pub fee_ccy: String,
    pub fee: String,
    pub ts: String,
}

impl Trade {
    /// You can place an order only if you have sufficient funds.
    /// For leading contracts, this endpoint supports placement, but can't close positions.
    pub fn place_order(
        &self,
        params: &PlaceOrderParams,
    ) -> Result<ApiResponse<Vec<PlaceOrderResponse>>> {
        let payload: String = to_string(params)?;
        let order: ApiResponse<Vec<PlaceOrderResponse>> =
            self.client.post_signed(API::Trade(PlaceOrder), payload)?;

        Ok(order)
    }

    /// Place orders in batches. Maximum 20 orders can be placed per request. Request parameters should be passed in the form of an array.
    /// For leading contracts, this endpoint supports placement, but can't close positions.
    pub fn place_multiple_orders(
        &self,
        params: &Vec<PlaceOrderParams>,
    ) -> Result<ApiResponse<Vec<PlaceOrderResponse>>> {
        let payload: String = to_string(params)?;
        let orders: ApiResponse<Vec<PlaceOrderResponse>> = self
            .client
            .post_signed(API::Trade(PlaceMultipleOrders), payload)?;

        Ok(orders)
    }

    /// Cancel an incomplete order.
    /// Cancel order returns with sCode equal to 0. It is not strictly considered that the order has been canceled. It only means that your cancelation request has been accepted by the system server. The result of the cancelation is subject to the state pushed by the order channel or the get order state.
    pub fn cancel_order(
        &self,
        params: &CancelOrderParams,
    ) -> Result<ApiResponse<Vec<CancelOrderResponse>>> {
        let payload: String = to_string(params)?;
        let order: ApiResponse<Vec<CancelOrderResponse>> =
            self.client.post_signed(API::Trade(CancelOrder), payload)?;

        Ok(order)
    }

    /// Cancel an incomplete order.
    /// Cancel order returns with sCode equal to 0. It is not strictly considered that the order has been canceled. It only means that your cancelation request has been accepted by the system server. The result of the cancelation is subject to the state pushed by the order channel or the get order state.
    pub fn cancel_multiple_orders(
        &self,
        params: &Vec<CancelOrderParams>,
    ) -> Result<ApiResponse<Vec<CancelOrderResponse>>> {
        let payload: String = to_string(params)?;
        let orders: ApiResponse<Vec<CancelOrderResponse>> = self
            .client
            .post_signed(API::Trade(CancelMultipleOrders), payload)?;

        Ok(orders)
    }

    /// Amend an incomplete order.
    pub fn amend_order(
        &self,
        params: &AmendOrderParams,
    ) -> Result<ApiResponse<Vec<AmendOrderResponse>>> {
        let payload: String = to_string(params)?;
        let order: ApiResponse<Vec<AmendOrderResponse>> =
            self.client.post_signed(API::Trade(AmendOrder), payload)?;

        Ok(order)
    }

    /// Amend incomplete orders in batches. Maximum 20 orders can be amended per request. Request parameters should be passed in the form of an array.
    pub fn amend_multiple_order(
        &self,
        params: &Vec<AmendOrderParams>,
    ) -> Result<ApiResponse<Vec<AmendOrderResponse>>> {
        let payload: String = to_string(params)?;
        let orders: ApiResponse<Vec<AmendOrderResponse>> = self
            .client
            .post_signed(API::Trade(AmendMultipleOrders), payload)?;

        Ok(orders)
    }

    /// Amend incomplete orders in batches. Maximum 20 orders can be amended per request. Request parameters should be passed in the form of an array.
    /// if there are any pending orders for closing out and the orders do not need to be automatically canceled, it will return an error code and message to prompt users to cancel pending orders before closing the positions.
    pub fn close_position(
        &self,
        params: &ClosePositionParams,
    ) -> Result<ApiResponse<Vec<ClosePositionResponse>>> {
        let payload: String = to_string(params)?;
        let order: ApiResponse<Vec<ClosePositionResponse>> = self
            .client
            .post_signed(API::Trade(ClosePositions), payload)?;

        Ok(order)
    }

    /// Retrieve order details.
    pub fn get_order_details(
        &self,
        params: &OrderDetailsParams,
    ) -> Result<ApiResponse<Vec<OrderDetailsResponse>>> {
        let order: ApiResponse<Vec<OrderDetailsResponse>> = self
            .client
            .get_signed(API::Trade(GetOrderDetails), Some(params.to_query()))?;

        Ok(order)
    }

    /// Retrieve all incomplete orders under the current account.
    pub fn get_order_list(
        &self,
        params: &OrderListParams,
    ) -> Result<ApiResponse<Vec<OrderDetailsResponse>>> {
        let order: ApiResponse<Vec<OrderDetailsResponse>> = self
            .client
            .get_signed(API::Trade(GetOrderList), Some(params.to_query()))?;

        Ok(order)
    }

    /// Retrieve the completed order data of the last 3 months.
    pub fn get_order_hist(
        &self,
        params: &OrderListParams,
    ) -> Result<ApiResponse<Vec<OrderDetailsResponse>>> {
        let order: ApiResponse<Vec<OrderDetailsResponse>> = self
            .client
            .get_signed(API::Trade(GetOrderHist), Some(params.to_query()))?;

        Ok(order)
    }

    /// Retrieve recently-filled transaction details in the last 3 day.
    pub fn get_fills(&self, params: &FillsParams) -> Result<ApiResponse<Vec<FillsResponse>>> {
        let fills: ApiResponse<Vec<FillsResponse>> = self
            .client
            .get_signed(API::Trade(GetFills), Some(params.to_query()))?;

        Ok(fills)
    }

    /// Retrieve recently-filled transaction details in the last 3 months.
    pub fn get_fills_hist(&self, params: &FillsParams) -> Result<ApiResponse<Vec<FillsResponse>>> {
        let fills: ApiResponse<Vec<FillsResponse>> = self
            .client
            .get_signed(API::Trade(GetFillsHist), Some(params.to_query()))?;

        Ok(fills)
    }
}
