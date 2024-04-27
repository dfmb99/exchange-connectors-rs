use crate::commons::errors::*;
use crate::commons::client::Client;
use crate::rest::api::{API, ApiResponse};
use crate::rest::api::TradingData::{
    GetContractsOIVolume, GetLongShortRatio, GetMarginLendingRatio, GetOptionsOIVolume,
    GetPutCallRatio, GetSupportCoin, GetTakerFlow, GetTakerVolume,
};
use crate::serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct TradingData {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportCoinsResponse {
    pub contract: Vec<String>,
    pub option: Vec<String>,
    pub spot: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TakerVolumeParams {
    /// Currency
    pub ccy: String,
    /// Instrument type SPOT, CONTRACTS
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Begin time, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// End time, e.g. 1597026383011
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Period, the default is 5m, e.g. [5m/1H/1D]
    /// 5m granularity can only query data within two days at most
    /// 1H granularity can only query data within 30 days at most
    /// 1D granularity can only query data within 180 days at most
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl TakerVolumeParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}&{}={}",
            "ccy",
            self.ccy,
            "instType",
            self.inst_type,
            "begin",
            self.begin.to_owned().unwrap_or_else(|| "".into()),
            "end",
            self.end.to_owned().unwrap_or_else(|| "".into()),
            "period",
            self.period.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct MarginLendingRatioParams {
    /// Currency
    pub ccy: String,
    /// Begin time, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// End time, e.g. 1597026383011
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Period, the default is 5m, e.g. [5m/1H/1D]
    /// 5m granularity can only query data within two days at most
    /// 1H granularity can only query data within 30 days at most
    /// 1D granularity can only query data within 180 days at most
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl MarginLendingRatioParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "ccy",
            self.ccy,
            "begin",
            self.begin.to_owned().unwrap_or_else(|| "".into()),
            "end",
            self.end.to_owned().unwrap_or_else(|| "".into()),
            "period",
            self.period.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct LongShortRatioParams {
    /// Currency
    pub ccy: String,
    /// Begin time, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// End time, e.g. 1597026383011
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Period, the default is 5m, e.g. [5m/1H/1D]
    /// 5m granularity can only query data within two days at most
    /// 1H granularity can only query data within 30 days at most
    /// 1D granularity can only query data within 180 days at most
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl LongShortRatioParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "ccy",
            self.ccy,
            "begin",
            self.begin.to_owned().unwrap_or_else(|| "".into()),
            "end",
            self.end.to_owned().unwrap_or_else(|| "".into()),
            "period",
            self.period.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ContractsOIVolumeParams {
    /// Currency
    pub ccy: String,
    /// Begin time, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// End time, e.g. 1597026383011
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Period, the default is 5m, e.g. [5m/1H/1D]
    /// 5m granularity can only query data within two days at most
    /// 1H granularity can only query data within 30 days at most
    /// 1D granularity can only query data within 180 days at most
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl ContractsOIVolumeParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "ccy",
            self.ccy,
            "begin",
            self.begin.to_owned().unwrap_or_else(|| "".into()),
            "end",
            self.end.to_owned().unwrap_or_else(|| "".into()),
            "period",
            self.period.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct OptionsOIVolumeParams {
    /// Currency
    pub ccy: String,
    /// Period, the default is 8H. e.g. [8H/1D]
    /// Each granularity can only query 72 pieces of data at the earliest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl OptionsOIVolumeParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}",
            "ccy",
            self.ccy,
            "begin",
            self.period.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PutCallRatioParams {
    /// Currency
    pub ccy: String,
    /// Period, the default is 8H. e.g. [8H/1D]
    /// Each granularity can only query 72 pieces of data at the earliest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl PutCallRatioParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}",
            "ccy",
            self.ccy,
            "begin",
            self.period.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TakerFlowParams {
    /// Currency
    pub ccy: String,
    /// Period, the default is 8H. e.g. [8H/1D]
    /// Each granularity can only query 72 pieces of data at the earliest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

impl TakerFlowParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}",
            "ccy",
            self.ccy,
            "begin",
            self.period.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

impl TradingData {
    /// Retrieve the currencies supported by the trading data endpoints.
    pub fn get_support_coins(&self) -> Result<ApiResponse<SupportCoinsResponse>> {
        let support_coins: ApiResponse<SupportCoinsResponse> =
            self.client.get(API::TradingData(GetSupportCoin), None)?;

        Ok(support_coins)
    }

    /// Retrieve the taker volume for both buyers and sellers.
    pub fn get_taker_volume(
        &self, params: &TakerVolumeParams,
    ) -> Result<ApiResponse<Vec<Vec<String>>>> {
        let taker_flow: ApiResponse<Vec<Vec<String>>> = self
            .client
            .get(API::TradingData(GetTakerVolume), Some(params.to_query()))?;

        Ok(taker_flow)
    }

    /// Retrieve the ratio of cumulative amount between currency margin quote currency and base currency.
    pub fn get_margin_lending_ratio(
        &self, params: &MarginLendingRatioParams,
    ) -> Result<ApiResponse<Vec<Vec<String>>>> {
        let margin_lending_ratio: ApiResponse<Vec<Vec<String>>> = self.client.get(
            API::TradingData(GetMarginLendingRatio),
            Some(params.to_query()),
        )?;

        Ok(margin_lending_ratio)
    }

    /// Retrieve the ratio of users with net long vs net short positions for futures and perpetual swaps.
    pub fn get_long_short_ratio(
        &self, params: &LongShortRatioParams,
    ) -> Result<ApiResponse<Vec<Vec<String>>>> {
        let long_short_ratio: ApiResponse<Vec<Vec<String>>> = self
            .client
            .get(API::TradingData(GetLongShortRatio), Some(params.to_query()))?;

        Ok(long_short_ratio)
    }

    /// Retrieve the open interest and trading volume for futures and perpetual swaps.
    pub fn get_contracts_oi_volume(
        &self, params: &ContractsOIVolumeParams,
    ) -> Result<ApiResponse<Vec<Vec<String>>>> {
        let contracts_data: ApiResponse<Vec<Vec<String>>> = self.client.get(
            API::TradingData(GetContractsOIVolume),
            Some(params.to_query()),
        )?;

        Ok(contracts_data)
    }

    /// Retrieve the open interest and trading volume for options.
    pub fn get_options_oi_volume(
        &self, params: &OptionsOIVolumeParams,
    ) -> Result<ApiResponse<Vec<Vec<String>>>> {
        let options_data: ApiResponse<Vec<Vec<String>>> = self.client.get(
            API::TradingData(GetOptionsOIVolume),
            Some(params.to_query()),
        )?;

        Ok(options_data)
    }

    /// Retrieve the open interest ratio and trading volume ratio of calls vs puts.
    pub fn get_put_call_ratio(
        &self, params: &PutCallRatioParams,
    ) -> Result<ApiResponse<Vec<Vec<String>>>> {
        let put_call_ratio: ApiResponse<Vec<Vec<String>>> = self
            .client
            .get(API::TradingData(GetPutCallRatio), Some(params.to_query()))?;

        Ok(put_call_ratio)
    }

    /// This shows the relative buy/sell volume for calls and puts. It shows whether traders are bullish or bearish on price and volatility.
    pub fn get_taker_flow(&self, params: &TakerFlowParams) -> Result<ApiResponse<Vec<String>>> {
        let taker_flow: ApiResponse<Vec<String>> = self
            .client
            .get(API::TradingData(GetTakerFlow), Some(params.to_query()))?;

        Ok(taker_flow)
    }
}
