use crate::commons::errors::BinanceError;
use crate::rest::api::{Futures, API};
use crate::rest::client::Client;
use crate::rest::futures::model::{ExchangeInformation, Symbol};
use crate::rest::model::ServerTime;

#[derive(Clone)]
pub struct FuturesGeneral {
    pub client: Client,
}

impl FuturesGeneral {
    // Test connectivity
    pub fn ping(&self) -> Result<String, BinanceError> {
        self.client.get::<()>(API::Futures(Futures::Ping), None)?;
        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime, BinanceError> {
        self.client.get(API::Futures(Futures::Time), None)
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation, BinanceError> {
        self.client.get(API::Futures(Futures::ExchangeInfo), None)
    }

    // Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol, BinanceError>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info() {
            Ok(info) => info
                .symbols
                .into_iter()
                .find(|item| item.symbol == upper_symbol)
                .ok_or(BinanceError::SymbolNotFound),
            Err(e) => Err(e),
        }
    }
}
