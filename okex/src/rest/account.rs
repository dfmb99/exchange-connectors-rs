use serde_json::to_string;
use crate::commons::errors::*;
use crate::commons::client::Client;
use crate::commons::utils::{ExecType, InstType, PosSide, TradeMode};
use crate::rest::api::{API, ApiResponse};
use crate::rest::api::Account::{
    GetAccountPositionRisk, GetBalance, GetBillsDetails, GetFeeRates, GetLeverage, GetPosition,
    SetLeverage, SetPositionMode,
};
use crate::serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Account {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Default)]
pub struct BalanceParams {
    /// Single currency or multiple currencies (no more than 20) separated with comma, e.g. BTC or BTC,ETH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

impl BalanceParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}",
            "ccy",
            self.ccy.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct BalanceResponse {
    #[serde(rename = "adjEq")]
    pub adj_eq: String,
    pub details: Vec<BalanceDetails>,
    pub imr: String,
    #[serde(rename = "isoEq")]
    pub iso_eq: String,
    #[serde(rename = "mgnRatio")]
    pub mgn_ratio: String,
    pub mmr: String,
    #[serde(rename = "notionalUsd")]
    pub notional_usd: String,
    #[serde(rename = "ordFroz")]
    pub ord_froz: String,
    #[serde(rename = "totalEq")]
    pub total_eq: String,
    #[serde(rename = "uTime")]
    pub u_time: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct BalanceDetails {
    #[serde(rename = "availBal")]
    pub avail_bal: String,
    #[serde(rename = "availEq")]
    pub avail_eq: String,
    #[serde(rename = "cashBal")]
    pub cash_bal: String,
    pub ccy: String,
    #[serde(rename = "crossLiab")]
    pub cross_liab: String,
    #[serde(rename = "disEq")]
    pub dis_eq: String,
    pub eq: String,
    #[serde(rename = "eqUsd")]
    pub eq_usd: String,
    #[serde(rename = "frozenBal")]
    pub frozen_bal: String,
    pub interest: String,
    #[serde(rename = "isoEq")]
    pub iso_eq: String,
    #[serde(rename = "isoLiab")]
    pub iso_liab: String,
    #[serde(rename = "isoUpl")]
    pub iso_upl: String,
    pub liab: String,
    #[serde(rename = "maxLoan")]
    pub max_loan: String,
    #[serde(rename = "mgnRatio")]
    pub mgn_ratio: String,
    #[serde(rename = "notionalLever")]
    pub notional_lever: String,
    #[serde(rename = "ordFrozen")]
    pub ord_frozen: String,
    pub twap: String,
    #[serde(rename = "uTime")]
    pub u_time: String,
    pub upl: String,
    #[serde(rename = "uplLiab")]
    pub upl_liab: String,
    #[serde(rename = "stgyEq")]
    pub stgy_eq: String,
    #[serde(rename = "spotInUseAmt")]
    pub spot_in_use_amt: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PositionsParams {
    /// Instrument type SPOT, SWAP, FUTURES, OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instType")]
    pub inst_type: Option<String>,
    /// Instrument ID, e.g. BTC-USD-190927-5000-C. Single instrument ID or multiple instrument IDs (no more than 10) separated with comma
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// Instrument family. Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "posId")]
    pub pos_id: Option<String>,
}

impl PositionsParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}",
            "inst_type",
            self.inst_type.to_owned().unwrap_or_else(|| "".into()),
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
            "posId",
            self.pos_id.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseOrderAlgo {
    #[serde(rename = "algoId")]
    pub algo_id: String,
    #[serde(rename = "slTriggerPx")]
    pub sl_trigger_px: String,
    #[serde(rename = "slTriggerPxType")]
    pub sl_trigger_px_type: String,
    #[serde(rename = "tpTriggerPx")]
    pub tp_trigger_px: String,
    #[serde(rename = "tpTriggerPxType")]
    pub tp_trigger_px_type: String,
    #[serde(rename = "closeFraction")]
    pub close_fraction: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionResponse {
    pub adl: String,
    #[serde(rename = "availPos")]
    pub avail_pos: String,
    #[serde(rename = "avgPx")]
    pub avg_px: String,
    #[serde(rename = "cTime")]
    pub c_time: String,
    pub ccy: String,
    #[serde(rename = "deltaBS")]
    pub delta_bs: String,
    #[serde(rename = "deltaPA")]
    pub delta_pa: String,
    #[serde(rename = "gammaBS")]
    pub gamma_bs: String,
    #[serde(rename = "gammaPA")]
    pub gamma_pa: String,
    pub imr: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: String,
    pub interest: String,
    #[serde(rename = "usdPx")]
    pub usd_px: String,
    pub last: String,
    pub lever: String,
    pub liab: String,
    #[serde(rename = "liabCcy")]
    pub liab_ccy: String,
    #[serde(rename = "liqPx")]
    pub liq_px: String,
    #[serde(rename = "markPx")]
    pub mark_px: String,
    pub margin: String,
    #[serde(rename = "mgnMode")]
    pub mgn_mode: TradeMode,
    #[serde(rename = "mgnRatio")]
    pub mgn_ratio: String,
    pub mmr: String,
    #[serde(rename = "notionalUsd")]
    pub notional_usd: String,
    #[serde(rename = "optVal")]
    pub opt_val: String,
    #[serde(rename = "pTime")]
    pub p_time: String,
    pub pos: String,
    #[serde(rename = "baseBorrowed")]
    pub base_borrowed: String,
    #[serde(rename = "baseInterest")]
    pub base_interest: String,
    #[serde(rename = "quoteBorrowed")]
    pub quote_borrowed: String,
    #[serde(rename = "quoteInterest")]
    pub quote_interest: String,
    #[serde(rename = "posCcy")]
    pub pos_ccy: String,
    #[serde(rename = "posId")]
    pub pos_id: String,
    #[serde(rename = "posSide")]
    pub pos_side: String,
    #[serde(rename = "spotInUseAmt")]
    pub spot_in_use_amt: String,
    #[serde(rename = "spotInUseCcy")]
    pub spot_in_use_ccy: String,
    #[serde(rename = "bizRefId")]
    pub biz_ref_id: String,
    #[serde(rename = "bizRefType")]
    pub biz_ref_type: String,
    #[serde(rename = "thetaBS")]
    pub theta_bs: String,
    #[serde(rename = "thetaPA")]
    pub theta_pa: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    #[serde(rename = "uTime")]
    pub u_time: String,
    pub upl: String,
    #[serde(rename = "uplRatio")]
    pub upl_ratio: String,
    #[serde(rename = "vegaBS")]
    pub vega_bs: String,
    #[serde(rename = "vegaPA")]
    pub vega_pa: String,
    #[serde(rename = "closeOrderAlgo")]
    pub close_order_algo: Vec<CloseOrderAlgo>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AccountPositionRiskParams {
    /// Instrument type: MARGIN, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: Option<String>,
}

impl AccountPositionRiskParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}",
            "instType",
            self.inst_type.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PosData {
    #[serde(rename = "baseBal")]
    pub base_bal: String,
    pub ccy: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "mgnMode")]
    pub mgn_mode: TradeMode,
    #[serde(rename = "notionalCcy")]
    pub notional_ccy: String,
    #[serde(rename = "notionalUsd")]
    pub notional_usd: String,
    pub pos: String,
    #[serde(rename = "posCcy")]
    pub pos_ccy: String,
    #[serde(rename = "posId")]
    pub pos_id: String,
    #[serde(rename = "posSide")]
    pub pos_side: String,
    #[serde(rename = "quoteBal")]
    pub quote_bal: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceData {
    pub ccy: String,
    #[serde(rename = "disEq")]
    pub dis_eq: String,
    pub eq: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountPositionRiskResponse {
    #[serde(rename = "adjEq")]
    pub adj_eq: String,
    #[serde(rename = "balData")]
    pub bal_data: Vec<BalanceData>,
    #[serde(rename = "posData")]
    pub pos_data: Vec<PosData>,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PositionModeParams {
    /// Position mode
    /// long_short_mode: long/short, only applicable to FUTURES/SWAP
    /// net_mode: net
    #[serde(rename = "posMode")]
    pub pos_mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionModeResponse {
    #[serde(rename = "posMode")]
    pub pos_mode: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SetLeverageParams {
    /// Instrument ID
    /// Either instId or ccy is required; if both are passed, instId will be used by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// Currency used for margin
    /// Only applicable to cross MARGIN of Multi-currency margin
    /// Required when setting the leverage of automatic borrowing coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Leverage
    pub lever: String,
    /// Margin mode: isolated, cross
    /// Only can be cross if ccy is passed.
    #[serde(rename = "mgnMode")]
    pub mgn_mode: TradeMode,
    /// Position side: long, short
    /// Only required when margin mode is isolated in long/short mode for FUTURES/SWAP.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "posSide")]
    pub pos_side: Option<PosSide>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetLeverageParams {
    /// Instrument ID
    /// Single instrument ID or multiple instrument IDs (no more than 20) separated with comma
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Margin mode: isolated, cross
    #[serde(rename = "mgnMode")]
    pub mgn_mode: TradeMode,
}

impl GetLeverageParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}",
            "instId", self.inst_id, "mgnMode", self.mgn_mode,
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeverageResponse {
    pub lever: String,
    #[serde(rename = "mgnMode")]
    pub mgn_mode: TradeMode,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "posSide")]
    pub pos_side: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct BillsDetailsParams {
    /// Instrument type SPOT, MARGIN SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: Option<String>,
    /// Bill currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Margin mode: isolated, cross
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mgnMode")]
    pub mgn_mode: Option<String>,
    /// Contract type: linear, inverse
    /// Only applicable to FUTURES/SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ctType")]
    pub ct_type: Option<String>,
    /// Bill type
    /// 1: Transfer 2: Trade 3: Delivery 4: Auto token conversion 5: Liquidation 6: Margin transfer 7: Interest deduction 8: Funding fee 9: ADL 10: Clawback 11: System token conversion 12: Strategy transfer 13: ddh 14: Block trade 15: Quick Margin 18: Profit sharing 22: Repay
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    /// Bill subtype
    /// 1: Buy 2: Sell 3: Open long 4: Open short 5: Close long 6: Close short 9: Interest deduction for Market loans 11: Transfer in 12: Transfer out 14: Interest deduction for VIP loans 160: Manual margin increase 161: Manual margin decrease 162: Auto margin increase 114: Auto buy 115: Auto sell 118: System token conversion transfer in 119: System token conversion transfer out 100: Partial liquidation close long 101: Partial liquidation close short 102: Partial liquidation buy 103: Partial liquidation sell 104: Liquidation long 105: Liquidation short 106: Liquidation buy 107: Liquidation sell 110: Liquidation transfer in 111: Liquidation transfer out 125: ADL close long 126: ADL close short 127: ADL buy 128: ADL sell 131: ddh buy 132: ddh sell 170: Exercised 171: Counterparty exercised 172: Expired OTM 112: Delivery long 113: Delivery short 117: Delivery/Exercise clawback 173: Funding fee expense 174: Funding fee income 200:System transfer in 201: Manually transfer in 202: System transfer out 203: Manually transfer out 204: block trade buy 205: block trade sell 206: block trade open long 207: block trade open short 208: block trade close open 209: block trade close short 210: Manual Borrowing 211: Manual Repayment 212: Auto borrow 213: Auto repay 16: Repay forcibly 17: Repay interest by borrowing forcibly 224: repayment transfer in 225: repayment transfer out 250: Profit sharing expenses; 251: Profit sharing refund; 252: Profit sharing income;
    #[serde(rename = "subType")]
    pub sub_type: Option<String>,
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

impl BillsDetailsParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type.to_owned().unwrap_or_else(|| "".into()),
            "ccy",
            self.ccy.to_owned().unwrap_or_else(|| "".into()),
            "mgnMode",
            self.mgn_mode.to_owned().unwrap_or_else(|| "".into()),
            "ctType",
            self.ct_type.to_owned().unwrap_or_else(|| "".into()),
            "type",
            self.bill_type.to_owned().unwrap_or_else(|| "".into()),
            "subType",
            self.sub_type.to_owned().unwrap_or_else(|| "".into()),
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
pub struct BillDetailsResponse {
    pub bal: String,
    #[serde(rename = "balChg")]
    pub bal_chg: String,
    #[serde(rename = "billId")]
    pub bill_id: String,
    pub ccy: String,
    #[serde(rename = "execType")]
    pub exec_type: ExecType,
    pub fee: String,
    pub from: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "mgnMode")]
    pub mgn_mode: TradeMode,
    pub notes: String,
    #[serde(rename = "ordId")]
    pub ord_id: String,
    pub pnl: String,
    #[serde(rename = "posBal")]
    pub pos_bal: String,
    #[serde(rename = "posBalChg")]
    pub pos_bal_chg: String,
    #[serde(rename = "subType")]
    pub sub_type: String,
    pub sz: String,
    pub to: String,
    pub ts: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FeeRatesParams {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID, e.g. BTC-USDT
    /// Applicable to SPOT/MARGIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// Margin mode: isolated, cross
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family, e.g. BTC-USD
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
}

impl FeeRatesParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeRatesResponse {
    pub category: String,
    pub delivery: String,
    pub exercise: String,
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    pub level: String,
    pub maker: String,
    #[serde(rename = "makerU")]
    pub maker_u: String,
    #[serde(rename = "makerUSDC")]
    pub maker_usdc: String,
    pub taker: String,
    #[serde(rename = "takerU")]
    pub taker_u: String,
    #[serde(rename = "takerUSDC")]
    pub taker_usdc: String,
    pub ts: String,
}

impl Account {
    /// Retrieve a list of assets (with non-zero balance), remaining balance, and available amount in the trading account.
    pub fn get_balance(&self, params: &BalanceParams) -> Result<ApiResponse<Vec<BalanceResponse>>> {
        let balance: ApiResponse<Vec<BalanceResponse>> = self
            .client
            .get_signed(API::Account(GetBalance), Some(params.to_query()))?;

        Ok(balance)
    }

    /// Retrieve information on your positions. When the account is in net mode, net positions will be displayed, and when the account is in long/short mode, long or short positions will be displayed. Return in reverse chronological order using ctime.
    pub fn get_positions(
        &self, params: &PositionsParams,
    ) -> Result<ApiResponse<Vec<PositionResponse>>> {
        let positions: ApiResponse<Vec<PositionResponse>> = self
            .client
            .get_signed(API::Account(GetPosition), Some(params.to_query()))?;

        Ok(positions)
    }

    /// Get account and position risk
    pub fn get_acc_position_risk(
        &self, params: &AccountPositionRiskParams,
    ) -> Result<ApiResponse<Vec<AccountPositionRiskResponse>>> {
        let acc_position_risk: ApiResponse<Vec<AccountPositionRiskResponse>> =
            self.client.get_signed(
                API::Account(GetAccountPositionRisk),
                Some(params.to_query()),
            )?;

        Ok(acc_position_risk)
    }

    /// Single-currency mode and Multi-currency mode: FUTURES and SWAP support both long/short mode and net mode. In net mode, users can only have positions in one direction; In long/short mode, users can hold positions in long and short directions.
    /// Portfolio margin mode: FUTURES and SWAP only support net mode
    pub fn set_position_mode(
        &self, params: &PositionModeParams,
    ) -> Result<ApiResponse<Vec<PositionModeResponse>>> {
        let payload: String = to_string(params)?;
        let position_mode: ApiResponse<Vec<PositionModeResponse>> = self
            .client
            .post_signed(API::Account(SetPositionMode), payload)?;

        Ok(position_mode)
    }

    /// There are 9 different scenarios for leverage setting:
    ///
    /// 1. Set leverage for MARGIN instruments under isolated-margin trade mode at pairs level.
    /// 2. Set leverage for MARGIN instruments under cross-margin trade mode and Single-currency margin account mode at pairs level.
    /// 3. Set leverage for MARGIN instruments under cross-margin trade mode and Multi-currency margin at currency level.
    /// 4. Set leverage for FUTURES instruments under cross-margin trade mode at underlying level.
    /// 5. Set leverage for FUTURES instruments under isolated-margin trade mode and buy/sell position mode at contract level.
    /// 6. Set leverage for FUTURES instruments under isolated-margin trade mode and long/short position mode at contract and position side level.
    /// 7. Set leverage for SWAP instruments under cross-margin trade at contract level.
    /// 8. Set leverage for SWAP instruments under isolated-margin trade mode and buy/sell position mode at contract level.
    /// 9. Set leverage for SWAP instruments under isolated-margin trade mode and long/short position mode at contract and position side level.
    ///
    ///
    /// Note that the request parameter posSide is only required when margin mode is isolated in long/short position mode for FUTURES/SWAP instruments (see scenario 6 and 9 above).
    /// Please refer to the request examples on the right side for each case.
    pub fn set_leverage(
        &self, params: &SetLeverageParams,
    ) -> Result<ApiResponse<Vec<LeverageResponse>>> {
        let payload: String = to_string(params)?;
        let leverage: ApiResponse<Vec<LeverageResponse>> = self
            .client
            .post_signed(API::Account(SetLeverage), payload)?;

        Ok(leverage)
    }

    /// Get leverage
    pub fn get_leverage(
        &self, params: &GetLeverageParams,
    ) -> Result<ApiResponse<Vec<LeverageResponse>>> {
        let leverage: ApiResponse<Vec<LeverageResponse>> = self
            .client
            .get_signed(API::Account(GetLeverage), Some(params.to_query()))?;

        Ok(leverage)
    }

    /// Retrieve the bills of the account. The bill refers to all transaction records that result in changing the balance of an account. Pagination is supported, and the response is sorted with the most recent first. This endpoint can retrieve data from the last 7 days.
    pub fn get_bills_details(
        &self, params: &BillsDetailsParams,
    ) -> Result<ApiResponse<Vec<BillDetailsResponse>>> {
        let bills: ApiResponse<Vec<BillDetailsResponse>> = self
            .client
            .get_signed(API::Account(GetBillsDetails), Some(params.to_query()))?;

        Ok(bills)
    }

    /// Get fee rates
    pub fn get_fee_rates(
        &self, params: &FeeRatesParams,
    ) -> Result<ApiResponse<Vec<FeeRatesResponse>>> {
        let trade_fees: ApiResponse<Vec<FeeRatesResponse>> = self
            .client
            .get_signed(API::Account(GetFeeRates), Some(params.to_query()))?;

        Ok(trade_fees)
    }
}
