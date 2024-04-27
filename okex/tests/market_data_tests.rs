use okx::commons::config::Config;
use okx::rest::api::Okx;
use okx::rest::market_data::MarketData;
use mockito::{mock, Matcher};
use dotenv::dotenv;
use okx::rest::market_data::{
    CandleSticksParams, IndexTickerParams, OrderBookParams, TickerParams, TickersParams,
    TradesParams,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tickers_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock_get_market_data = mock("GET", "/api/v5/market/tickers")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SWAP".into()))
            .with_body_from_file("tests/mocks/market_data/get_tickers.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let market_data: MarketData = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = TickersParams {
            inst_type: "SWAP".into(),
            uly: None,
            inst_family: None,
        };
        let market_data = market_data.get_tickers(&params).unwrap();

        mock_get_market_data.assert();

        assert_eq!(market_data.code, "0".to_string());
        assert_eq!(market_data.data[0].inst_id, "LTC-USD-SWAP".to_string());
    }

    #[test]
    fn get_ticker_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock_get_market_data = mock("GET", "/api/v5/market/ticker")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USD-SWAP".into()))
            .with_body_from_file("tests/mocks/market_data/get_ticker.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let market_data: MarketData = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = TickerParams {
            inst_id: "BTC-USD-SWAP".to_string(),
        };
        let market_data = market_data.get_ticker(&params).unwrap();

        mock_get_market_data.assert();

        assert_eq!(market_data.code, "0".to_string());
        assert_eq!(market_data.data[0].inst_id, "BTC-USD-SWAP".to_string());
    }

    #[test]
    fn get_index_tickers_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock_get_market_data = mock("GET", "/api/v5/market/index-tickers")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USDT".into()))
            .with_body_from_file("tests/mocks/market_data/get_index_tickers.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let market_data: MarketData = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = IndexTickerParams {
            inst_id: Some("BTC-USDT".to_string()),
            ..Default::default()
        };
        let market_data = market_data.get_index_tickers(&params).unwrap();

        mock_get_market_data.assert();

        assert_eq!(market_data.code, "0".to_string());
        assert_eq!(market_data.data[0].inst_id, "BTC-USDT".to_string());
    }

    #[test]
    fn get_order_book_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock_get_market_data = mock("GET", "/api/v5/market/books")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USDT".into()))
            .with_body_from_file("tests/mocks/market_data/get_order_book.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let market_data: MarketData = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = OrderBookParams {
            inst_id: "BTC-USDT".to_string(),
            ..Default::default()
        };
        let market_data = market_data.get_order_book(&params).unwrap();

        mock_get_market_data.assert();

        assert_eq!(market_data.code, "0".to_string());
        assert!(!market_data.data[0].asks.is_empty());
        assert_eq!(market_data.data[0].asks[0].num_orders, "1".to_string());
        assert!(!market_data.data[0].bids.is_empty());
        assert_eq!(market_data.data[0].bids[0].num_orders, "2".to_string());
    }

    #[test]
    fn get_order_book_lite_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock_get_market_data = mock("GET", "/api/v5/market/books-lite")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USDT".into()))
            .with_body_from_file("tests/mocks/market_data/get_order_book.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let market_data: MarketData = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = OrderBookParams {
            inst_id: "BTC-USDT".to_string(),
            ..Default::default()
        };
        let market_data = market_data.get_order_book_lite(&params).unwrap();

        mock_get_market_data.assert();

        assert_eq!(market_data.code, "0".to_string());
        assert!(!market_data.data[0].asks.is_empty());
        assert_eq!(market_data.data[0].asks[0].num_orders, "1".to_string());
        assert!(!market_data.data[0].bids.is_empty());
        assert_eq!(market_data.data[0].bids[0].num_orders, "2".to_string());
    }

    #[test]
    fn get_candles_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock_get_market_data = mock("GET", "/api/v5/market/candles")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USDT".into()))
            .with_body_from_file("tests/mocks/market_data/get_candles.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let market_data: MarketData = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = CandleSticksParams {
            inst_id: "BTC-USDT".to_string(),
            ..Default::default()
        };
        let market_data = market_data.get_candles(&params).unwrap();

        mock_get_market_data.assert();

        assert_eq!(market_data.code, "0".to_string());
        assert_eq!(market_data.data[0].ts, "1597026383085".to_string());
        assert_eq!(market_data.data[0].confirm, "0".to_string());
        assert_eq!(market_data.data[1].ts, "1597026383085".to_string());
        assert_eq!(market_data.data[1].confirm, "1".to_string());
    }

    #[test]
    fn get_index_candles_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock_get_market_data = mock("GET", "/api/v5/market/index-candles")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USDT".into()))
            .with_body_from_file("tests/mocks/market_data/get_index_candles.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let market_data: MarketData = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = CandleSticksParams {
            inst_id: "BTC-USDT".to_string(),
            ..Default::default()
        };
        let market_data = market_data.get_index_candles(&params).unwrap();

        mock_get_market_data.assert();

        assert_eq!(market_data.code, "0".to_string());
        assert_eq!(market_data.data[0].ts, "1597026383085".to_string());
        assert_eq!(market_data.data[0].confirm, "0".to_string());
        assert_eq!(market_data.data[1].ts, "1597026383085".to_string());
        assert_eq!(market_data.data[1].confirm, "1".to_string());
    }

    #[test]
    fn get_trades_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock_get_market_data = mock("GET", "/api/v5/market/trades")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USDT".into()))
            .with_body_from_file("tests/mocks/market_data/get_trades.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let market_data: MarketData = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = TradesParams {
            inst_id: "BTC-USDT".to_string(),
            ..Default::default()
        };
        let market_data = market_data.get_trades(&params).unwrap();

        mock_get_market_data.assert();

        assert_eq!(market_data.code, "0".to_string());
        assert_eq!(market_data.data[0].inst_id, "BTC-USDT".to_string());
        assert_eq!(market_data.data[0].trade_id, "242720720".to_string());
    }
}
