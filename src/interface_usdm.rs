use crate::api::Binance;
use crate::config::Config;
use crate::futures::account::FuturesAccount;
use crate::futures::general::FuturesGeneral;
use crate::futures::market::FuturesMarket;
use crate::interface_usdm_data::UsdmData;
use crate::ws_usdm::WsInterface;
use crate::errors::*;
use crate::futures::model::{ExchangeInformation, Symbol};
use crate::model::{ServerTime};

struct UsdmInterface {
    symbol: String,
    general: FuturesGeneral,
    account: FuturesAccount,
    market: FuturesMarket,
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
        UsdmInterface {
            symbol,
            general: Binance::new_with_config(api_key.to_owned(), api_secret.to_owned(), config),
            account: Binance::new_with_config(api_key.to_owned(), api_secret.to_owned(), config),
            market: Binance::new_with_config(api_key.to_owned(), api_secret.to_owned(), config),
            ws: WsInterface::new(symbol.to_owned(), api_key.to_owned(), api_secret.to_owned(), config),
            data: UsdmData::default()
        }
    }

    /// Test connectivity
    pub fn ping(&self) -> Result<String> {
        self.general.ping()
    }

    /// Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        self.general.get_server_time()
    }

    /// Obtain exchange information
    /// - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.general.exchange_info()
    }

    /// Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
        where
            S: Into<String>,
    {
        self.general.get_symbol_info(symbol)
    }

}

fn update_last_day_klines(data: UsdmData) {

}
