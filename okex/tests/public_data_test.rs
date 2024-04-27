use dotenv::dotenv;
use mockito::{Matcher, mock};
use okx::commons::config::Config;
use okx::commons::utils::{InstType, TradeMode};
use okx::rest::api::Okx;
use okx::rest::public_data::{
    DeliveryHistParams, DiscountRateParams, EstimatedDeliveryPriceParams, FundingRateHistParams,
    FundingRateParams, InstrumentsParams, InsuranceFundParams, LimitPriceParams,
    LiquidationOrdersParams, MarkPriceParams, OpenInterestParams, OptionMarketDataParams,
    OptionTradesParams, PositionTiersParams, PublicData, UnderlyingParams, UnitConvertParams,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_instruments_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/instruments")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SWAP&.*instId=LTC-USD-SWAP".into()))
            .with_body_from_file("tests/mocks/public_data/get_instruments.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = InstrumentsParams {
            inst_type: InstType::Swap,
            uly: None,
            inst_family: None,
            inst_id: Some("LTC-USD-SWAP".into()),
        };
        let response = public_data.get_instruments(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].inst_id, "LTC-USD-SWAP".to_string());
    }

    #[test]
    fn get_delivery_hist_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/delivery-exercise-history")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=OPTION&.*uly=BTC-USD".into()))
            .with_body_from_file("tests/mocks/public_data/get_delivery.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = DeliveryHistParams {
            inst_type: InstType::Option,
            uly: Some("BTC-USD".into()),
            ..Default::default()
        };
        let response = public_data.get_delivery_hist(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(
            response.data[0].details[0].inst_id,
            "BTC-USD-190927".to_string()
        );
    }

    #[test]
    fn get_open_interest_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/open-interest")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SWAP&.*uly=BTC-USDT-SWAP".into()))
            .with_body_from_file("tests/mocks/public_data/get_open_interest.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = OpenInterestParams {
            inst_type: InstType::Swap,
            uly: Some("BTC-USDT-SWAP".into()),
            ..Default::default()
        };
        let response = public_data.get_open_interest(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].oi, "5000".to_string())
    }

    #[test]
    fn get_funding_rate_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/funding-rate")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USD-SWAP".into()))
            .with_body_from_file("tests/mocks/public_data/get_funding_rate.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = FundingRateParams {
            inst_id: "BTC-USD-SWAP".to_string(),
        };
        let response = public_data.get_funding_rate(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].funding_rate, "0.0001515".to_string())
    }

    #[test]
    fn get_funding_rate_hist_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/funding-rate-history")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USD-SWAP".into()))
            .with_body_from_file("tests/mocks/public_data/get_funding_rate_history.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = FundingRateHistParams {
            inst_id: "BTC-USD-SWAP".to_string(),
            ..Default::default()
        };
        let response = public_data.get_funding_rate_hist(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].funding_rate, "0.018".to_string());
        assert_eq!(response.data[0].funding_time, "1597026383085".to_string());
    }

    #[test]
    fn get_limit_price_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/price-limit")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USDT-SWAP".into()))
            .with_body_from_file("tests/mocks/public_data/get_limit_price.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = LimitPriceParams {
            inst_id: "BTC-USDT-SWAP".to_string(),
        };
        let response = public_data.get_limit_price(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].buy_lmt, "17057.9".to_string());
        assert_eq!(response.data[0].sell_lmt, "16388.9".to_string());
    }

    #[test]
    fn get_option_market_data_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/opt-summary")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("uly=BTC-USD".into()))
            .with_body_from_file("tests/mocks/public_data/get_option_market_data.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = OptionMarketDataParams {
            uly: Some("BTC-USD".to_string()),
            ..Default::default()
        };
        let response = public_data.get_option_market_data(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(
            response.data[0].inst_id,
            "BTC-USD-200103-5500-C".to_string()
        );
        assert_eq!(
            response.data[1].inst_id,
            "BTC-USD-200103-6500-C".to_string()
        );
    }

    #[test]
    fn get_estimated_delivery_price_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/estimated-price")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=BTC-USDT-201227".into()))
            .with_body_from_file("tests/mocks/public_data/get_estimated_delivery_price.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = EstimatedDeliveryPriceParams {
            inst_id: "BTC-USDT-201227".to_string(),
        };
        let response = public_data.get_estimated_delivery_price(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].inst_id, "BTC-USDT-201227".to_string());
    }

    #[test]
    fn get_discount_rate_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/discount-rate-interest-free-quota")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("ccy=LTC".into()))
            .with_body_from_file("tests/mocks/public_data/get_discount_rate.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = DiscountRateParams {
            ccy: Some("LTC".into()),
            ..Default::default()
        };
        let public_data = public_data.get_discount_rate(&params).unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].ccy, "LTC".to_string());
    }

    #[test]
    fn get_system_time_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/time")
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/mocks/public_data/get_system_time.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let public_data = public_data.get_system_time().unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].ts, "1597026383085".to_string());
    }

    #[test]
    fn get_liquidation_orders_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/liquidation-orders")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=MARGIN&.*uly=BTC".into()))
            .with_body_from_file("tests/mocks/public_data/get_liquidation_orders.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = LiquidationOrdersParams {
            inst_type: InstType::Margin,
            uly: Some("BTC".into()),
            ..Default::default()
        };
        let public_data = public_data.get_liquidation_orders(&params).unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].inst_id, "BTC-USDT".to_string());
    }

    #[test]
    fn get_mark_price_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/mark-price")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SWAP".into()))
            .with_body_from_file("tests/mocks/public_data/get_mark_price.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = MarkPriceParams {
            inst_type: InstType::Swap,
            ..Default::default()
        };
        let public_data = public_data.get_mark_price(&params).unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].mark_px, "200".to_string());
    }

    #[test]
    fn get_position_tiers_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/position-tiers")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SWAP.*tdMode=isolated".into()))
            .with_body_from_file("tests/mocks/public_data/get_position_tiers.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = PositionTiersParams {
            inst_type: InstType::Swap,
            td_mode: TradeMode::Isolated,
            ..Default::default()
        };
        let public_data = public_data.get_position_tiers(&params).unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].inst_id, "BTC-USDT".to_string());
    }

    #[test]
    fn get_interest_rate_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/interest-rate-loan-quota")
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/mocks/public_data/get_interest_rate.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let public_data = public_data.get_interest_rate().unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].basic[0].rate, "0.00043728".to_string());
        assert_eq!(public_data.data[0].vip[0].ir_discount, "0.7".to_string());
    }

    #[test]
    fn get_underlying_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/underlying")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SPOT".into()))
            .with_body_from_file("tests/mocks/public_data/get_underlying.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = UnderlyingParams {
            inst_type: InstType::Spot,
        };
        let public_data = public_data.get_underlying(&params).unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0][0], "LTC-USDT".to_string());
    }

    #[test]
    fn get_insurance_fund_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/insurance-fund")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SWAP&.*uly=BTC-USD".into()))
            .with_body_from_file("tests/mocks/public_data/get_insurance_fund.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = InsuranceFundParams {
            inst_type: InstType::Swap,
            uly: Some("BTC-USD".into()),
            ..Default::default()
        };
        let public_data = public_data.get_insurance_fund(&params).unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].details[0].amt, "0.2465".to_string());
    }

    #[test]
    fn unit_convert_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/convert-contract-coin")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex(
                "instId=BTC-USD-SWAP&.*sz=0.888&.*px=35000".into(),
            ))
            .with_body_from_file("tests/mocks/public_data/unit_convert.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = UnitConvertParams {
            inst_id: "BTC-USD-SWAP".to_string(),
            sz: "0.888".to_string(),
            px: Some("35000".into()),
            ..Default::default()
        };
        let public_data = public_data.unit_convert(&params).unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].sz, "311".to_string());
    }

    #[test]
    fn get_options_trades_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/public/option-trades")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instFamily=BTC-USDT".into()))
            .with_body_from_file("tests/mocks/public_data/get_option_trades.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: PublicData = Okx::new_with_config(None, None, None, &config);

        let params = OptionTradesParams {
            inst_family: Some("BTC-USDT".into()),
            ..Default::default()
        };
        let public_data = public_data.get_options_trades(&params).unwrap();

        mock.assert();

        assert_eq!(public_data.code, "0".to_string());
        assert_eq!(public_data.data[0].index_px, "16667".to_string());
        assert_eq!(public_data.data[0].px, "0.005".to_string());
    }
}
