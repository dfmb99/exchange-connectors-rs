use crate::commons::client::Client;
use crate::commons::config::Config;
use serde::{Deserialize, Serialize};

#[allow(clippy::all)]
pub enum API {
    Trade(Trade),
    Account(Account),
    MarketData(MarketData),
    PublicData(PublicData),
    TradingData(TradingData),
    Status(Status),
}

#[derive(Clone, Copy)]
pub enum Trade {
    PlaceOrder,
    PlaceMultipleOrders,
    CancelOrder,
    CancelMultipleOrders,
    AmendOrder,
    AmendMultipleOrders,
    ClosePositions,
    GetOrderDetails,
    GetOrderList,
    GetOrderHist,
    GetFills,
    GetFillsHist,
    PlaceAlgoOrder,
    CancelAlgoOrder,
    GetAlgoOrders,
}

#[derive(Clone, Copy)]
pub enum Account {
    GetBalance,
    GetPosition,
    GetAccountPositionRisk,
    GetBillsDetails,
    SetPositionMode,
    SetLeverage,
    GetLeverage,
    GetFeeRates,
}

#[derive(Clone, Copy)]
pub enum MarketData {
    GetTickers,
    GetTicker,
    GetIndexTickers,
    GetOrderBook,
    GetOrderBookLite,
    GetCandles,
    GetCandlesHist,
    GetIndexCandles,
    GetIndexCandlesHist,
    GetMarkPriceCandles,
    GetMarkPriceCandlesHist,
    GetTrades,
    GetTradesHist,
    GetOptionsTrades,
    Get24hVolume,
    GetOracle,
    GetExchangeRate,
    GetIndexComponents,
}

#[derive(Clone, Copy)]
pub enum PublicData {
    GetInstruments,
    GetDeliveryHist,
    GetOpenInterest,
    GetFundingRate,
    GetFundingRateHist,
    GetLimitPrice,
    GetOptionsMarketData,
    GetEstimatedDeliveryPrice,
    GetDiscountRate,
    GetSystemTime,
    GetLiquidationOrders,
    GetMarkPrice,
    GetPositionTiers,
    GetInterestRate,
    GetUnderlying,
    GetInsuranceFund,
    UnitConvert,
    GetOptionTrades,
}

#[derive(Clone, Copy)]
pub enum TradingData {
    GetSupportCoin,
    GetTakerVolume,
    GetMarginLendingRatio,
    GetLongShortRatio,
    GetContractsOIVolume,
    GetOptionsOIVolume,
    GetPutCallRatio,
    GetTakerFlow,
}
#[derive(Clone, Copy)]
pub enum Status {
    SystemStatus,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Trade(route) => match route {
                Trade::PlaceOrder => "/api/v5/trade/order",
                Trade::PlaceMultipleOrders => "/api/v5/trade/batch-orders",
                Trade::CancelOrder => "/api/v5/trade/cancel-order",
                Trade::CancelMultipleOrders => "/api/v5/trade/cancel-batch-orders",
                Trade::AmendOrder => "/api/v5/trade/amend-order",
                Trade::AmendMultipleOrders => "/api/v5/trade/amend-batch-orders",
                Trade::ClosePositions => "/api/v5/trade/close-position",
                Trade::GetOrderDetails => "/api/v5/trade/order",
                Trade::GetOrderList => "/api/v5/trade/orders-pending",
                Trade::GetOrderHist => "/api/v5/trade/orders-history-archive",
                Trade::GetFills => "/api/v5/trade/fills",
                Trade::GetFillsHist => "/api/v5/trade/fills-history",
                Trade::PlaceAlgoOrder => "/api/v5/trade/order-algo",
                Trade::CancelAlgoOrder => "/api/v5/trade/cancel-algos",
                Trade::GetAlgoOrders => "/api/v5/trade/orders-algo-pending",
            },
            API::Account(route) => match route {
                Account::GetBalance => "/api/v5/account/balance",
                Account::GetPosition => "/api/v5/account/positions",
                Account::GetAccountPositionRisk => "/api/v5/account/account-position-risk",
                Account::GetBillsDetails => "/api/v5/account/bills",
                Account::SetPositionMode => "/api/v5/account/set-position-mode",
                Account::SetLeverage => "/api/v5/account/set-leverage",
                Account::GetLeverage => "/api/v5/account/leverage-info",
                Account::GetFeeRates => "/api/v5/account/trade-fee",
            },
            API::MarketData(route) => match route {
                MarketData::GetTickers => "/api/v5/market/tickers",
                MarketData::GetTicker => "/api/v5/market/ticker",
                MarketData::GetIndexTickers => "/api/v5/market/index-tickers",
                MarketData::GetOrderBook => "/api/v5/market/books",
                MarketData::GetOrderBookLite => "/api/v5/market/books-lite",
                MarketData::GetCandles => "/api/v5/market/candles",
                MarketData::GetCandlesHist => "/api/v5/market/history-candles",
                MarketData::GetIndexCandles => "/api/v5/market/index-candles",
                MarketData::GetIndexCandlesHist => "/api/v5/market/history-index-candles",
                MarketData::GetMarkPriceCandles => "/api/v5/market/mark-price-candles",
                MarketData::GetMarkPriceCandlesHist => "/api/v5/market/history-mark-price-candles",
                MarketData::GetTrades => "/api/v5/market/trades",
                MarketData::GetTradesHist => "/api/v5/market/history-trades",
                MarketData::GetOptionsTrades => "/api/v5/market/option/instrument-family-trades",
                MarketData::Get24hVolume => "/api/v5/market/platform-24-volume",
                MarketData::GetOracle => "/api/v5/market/open-oracle",
                MarketData::GetExchangeRate => "/api/v5/market/exchange-rate",
                MarketData::GetIndexComponents => "/api/v5/market/index-components",
            },
            API::PublicData(route) => match route {
                PublicData::GetInstruments => "/api/v5/public/instruments",
                PublicData::GetDeliveryHist => "/api/v5/public/delivery-exercise-history",
                PublicData::GetOpenInterest => "/api/v5/public/open-interest",
                PublicData::GetFundingRate => "/api/v5/public/funding-rate",
                PublicData::GetFundingRateHist => "/api/v5/public/funding-rate-history",
                PublicData::GetLimitPrice => "/api/v5/public/price-limit",
                PublicData::GetOptionsMarketData => "/api/v5/public/opt-summary",
                PublicData::GetEstimatedDeliveryPrice => "/api/v5/public/estimated-price",
                PublicData::GetDiscountRate => "/api/v5/public/discount-rate-interest-free-quota",
                PublicData::GetSystemTime => "/api/v5/public/time",
                PublicData::GetLiquidationOrders => "/api/v5/public/liquidation-orders",
                PublicData::GetMarkPrice => "/api/v5/public/mark-price",
                PublicData::GetPositionTiers => "/api/v5/public/position-tiers",
                PublicData::GetInterestRate => "/api/v5/public/interest-rate-loan-quota",
                PublicData::GetUnderlying => "/api/v5/public/underlying",
                PublicData::GetInsuranceFund => "/api/v5/public/insurance-fund",
                PublicData::UnitConvert => "/api/v5/public/convert-contract-coin",
                PublicData::GetOptionTrades => "/api/v5/public/option-trades",
            },
            API::TradingData(route) => match route {
                TradingData::GetSupportCoin => "/api/v5/rubik/stat/trading-data/support-coin",
                TradingData::GetTakerVolume => "/api/v5/rubik/stat/taker-volume",
                TradingData::GetMarginLendingRatio => "/api/v5/rubik/stat/margin/loan-ratio",
                TradingData::GetLongShortRatio => {
                    "/api/v5/rubik/stat/contracts/long-short-account-ratio"
                }
                TradingData::GetContractsOIVolume => {
                    "/api/v5/rubik/stat/contracts/open-interest-volume"
                }
                TradingData::GetOptionsOIVolume => "/api/v5/rubik/stat/option/open-interest-volume",
                TradingData::GetPutCallRatio => {
                    "/api/v5/rubik/stat/option/open-interest-volume-ratio"
                }
                TradingData::GetTakerFlow => "/api/v5/rubik/stat/option/taker-block-volume",
            },
            API::Status(route) => match route {
                Status::SystemStatus => "/api/v5/system/status",
            },
        })
    }
}

#[derive(Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: String,
    pub msg: String,
    pub data: T,
}

pub trait Okx {
    fn new(api_key: Option<String>, secret_key: Option<String>, passphrase: Option<String>)
        -> Self;
    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
        config: &Config,
    ) -> Self;
}

use crate::rest::account::Account as RestAccount;
use crate::rest::market_data::MarketData as RestMarketData;
use crate::rest::public_data::PublicData as RestPublicData;
use crate::rest::status::Status as RestStatus;
use crate::rest::trade::Trade as RestTrade;
use crate::rest::trading_data::TradingData as RestTradingData;

impl Okx for RestMarketData {
    fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        Self::new_with_config(api_key, secret_key, passphrase, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
        config: &Config,
    ) -> RestMarketData {
        RestMarketData {
            client: Client::new(
                api_key,
                secret_key,
                passphrase,
                config.rest_endpoint.clone(),
                config.simulated_trading,
            ),
        }
    }
}

impl Okx for RestTrade {
    fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        Self::new_with_config(api_key, secret_key, passphrase, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
        config: &Config,
    ) -> RestTrade {
        RestTrade {
            client: Client::new(
                api_key,
                secret_key,
                passphrase,
                config.rest_endpoint.clone(),
                config.simulated_trading,
            ),
        }
    }
}

impl Okx for RestAccount {
    fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        Self::new_with_config(api_key, secret_key, passphrase, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
        config: &Config,
    ) -> RestAccount {
        RestAccount {
            client: Client::new(
                api_key,
                secret_key,
                passphrase,
                config.rest_endpoint.clone(),
                config.simulated_trading,
            ),
        }
    }
}

impl Okx for RestPublicData {
    fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        Self::new_with_config(api_key, secret_key, passphrase, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
        config: &Config,
    ) -> RestPublicData {
        RestPublicData {
            client: Client::new(
                api_key,
                secret_key,
                passphrase,
                config.rest_endpoint.clone(),
                config.simulated_trading,
            ),
        }
    }
}

impl Okx for RestTradingData {
    fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        Self::new_with_config(api_key, secret_key, passphrase, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
        config: &Config,
    ) -> RestTradingData {
        RestTradingData {
            client: Client::new(
                api_key,
                secret_key,
                passphrase,
                config.rest_endpoint.clone(),
                config.simulated_trading,
            ),
        }
    }
}

impl Okx for RestStatus {
    fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        Self::new_with_config(api_key, secret_key, passphrase, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        passphrase: Option<String>,
        config: &Config,
    ) -> RestStatus {
        RestStatus {
            client: Client::new(
                api_key,
                secret_key,
                passphrase,
                config.rest_endpoint.clone(),
                config.simulated_trading,
            ),
        }
    }
}
