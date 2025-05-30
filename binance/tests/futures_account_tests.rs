use binance::commons::config::Config;
use binance::rest::api::Binance;
use binance::rest::futures::account::{CustomOrderRequest, FuturesAccount, OrderType};
use binance::rest::futures::model::{Order, Transaction};
use binance::rest::spot::account::OrderSide;

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;
    use mockito::{Matcher, Server};

    #[test]
    fn change_initial_leverage() {
        let mut server = Server::new();
        let mock_change_leverage = server
            .mock("POST", "/fapi/v1/leverage")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "leverage=2&recvWindow=1234&symbol=LTCUSDT&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_initial_leverage.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let response = account.change_initial_leverage("LTCUSDT", 2).unwrap();

        mock_change_leverage.assert();

        assert_eq!(response.leverage, 2);
        assert_eq!(response.symbol, "LTCUSDT");
        assert!(approx_eq!(
            f64,
            response.max_notional_value,
            9223372036854776000.0,
            ulps = 2
        ));
    }

    #[test]
    fn cancel_all_open_orders() {
        let mut server = Server::new();
        let mock = server
            .mock("DELETE", "/fapi/v1/allOpenOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/cancel_all_open_orders.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.cancel_all_open_orders("BTCUSDT").unwrap();

        mock.assert();
    }

    #[test]
    fn change_position_mode() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/fapi/v1/positionSide/dual")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "dualSidePosition=true&recvWindow=1234&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_position_mode.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.change_position_mode(true).unwrap();

        mock.assert();
    }

    #[test]
    fn stop_market_close_buy() {
        let mut server = Server::new();
        let mock_stop_market_close_sell = server.mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=TRUE&recvWindow=1234&side=BUY&stopPrice=10.5&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_buy.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.stop_market_close_buy("SRMUSDT", 10.5).unwrap();

        mock_stop_market_close_sell.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "BUY");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert!(approx_eq!(f64, transaction.stop_price, 10.5, ulps = 2));
    }

    #[test]
    fn stop_market_close_sell() {
        let mut server = Server::new();
        let mock_stop_market_close_sell = server.mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=TRUE&recvWindow=1234&side=SELL&stopPrice=7.4&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_sell.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.stop_market_close_sell("SRMUSDT", 7.4).unwrap();

        mock_stop_market_close_sell.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "SELL");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert!(approx_eq!(f64, transaction.stop_price, 7.4, ulps = 2));
    }

    #[test]
    fn custom_order() {
        let mut server = Server::new();
        let mock_custom_order = server.mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=TRUE&recvWindow=1234&side=SELL&stopPrice=7.4&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_sell.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let custom_order = CustomOrderRequest {
            symbol: "SRMUSDT".into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::StopMarket,
            time_in_force: None,
            qty: None,
            reduce_only: None,
            price: None,
            stop_price: Some(7.4),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let transaction: Transaction = account.custom_order(custom_order).unwrap();

        mock_custom_order.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "SELL");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert!(approx_eq!(f64, transaction.stop_price, 7.4, ulps = 2));
    }

    #[test]
    fn order_status() {
        let mut server = Server::new();
        let mock_order_status = server
            .mock("GET", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "orderId=1917641&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/order_status.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let order_status: Order = account.get_order("BTCUSDT", 1917641).unwrap();

        mock_order_status.assert();

        assert_eq!(order_status.symbol, "BTCUSDT");
        assert_eq!(order_status.order_id, 1917641);
        assert_eq!(order_status.client_order_id, "abc");
        assert_eq!(order_status.price, 0.0);
        assert_eq!(order_status.orig_qty, 0.40);
        assert_eq!(order_status.executed_qty, 0.0);
        assert_eq!(order_status.status, "NEW");
        assert_eq!(order_status.time_in_force, "GTC"); //Migrate to TimeInForce enum
        assert_eq!(order_status.side, "BUY");
        assert_eq!(order_status.update_time, 1579276756075);
    }
}
