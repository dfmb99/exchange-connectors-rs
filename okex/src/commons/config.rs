#[derive(Clone, Debug)]
pub struct Config {
    pub rest_endpoint: String,
    pub public_ws_endpoint: String,
    pub private_ws_endpoint: String,
    pub simulated_trading: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_endpoint: "https://www.okx.com".into(),
            public_ws_endpoint: "wss://ws.okx.com:8443/ws/v5/public".into(),
            private_ws_endpoint: "wss://ws.okx.com:8443/ws/v5/private".into(),
            simulated_trading: false,
        }
    }
}

impl Config {
    pub fn testnet() -> Self {
        Self::default()
            .set_rest_endpoint("https://www.okx.com")
            .set_public_ws_endpoint("wss://wspap.okx.com:8443/ws/v5/public?brokerId=9999")
            .set_private_ws_endpoint("wss://wspap.okx.com:8443/ws/v5/private?brokerId=9999")
            .simulated_trading(true)
    }

    pub fn set_rest_endpoint<T: Into<String>>(mut self, rest_endpoint: T) -> Self {
        self.rest_endpoint = rest_endpoint.into();
        self
    }

    pub fn set_public_ws_endpoint<T: Into<String>>(mut self, public_ws_endpoint: T) -> Self {
        self.public_ws_endpoint = public_ws_endpoint.into();
        self
    }

    pub fn set_private_ws_endpoint<T: Into<String>>(mut self, private_ws_endpoint: T) -> Self {
        self.private_ws_endpoint = private_ws_endpoint.into();
        self
    }
    pub fn simulated_trading(mut self, simulated_trading: bool) -> Self {
        self.simulated_trading = simulated_trading;
        self
    }
}
