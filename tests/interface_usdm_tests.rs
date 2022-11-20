use binance::interface_usdm::UsdmInterface;
use binance::config::Config;
use binance::interface_usdm_data::UsdmConfig;
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_time_test() {
        let api_key_user =
            Some("f7349ef10fed52e0282e9c66d7269acfb046d70d8b48f0ca34733e67322471c9".into());
        let api_secret_user =
            Some("7dedd32206a93e7d86f84372940a74e762711cd0800833a1e5fe56e6ed059cc1".into());
        let config = Config::testnet();
        let usdm_config = UsdmConfig::default();
        let usdm = UsdmInterface::new(
            "btcusdt".to_string(),
            api_key_user,
            api_secret_user,
            &config,
            usdm_config,
        );

        let result = usdm.get_server_time();
        assert!(result.is_ok());
    }

    #[test]
    fn interface_usdm_test() {
        let api_key_user =
            Some("f7349ef10fed52e0282e9c66d7269acfb046d70d8b48f0ca34733e67322471c9".into());
        let api_secret_user =
            Some("7dedd32206a93e7d86f84372940a74e762711cd0800833a1e5fe56e6ed059cc1".into());
        let config = Config::testnet();
        let usdm_config = UsdmConfig::default();
        let usdm = UsdmInterface::new(
            "btcusdt".to_string(),
            api_key_user,
            api_secret_user,
            &config,
            usdm_config,
        );

        // Market buys 0.001 btcusdt
        let buy = usdm.market_buy("btcusdt".to_string(), 0.001);
        print!("{:?}", buy);
        assert!(buy.is_ok());

        // Market sells 0.001 btcusdt
        let sell = usdm.market_sell("btcusdt".to_string(), 0.001);
        print!("{:?}", sell);
        assert!(sell.is_ok());

        // Places limit buy order 0.001 btcusdt @ 10000.0
        let limit_buy = usdm.limit_buy("btcusdt".to_string(), 0.001, 10000.0);
        print!("{:?}", limit_buy);
        assert!(limit_buy.is_ok());

        // Places limit buy order 0.001 btcusdt @ 100000.0
        let limit_sell = usdm.limit_sell("btcusdt".to_string(), 0.001, 100000.0);
        print!("{:?}", limit_sell);
        assert!(limit_sell.is_ok());

        // Cancel previous limit sell order placed
        let canceled_sell = usdm.cancel_order("btcusdt".to_string(), limit_sell.as_ref().unwrap().order_id);
        print!("{:?}", canceled_sell);
        assert!(canceled_sell.is_ok());

        // waits for ws data to update
        thread::sleep(Duration::from_millis(3000));

        // Gets current position
        let pos = usdm.get_position_size_ws();
        print!("{:?}", pos);
        assert!(pos.is_some());
        assert_eq!(pos.unwrap(), 0.0);

        // Check open orders
        let open_orders = usdm.get_open_orders_ws();
        print!("{:?}", open_orders);
        assert_eq!(open_orders.len(), 1);
        assert_eq!(open_orders.get(0).unwrap().order_id, limit_buy.as_ref().unwrap().order_id);

        // Check open orders
        let filled_orders = usdm.get_filled_orders_ws();
        print!("{:?}", filled_orders);
        assert_eq!(filled_orders.len(), 2);
        assert_eq!(filled_orders.get(0).unwrap().order_id, buy.as_ref().unwrap().order_id);
        assert_eq!(filled_orders.get(1).unwrap().order_id, sell.as_ref().unwrap().order_id);

        // Check open orders
        let canceled_orders = usdm.get_canceled_orders_ws();
        print!("{:?}", canceled_orders);
        assert_eq!(canceled_orders.len(), 1);
        assert_eq!(canceled_orders.get(0).unwrap().order_id, limit_sell.as_ref().unwrap().order_id);

        // Cancel previous limit buy order placed
        let canceled_buy = usdm.cancel_order("btcusdt".to_string(), limit_buy.as_ref().unwrap().order_id);
        print!("{:?}", canceled_buy);
        assert!(canceled_buy.is_ok());

        // waits for ws data to update
        thread::sleep(Duration::from_millis(3000));

        let aggr_trades = usdm.get_aggr_trades_ws();
        print!("{:?}", aggr_trades);
        assert!(aggr_trades.len() > 1);
        assert!(aggr_trades.get(aggr_trades.len() - 1).unwrap().trade_order_time > aggr_trades.get(0).unwrap().trade_order_time);
    }

    #[test]
    fn mark_price_snaps_test() {
        let api_key_user =
            Some("f7349ef10fed52e0282e9c66d7269acfb046d70d8b48f0ca34733e67322471c9".into());
        let api_secret_user =
            Some("7dedd32206a93e7d86f84372940a74e762711cd0800833a1e5fe56e6ed059cc1".into());
        let config = Config::testnet();
        let usdm_config = UsdmConfig::default();
        let usdm = UsdmInterface::new(
            "btcusdt".to_string(),
            api_key_user,
            api_secret_user,
            &config,
            usdm_config,
        );

        // waits to collect minium mark price snaps
        thread::sleep(Duration::from_millis(60000));

        let mark_price_snaps = usdm.get_mark_price_snaps_ws();
        assert!(mark_price_snaps.len() > 3 );
        assert!(mark_price_snaps.get(mark_price_snaps.len() - 1).unwrap().event_time > mark_price_snaps.get(0).unwrap().event_time);
    }
}
