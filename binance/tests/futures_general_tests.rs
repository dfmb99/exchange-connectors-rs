use binance::commons::config::Config;
use binance::rest::api::Binance;

#[cfg(test)]
mod tests {
    use super::*;
    use binance::rest::futures::general::FuturesGeneral;
    use mockito::Server;

    #[test]
    fn exchange_info() {
        let mut server = Server::new();
        let mock_exchange_info = server
            .mock("GET", "/fapi/v1/exchangeInfo")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/futures/general/exchange_info.json")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let general: FuturesGeneral = Binance::new_with_config(None, None, &config);

        let exchange_info = general.exchange_info().unwrap();
        mock_exchange_info.assert();

        assert!(exchange_info.server_time == 1565613908500);
    }
}
