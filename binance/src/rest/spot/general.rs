use crate::commons::errors::*;
use crate::rest::api::{Spot, API};
use crate::rest::client::Client;
use crate::rest::model::{Empty, ExchangeInformation, ServerTime, Symbol};

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    // Test connectivity
    pub fn ping(&self) -> Result<String> {
        self.client.get::<Empty>(API::Spot(Spot::Ping), None)?;
        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get(API::Spot(Spot::Time), None)
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get(API::Spot(Spot::ExchangeInfo), None)
    }

    // Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info() {
            Ok(info) => {
                info.symbols
                .into_iter()
                .find(|item| item.symbol == upper_symbol)
                .ok_or(BinanceError::SymbolNotFound)
            }
            Err(e) => Err(e),
        }
    }
}
