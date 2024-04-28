use crate::commons::client::Client;
use crate::commons::errors::*;
use crate::commons::utils::{Alias, InstType, OrdState, TradeMode};
use crate::rest::api::PublicData::{
    GetDeliveryHist, GetDiscountRate, GetEstimatedDeliveryPrice, GetFundingRate,
    GetFundingRateHist, GetInstruments, GetInsuranceFund, GetInterestRate, GetLimitPrice,
    GetLiquidationOrders, GetMarkPrice, GetOpenInterest, GetOptionTrades, GetOptionsMarketData,
    GetPositionTiers, GetSystemTime, GetUnderlying, UnitConvert,
};
use crate::rest::api::{ApiResponse, API};
use crate::serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct PublicData {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Default)]
pub struct InstrumentsParams {
    /// Instrument type SPOT, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    /// Underlying
    /// Only applicable to FUTURES/SWAP/OPTION.If instType is OPTION, either uly or instFamily is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    /// Only applicable to FUTURES/SWAP/OPTION. If instType is OPTION, either uly or instFamily is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
}

impl InstrumentsParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentsResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instFamily")]
    pub inst_family: String,
    pub uly: String,
    pub category: String,
    #[serde(rename = "baseCcy")]
    pub base_ccy: String,
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: String,
    #[serde(rename = "settleCcy")]
    pub settle_ccy: String,
    #[serde(rename = "ctVal")]
    pub ct_val: String,
    #[serde(rename = "ctMult")]
    pub ct_mult: String,
    #[serde(rename = "ctValCcy")]
    pub ct_val_ccy: String,
    #[serde(rename = "optType")]
    pub opt_type: String,
    pub stk: String,
    #[serde(rename = "listTime")]
    pub list_time: String,
    #[serde(rename = "expTime")]
    pub exp_time: String,
    pub lever: String,
    #[serde(rename = "tickSz")]
    pub tick_sz: String,
    #[serde(rename = "lotSz")]
    pub lot_sz: String,
    #[serde(rename = "minSz")]
    pub min_sz: String,
    #[serde(rename = "ctType")]
    pub ct_type: String,
    pub alias: String,
    pub state: String,
    #[serde(rename = "maxLmtSz")]
    pub max_lmt_sz: String,
    #[serde(rename = "maxMktSz")]
    pub max_mkt_sz: String,
    #[serde(rename = "maxTwapSz")]
    pub max_twap_sz: String,
    #[serde(rename = "maxIcebergSz")]
    pub max_iceberg_sz: String,
    #[serde(rename = "maxTriggerSz")]
    pub max_trigger_sz: String,
    #[serde(rename = "maxStopSz")]
    pub max_stop_sz: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DeliveryHistParams {
    /// Instrument type FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    /// Underlying, only applicable to FUTURES/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family, only applicable to FUTURES/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl DeliveryHistParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
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
pub struct DeliveryHistData {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    pub px: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliveryHistResponse {
    pub ts: String,
    pub details: Vec<DeliveryHistData>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OpenInterestParams {
    /// Instrument type SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    /// Underlying
    /// Applicable to FUTURES/SWAP/OPTION.
    /// If instType is OPTION, either uly or instFamily is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    /// Applicable to FUTURES/SWAP/OPTION
    /// If instType is OPTION, either uly or instFamily is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Instrument ID, e.g. BTC-USD-180216
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
}

impl OpenInterestParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenInterestResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    pub oi: String,
    #[serde(rename = "oiCcy")]
    pub oi_ccy: String,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FundingRateParams {
    /// Instrument ID, e.g. BTC-USD-SWAP
    /// only applicable to SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,
}

impl FundingRateParams {
    pub fn to_query(&self) -> String {
        format!("{}={}", "instId", self.inst_id,)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingRateResponse {
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "fundingTime")]
    pub funding_time: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "nextFundingRate")]
    pub next_funding_rate: String,
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FundingRateHistParams {
    /// Instrument ID, e.g. BTC-USD-SWAP
    /// only applicable to SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Pagination of data to return records newer than the requested fundingTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records earlier than the requested fundingTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl FundingRateHistParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "instId",
            self.inst_id,
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
pub struct FundingRateHistResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "realizedRate")]
    pub realized_rate: String,
    #[serde(rename = "fundingTime")]
    pub funding_time: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct LimitPriceParams {
    /// Instrument ID, e.g. BTC-USDT-SWAP
    /// only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "instId")]
    pub inst_id: String,
}

impl LimitPriceParams {
    pub fn to_query(&self) -> String {
        format!("{}={}", "instId", self.inst_id,)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitPriceResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "buyLmt")]
    pub buy_lmt: String,
    #[serde(rename = "sellLmt")]
    pub sell_lmt: String,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OptionMarketDataParams {
    /// Underlying, only applicable to OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family, only applicable to OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Contract expiry date, the format is "YYMMDD", e.g. "200527"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expTime")]
    pub exp_time: Option<String>,
}

impl OptionMarketDataParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}",
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "expTime",
            self.exp_time.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionMarketDataResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    pub uly: String,
    pub delta: String,
    pub gamma: String,
    pub theta: String,
    pub vega: String,
    #[serde(rename = "deltaBS")]
    pub delta_bs: String,
    #[serde(rename = "gammaBS")]
    pub gamma_bs: String,
    #[serde(rename = "thetaBS")]
    pub theta_bs: String,
    #[serde(rename = "vegaBS")]
    pub vega_bs: String,
    #[serde(rename = "realVol")]
    pub real_vol: String,
    #[serde(rename = "bidVol")]
    pub bid_vol: String,
    #[serde(rename = "askVol")]
    pub ask_vol: String,
    #[serde(rename = "markVol")]
    pub mark_vol: String,
    pub lever: String,
    pub ts: String,
    #[serde(rename = "fwdPx")]
    pub fwd_px: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EstimatedDeliveryPriceParams {
    /// Instrument ID, e.g. BTC-USD-200214
    /// only applicable to FUTURES/OPTION
    #[serde(rename = "instId")]
    pub inst_id: String,
}

impl EstimatedDeliveryPriceParams {
    pub fn to_query(&self) -> String {
        format!("{}={}", "instId", self.inst_id,)
    }
}

#[derive(Serialize, Deserialize)]
pub struct EstimatedDeliveryPriceResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "settlePx")]
    pub settle_px: String,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DiscountRateParams {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Discount level
    /// 1:level 1
    /// 2:level 2
    /// 3:level 3
    /// 4:level 4
    /// 5:level 5
    #[serde(rename = "discountLv")]
    pub discount_lv: Option<String>,
}

impl DiscountRateParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}",
            "ccy",
            self.ccy.to_owned().unwrap_or_else(|| "".into()),
            "discountLv",
            self.discount_lv.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct DiscountRateData {
    #[serde(rename = "discountRate")]
    pub discount_rate: String,
    #[serde(rename = "maxAmt")]
    pub max_amt: String,
    #[serde(rename = "minAmt")]
    pub min_amt: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscountRateResponse {
    pub amt: String,
    pub ccy: String,
    #[serde(rename = "discountInfo")]
    pub discount_info: Vec<DiscountRateData>,
    #[serde(rename = "discountLv")]
    pub discount_lv: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemTimeResponse {
    pub ts: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct LiquidationOrdersParams {
    /// Instrument type MARGIN, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    /// Margin mode isolated, cross
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mgnMode")]
    pub mgn_mode: Option<TradeMode>,
    /// Instrument ID, only applicable to MARGIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// Liquidation currency, only applicable to cross MARGIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Underlying
    /// Required for FUTURES/SWAP/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    /// Required for FUTURES/SWAP/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// State: unfilled, filled
    /// unfilled by default
    /// Required for FUTURES/SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<OrdState>,
    /// this_week, next_week, quarter, next_quarter
    /// Required for FUTURES
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Alias>,
    /// Pagination of data to return records newer than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl LiquidationOrdersParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "mgnMode",
            self.mgn_mode.to_owned().unwrap_or_default(),
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
            "ccy",
            self.ccy.to_owned().unwrap_or_else(|| "".into()),
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "state",
            self.state.to_owned().unwrap_or(OrdState::Unfilled),
            "alias",
            self.alias.to_owned().unwrap_or_default(),
            "before",
            self.before.to_owned().unwrap_or_else(|| "".into()),
            "after",
            self.after.to_owned().unwrap_or_else(|| "".into()),
            "limit",
            self.limit.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidationOrdersData {
    #[serde(rename = "bkLoss")]
    pub bk_loss: String,
    #[serde(rename = "bkPx")]
    pub bk_px: String,
    pub ccy: String,
    #[serde(rename = "posSide")]
    pub pos_side: String,
    pub side: String,
    pub sz: String,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidationOrdersResponse {
    pub details: Vec<LiquidationOrdersData>,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "totalLoss")]
    pub total_loss: String,
    pub uly: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MarkPriceParams {
    /// Instrument type MARGIN, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    /// Underlying
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Instrument ID, only applicable to MARGIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
}

impl MarkPriceParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "uly",
            self.uly.to_owned().unwrap_or_default(),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkPriceResponse {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "markPx")]
    pub mark_px: String,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PositionTiersParams {
    /// Instrument type MARGIN, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    /// Trade mode
    /// Margin mode cross isolated
    #[serde(rename = "tdMode")]
    pub td_mode: TradeMode,
    /// Single underlying or multiple underlying (no more than 3) separated with comma.
    /// If instType is SWAP/FUTURES/OPTION, either uly or instFamily is required.
    /// If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Single instrument family or multiple instrument families (no more than 5) separated with comma.
    /// If instType is SWAP/FUTURES/OPTION, either uly or instFamily is required.
    /// If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Single instrument or multiple instruments (no more than 5) separated with comma.
    /// Either instId or ccy is required, if both are passed, instId will be used, ignore when instType is one of SWAP,FUTURES,OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Margin currency
    /// Only applicable to cross MARGIN. It will return borrowing amount for Multi-currency margin and Portfolio margin when ccy takes effect.
    pub ccy: Option<String>,
    /// Tiers
    pub tier: Option<String>,
}

impl PositionTiersParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "tdMode",
            self.td_mode,
            "uly",
            self.uly.to_owned().unwrap_or_default(),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
            "ccy",
            self.ccy.to_owned().unwrap_or_else(|| "".into()),
            "tier",
            self.tier.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PositionTiersResponse {
    #[serde(rename = "baseMaxLoan")]
    pub base_max_loan: String,
    pub imr: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "maxLever")]
    pub max_lever: String,
    #[serde(rename = "maxSz")]
    pub max_sz: String,
    #[serde(rename = "minSz")]
    pub min_sz: String,
    pub mmr: String,
    #[serde(rename = "optMgnFactor")]
    pub opt_mgn_factor: String,
    #[serde(rename = "quoteMaxLoan")]
    pub quote_max_loan: String,
    pub tier: String,
    pub uly: String,
    #[serde(rename = "instFamily")]
    pub inst_family: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InterestRateVIPData {
    #[serde(rename = "irDiscount")]
    pub ir_discount: String,
    #[serde(rename = "loanQuotaCoef")]
    pub loan_quota_coef: String,
    pub level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InterestRateBasicData {
    pub ccy: String,
    pub quota: String,
    pub rate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InterestRateResponse {
    pub basic: Vec<InterestRateBasicData>,
    pub vip: Vec<InterestRateVIPData>,
    pub regular: Vec<InterestRateVIPData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnderlyingParams {
    #[serde(rename = "instType")]
    pub inst_type: InstType,
}

impl UnderlyingParams {
    pub fn to_query(&self) -> String {
        format!("{}={}", "instType", self.inst_type,)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct InsuranceFundParams {
    /// Instrument type MARGIN, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    /// Type liquidation_balance_deposit bankruptcy_loss platform_revenue
    /// The default is all type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Instrument ID, only applicable to MARGIN
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// Underlying
    /// Required for FUTURES/SWAP/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family
    /// Required for FUTURES/SWAP/OPTION
    /// Either uly or instFamily is required. If both are passed, instFamily will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Currency, only applicable to MARGIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Pagination of data to return records newer than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination of data to return records earlier than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl InsuranceFundParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}&{}={}",
            "instType",
            self.inst_type,
            "type",
            self.r#type.to_owned().unwrap_or_default(),
            "uly",
            self.uly.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "ccy",
            self.ccy.to_owned().unwrap_or_else(|| "".into()),
            "before",
            self.before.to_owned().unwrap_or_else(|| "".into()),
            "after",
            self.after.to_owned().unwrap_or_else(|| "".into()),
            "limit",
            self.limit.to_owned().unwrap_or_default(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceFundData {
    pub amt: String,
    pub balance: String,
    pub ccy: String,
    pub ts: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsuranceFundResponse {
    pub details: Vec<InsuranceFundData>,
    #[serde(rename = "instFamily")]
    pub inst_family: String,
    pub total: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UnitConvertParams {
    /// Convert type
    /// 1: Convert currency to contract. It will be rounded up when the value of contract is decimal
    /// 2: Convert contract to currency
    /// The default is 1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Instrument ID, only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Quantity to buy or sell
    /// It is quantity of currency while converting currency to contract;
    /// It is quantity of contract while contract to currency. Quantity of coin needs to be positive integer
    pub sz: String,
    /// Order price
    /// For crypto-margined contracts, it is necessary while converting;
    /// For USDT-margined contracts, it is necessary while converting between usdt and contract, it is optional while converting between coin and contract.
    /// For OPTION, it is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// The unit of currency. coin usds: usdt or usdc, only applicable to USDⓈ-margined contracts from FUTURES/SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl UnitConvertParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}",
            "type",
            self.r#type.to_owned().unwrap_or_default(),
            "instId",
            self.inst_id,
            "sz",
            self.sz.to_owned(),
            "px",
            self.px.to_owned().unwrap_or_else(|| "".into()),
            "unit",
            self.unit.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitConvertResponse {
    #[serde(rename = "instId")]
    pub inst_id: String,
    pub px: String,
    pub sz: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OptionTradesParams {
    /// Instrument ID, e.g. BTC-USD-221230-4000-C, Either instId or instFamily is required. If both are passed, instId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    /// Instrument family, e.g. BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Option type, C: Call P: put
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "optType")]
    pub opt_type: Option<String>,
}

impl OptionTradesParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}",
            "instId",
            self.inst_id.to_owned().unwrap_or_else(|| "".into()),
            "instFamily",
            self.inst_family.to_owned().unwrap_or_else(|| "".into()),
            "optType",
            self.opt_type.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionTradesResponse {
    #[serde(rename = "fillVol")]
    pub fill_vol: String,
    #[serde(rename = "fwdPx")]
    pub fwd_px: String,
    #[serde(rename = "indexPx")]
    pub index_px: String,
    #[serde(rename = "instFamily")]
    pub inst_family: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "markPx")]
    pub mark_px: String,
    #[serde(rename = "optType")]
    pub opt_type: String,
    pub px: String,
    pub side: String,
    pub sz: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    pub ts: String,
}

impl PublicData {
    /// Retrieve a list of instruments with open contracts.
    pub fn get_instruments(
        &self,
        params: &InstrumentsParams,
    ) -> Result<ApiResponse<Vec<InstrumentsResponse>>> {
        let tickers: ApiResponse<Vec<InstrumentsResponse>> = self
            .client
            .get(API::PublicData(GetInstruments), Some(params.to_query()))?;

        Ok(tickers)
    }

    /// Retrieve delivery records of Futures and exercise records of Options in the last 3 months.
    pub fn get_delivery_hist(
        &self,
        params: &DeliveryHistParams,
    ) -> Result<ApiResponse<Vec<DeliveryHistResponse>>> {
        let delivery_hist: ApiResponse<Vec<DeliveryHistResponse>> = self
            .client
            .get(API::PublicData(GetDeliveryHist), Some(params.to_query()))?;

        Ok(delivery_hist)
    }

    /// Retrieve the total open interest for contracts on OKX.
    pub fn get_open_interest(
        &self,
        params: &OpenInterestParams,
    ) -> Result<ApiResponse<Vec<OpenInterestResponse>>> {
        let open_interest: ApiResponse<Vec<OpenInterestResponse>> = self
            .client
            .get(API::PublicData(GetOpenInterest), Some(params.to_query()))?;

        Ok(open_interest)
    }

    /// Retrieve funding rate.
    pub fn get_funding_rate(
        &self,
        params: &FundingRateParams,
    ) -> Result<ApiResponse<Vec<FundingRateResponse>>> {
        let funding_rate: ApiResponse<Vec<FundingRateResponse>> = self
            .client
            .get(API::PublicData(GetFundingRate), Some(params.to_query()))?;

        Ok(funding_rate)
    }

    /// Retrieve funding rate history. This endpoint can retrieve data from the last 3 months.
    pub fn get_funding_rate_hist(
        &self,
        params: &FundingRateHistParams,
    ) -> Result<ApiResponse<Vec<FundingRateHistResponse>>> {
        let funding_rate: ApiResponse<Vec<FundingRateHistResponse>> = self
            .client
            .get(API::PublicData(GetFundingRateHist), Some(params.to_query()))?;

        Ok(funding_rate)
    }

    /// Retrieve the highest buy limit and lowest sell limit of the instrument.
    pub fn get_limit_price(
        &self,
        params: &LimitPriceParams,
    ) -> Result<ApiResponse<Vec<LimitPriceResponse>>> {
        let limit_price: ApiResponse<Vec<LimitPriceResponse>> = self
            .client
            .get(API::PublicData(GetLimitPrice), Some(params.to_query()))?;

        Ok(limit_price)
    }

    /// Retrieve option market data.
    pub fn get_option_market_data(
        &self,
        params: &OptionMarketDataParams,
    ) -> Result<ApiResponse<Vec<OptionMarketDataResponse>>> {
        let option_data: ApiResponse<Vec<OptionMarketDataResponse>> = self.client.get(
            API::PublicData(GetOptionsMarketData),
            Some(params.to_query()),
        )?;

        Ok(option_data)
    }

    /// Retrieve the estimated delivery price which will only have a return value one hour before the delivery/exercise.
    pub fn get_estimated_delivery_price(
        &self,
        params: &EstimatedDeliveryPriceParams,
    ) -> Result<ApiResponse<Vec<EstimatedDeliveryPriceResponse>>> {
        let delivery_price: ApiResponse<Vec<EstimatedDeliveryPriceResponse>> = self.client.get(
            API::PublicData(GetEstimatedDeliveryPrice),
            Some(params.to_query()),
        )?;

        Ok(delivery_price)
    }

    /// Retrieve discount rate level and interest-free quota.
    pub fn get_discount_rate(
        &self,
        params: &DiscountRateParams,
    ) -> Result<ApiResponse<Vec<DiscountRateResponse>>> {
        let discount_rate: ApiResponse<Vec<DiscountRateResponse>> = self
            .client
            .get(API::PublicData(GetDiscountRate), Some(params.to_query()))?;

        Ok(discount_rate)
    }

    /// Retrieve API server time.
    pub fn get_system_time(&self) -> Result<ApiResponse<Vec<SystemTimeResponse>>> {
        let system_time: ApiResponse<Vec<SystemTimeResponse>> =
            self.client.get(API::PublicData(GetSystemTime), None)?;

        Ok(system_time)
    }

    /// Retrieve information on liquidation orders in the last day.
    pub fn get_liquidation_orders(
        &self,
        params: &LiquidationOrdersParams,
    ) -> Result<ApiResponse<Vec<LiquidationOrdersResponse>>> {
        let liquidations: ApiResponse<Vec<LiquidationOrdersResponse>> = self.client.get(
            API::PublicData(GetLiquidationOrders),
            Some(params.to_query()),
        )?;

        Ok(liquidations)
    }

    /// Retrieve mark price.
    /// We set the mark price based on the SPOT index and at a reasonable basis to prevent individual users from manipulating the market and causing the contract price to fluctuate.
    pub fn get_mark_price(
        &self,
        params: &MarkPriceParams,
    ) -> Result<ApiResponse<Vec<MarkPriceResponse>>> {
        let mark_price: ApiResponse<Vec<MarkPriceResponse>> = self
            .client
            .get(API::PublicData(GetMarkPrice), Some(params.to_query()))?;

        Ok(mark_price)
    }

    /// Retrieve position tiers information, maximum leverage depends on your borrowings and margin ratio.
    pub fn get_position_tiers(
        &self,
        params: &PositionTiersParams,
    ) -> Result<ApiResponse<Vec<PositionTiersResponse>>> {
        let position_tiers: ApiResponse<Vec<PositionTiersResponse>> = self
            .client
            .get(API::PublicData(GetPositionTiers), Some(params.to_query()))?;

        Ok(position_tiers)
    }

    /// Retrieve interest rate
    pub fn get_interest_rate(&self) -> Result<ApiResponse<Vec<InterestRateResponse>>> {
        let interest_rates: ApiResponse<Vec<InterestRateResponse>> =
            self.client.get(API::PublicData(GetInterestRate), None)?;

        Ok(interest_rates)
    }

    /// Get underlying
    pub fn get_underlying(
        &self,
        params: &UnderlyingParams,
    ) -> Result<ApiResponse<Vec<Vec<String>>>> {
        let underlying: ApiResponse<Vec<Vec<String>>> = self
            .client
            .get(API::PublicData(GetUnderlying), Some(params.to_query()))?;

        Ok(underlying)
    }

    /// Get insurance fund balance information
    pub fn get_insurance_fund(
        &self,
        params: &InsuranceFundParams,
    ) -> Result<ApiResponse<Vec<InsuranceFundResponse>>> {
        let insurance_fund: ApiResponse<Vec<InsuranceFundResponse>> = self
            .client
            .get(API::PublicData(GetInsuranceFund), Some(params.to_query()))?;

        Ok(insurance_fund)
    }

    /// Convert the crypto value to the number of contracts, or vice versa
    pub fn unit_convert(
        &self,
        params: &UnitConvertParams,
    ) -> Result<ApiResponse<Vec<UnitConvertResponse>>> {
        let unit_convert: ApiResponse<Vec<UnitConvertResponse>> = self
            .client
            .get(API::PublicData(UnitConvert), Some(params.to_query()))?;

        Ok(unit_convert)
    }

    /// Get option trades. The maximum is 100.
    pub fn get_options_trades(
        &self,
        params: &OptionTradesParams,
    ) -> Result<ApiResponse<Vec<OptionTradesResponse>>> {
        let option_trades: ApiResponse<Vec<OptionTradesResponse>> = self
            .client
            .get(API::PublicData(GetOptionTrades), Some(params.to_query()))?;

        Ok(option_trades)
    }
}
