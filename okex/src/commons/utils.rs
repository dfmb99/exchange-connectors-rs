use crate::commons::errors::*;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum InstType {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "FUTURES")]
    Futures,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "MARGIN")]
    Margin,
    #[serde(rename = "")]
    #[default]
    Empty,
}

impl Display for InstType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spot => write!(f, "SPOT"),
            Self::Swap => write!(f, "SWAP"),
            Self::Futures => write!(f, "FUTURES"),
            Self::Option => write!(f, "OPTION"),
            Self::Margin => write!(f, "MARGIN"),
            Self::Empty => write!(f, ""),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum TradeMode {
    #[serde(rename = "cross")]
    Cross,
    #[serde(rename = "isolated")]
    Isolated,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "")]
    #[default]
    Empty,
}

impl Display for TradeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cross => write!(f, "cross"),
            Self::Isolated => write!(f, "isolated"),
            Self::Cash => write!(f, "cash"),
            Self::Empty => write!(f, ""),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum Side {
    #[serde(rename = "buy")]
    #[default]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "buy"),
            Self::Sell => write!(f, "sell"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum PosSide {
    #[serde(rename = "net")]
    #[default]
    Net,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
}

impl Display for PosSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Net => write!(f, "net"),
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum OrdType {
    #[serde(rename = "market")]
    #[default]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "post_only")]
    PostOnly,
    #[serde(rename = "fok")]
    FillOrKill,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "optimal_limit_ioc")]
    OptimalImmediateOrCancel,
}

impl Display for OrdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Market => write!(f, "market"),
            Self::Limit => write!(f, "limit"),
            Self::PostOnly => write!(f, "post_only"),
            Self::FillOrKill => write!(f, "fok"),
            Self::ImmediateOrCancel => write!(f, "ioc"),
            Self::OptimalImmediateOrCancel => write!(f, "optimal_limit_ioc"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum TriggerPriceType {
    #[serde(rename = "last")]
    #[default]
    Last,
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "mark")]
    Mark,
}

impl Display for TriggerPriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Last => write!(f, "last"),
            Self::Index => write!(f, "index"),
            Self::Mark => write!(f, "mark"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum OrdState {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "unfilled")]
    Unfilled,
    #[serde(rename = "")]
    #[default]
    Empty,
}

impl Display for OrdState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Canceled => write!(f, "canceled"),
            Self::Live => write!(f, "live"),
            Self::PartiallyFilled => write!(f, "partially_filled"),
            Self::Filled => write!(f, "filled"),
            Self::Unfilled => write!(f, "unfilled"),
            Self::Empty => write!(f, ""),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum OrdCategory {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "twap")]
    Twap,
    #[serde(rename = "adl")]
    Adl,
    #[serde(rename = "full_liquidation")]
    FullLiquidation,
    #[serde(rename = "partial_liquidation")]
    PartialLiquidation,
    #[serde(rename = "delivery")]
    Delivery,
    #[serde(rename = "ddh")]
    Ddh,
    #[serde(rename = "")]
    #[default]
    Empty,
}

impl Display for OrdCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Twap => write!(f, "twap"),
            Self::Adl => write!(f, "adl"),
            Self::FullLiquidation => write!(f, "full_liquidation"),
            Self::PartialLiquidation => write!(f, "partial_liquidation"),
            Self::Delivery => write!(f, "delivery"),
            Self::Ddh => write!(f, "ddh"),
            Self::Empty => write!(f, ""),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum QuickMgnType {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "auto_borrow")]
    AutoBorrow,
    #[serde(rename = "auto_repay")]
    AutoRepay,
    #[serde(rename = "")]
    #[default]
    Empty,
}

impl Display for QuickMgnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::AutoBorrow => write!(f, "auto_borrow"),
            Self::AutoRepay => write!(f, "auto_repay"),
            Self::Empty => write!(f, ""),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum ExecType {
    #[serde(rename = "T")]
    Taker,
    #[serde(rename = "M")]
    Maker,
    #[serde(rename = "")]
    #[default]
    Empty,
}

impl Display for ExecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Taker => write!(f, "T"),
            Self::Maker => write!(f, "M"),
            Self::Empty => write!(f, ""),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum Alias {
    #[serde(rename = "this_week")]
    ThisWeek,
    #[serde(rename = "next_week")]
    NextWeek,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "next_quarter")]
    NextQuarter,
    #[serde(rename = "")]
    #[default]
    Empty,
}

impl Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThisWeek => write!(f, "this_week"),
            Self::NextWeek => write!(f, "next_week"),
            Self::Quarter => write!(f, "quarter"),
            Self::NextQuarter => write!(f, "next_quarter"),
            Self::Empty => write!(f, ""),
        }
    }
}

pub fn get_unix_timestamp(start: SystemTime) -> Result<u64> {
    let since_epoch = start.duration_since(UNIX_EPOCH)?;
    Ok(since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000)
}

pub fn get_timestamp_iso_format(now: SystemTime) -> String {
    let now: DateTime<Utc> = now.into();
    now.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn build_request(parameters: BTreeMap<String, String>) -> String {
    let mut request = String::new();
    for (key, value) in parameters {
        let param = format!("{key}={value}&");
        request.push_str(param.as_ref());
    }
    request.pop();
    request
}
