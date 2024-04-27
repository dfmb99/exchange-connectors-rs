use okx::commons::config::Config;
use okx::rest::api::Okx;
use std::env::var;
use dotenv::dotenv;
use mockito::{mock, Matcher};
use okx::commons::utils::{OrdType, Side, TradeMode};
use okx::rest::trade::{
    PlaceOrderParams, Trade, CancelOrderParams, AmendOrderParams, ClosePositionParams,
    OrderDetailsParams, OrderListParams, FillsParams,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_order_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("POST", "/api/v5/trade/order")
            .with_header("content-type", "application/json")
            .match_body(r#"{"instId":"BTC-USDT","tdMode":"cash","side":"buy","ordType":"market","sz":"0.01"}"#)
            .with_body_from_file("tests/mocks/trade/place_order.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = PlaceOrderParams {
            inst_id: "BTC-USDT".into(),
            side: Side::Buy,
            ord_type: OrdType::Market,
            sz: "0.01".into(),
            td_mode: TradeMode::Cash,
            ..Default::default()
        };
        let response = trade.place_order(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
    }

    #[test]
    fn cancel_order_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("POST", "/api/v5/trade/cancel-order")
            .with_header("content-type", "application/json")
            .match_body(r#"{"instId":"BTC-USDT","ordId":"12345689"}"#)
            .with_body_from_file("tests/mocks/trade/cancel_order.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = CancelOrderParams {
            inst_id: "BTC-USDT".into(),
            ord_id: Some("12345689".into()),
            ..Default::default()
        };

        let response = trade.cancel_order(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].ord_id, "12345689".to_string());
    }

    #[test]
    fn amend_order_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("POST", "/api/v5/trade/amend-order")
            .with_header("content-type", "application/json")
            .match_body(r#"{"instId":"BTC-USDT","ordId":"12344","newSz":"2"}"#)
            .with_body_from_file("tests/mocks/trade/amend_order.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = AmendOrderParams {
            inst_id: "BTC-USDT".into(),
            ord_id: Some("12344".into()),
            new_sz: Some("2".into()),
            ..Default::default()
        };

        let response = trade.amend_order(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].ord_id, "12344".to_string());
    }

    #[test]
    fn close_position_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("POST", "/api/v5/trade/close-position")
            .with_header("content-type", "application/json")
            .match_body(r#"{"instId":"BTC-USDT-SWAP","mgnMode":"cross"}"#)
            .with_body_from_file("tests/mocks/trade/close_position.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = ClosePositionParams {
            inst_id: "BTC-USDT-SWAP".into(),
            mgn_mode: TradeMode::Cross,
            ..Default::default()
        };

        let response = trade.close_position(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].inst_id, "BTC-USDT-SWAP".to_string());
    }

    #[test]
    fn get_order_details_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("GET", "/api/v5/trade/order")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex(
                "instId=BTC-USD-200329&ordId=312269865356374016".into(),
            ))
            .with_body_from_file("tests/mocks/trade/get_order_details.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = OrderDetailsParams {
            inst_id: "BTC-USD-200329".into(),
            ord_id: Some("312269865356374016".into()),
            ..Default::default()
        };

        let response = trade.get_order_details(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].inst_id, "BTC-USD-200329".to_string());
        assert_eq!(response.data[0].ord_id, "312269865356374016".to_string());
    }

    #[test]
    fn get_order_list_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("GET", "/api/v5/trade/orders-pending")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex(
                "instType=SPOT&.*ordType=post_only,fok,ioc".into(),
            ))
            .with_body_from_file("tests/mocks/trade/get_order_list.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = OrderListParams {
            inst_type: Some("SPOT".into()),
            ord_type: Some("post_only,fok,ioc".into()),
            ..Default::default()
        };

        let response = trade.get_order_list(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].inst_id, "BTC-USDT".to_string());
        assert_eq!(response.data[0].ord_id, "301835739059335168".to_string());
    }

    #[test]
    fn get_order_hist_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("GET", "/api/v5/trade/orders-history-archive")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex(
                "instType=FUTURES&.*ordType=post_only,fok,ioc".into(),
            ))
            .with_body_from_file("tests/mocks/trade/get_order_history.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = OrderListParams {
            inst_type: Some("FUTURES".into()),
            ord_type: Some("post_only,fok,ioc".into()),
            ..Default::default()
        };

        let response = trade.get_order_hist(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].inst_id, "BTC-USD-200329".to_string());
        assert_eq!(response.data[0].ord_id, "312269865356374016".to_string());
    }

    #[test]
    fn get_fills_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("GET", "/api/v5/trade/fills")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USD-200329".into()))
            .with_body_from_file("tests/mocks/trade/get_fills.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = FillsParams {
            inst_id: Some("BTC-USD-200329".into()),
            ..Default::default()
        };

        let response = trade.get_fills(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].inst_id, "BTC-USD-200329".to_string());
    }

    #[test]
    fn get_fills_hist_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let api_key = var("API_KEY").expect("API_KEY is not defined.");
        let api_secret = var("API_SECRET").expect("API_SECRET is not defined.");
        let passphrase = var("PASSPHRASE").expect("PASSPHRASE is not defined.");

        let mock = mock("GET", "/api/v5/trade/fills-history")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USD-200329".into()))
            .with_body_from_file("tests/mocks/trade/get_fills.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let trade: Trade =
            Okx::new_with_config(Some(api_key), Some(api_secret), Some(passphrase), &config);

        let params = FillsParams {
            inst_id: Some("BTC-USD-200329".into()),
            ..Default::default()
        };

        let response = trade.get_fills_hist(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].inst_id, "BTC-USD-200329".to_string());
    }
}
