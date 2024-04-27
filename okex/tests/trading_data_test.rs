use dotenv::dotenv;
use mockito::{Matcher, mock};
use okx::commons::config::Config;
use okx::rest::api::Okx;
use okx::rest::trading_data::{
    ContractsOIVolumeParams, LongShortRatioParams, MarginLendingRatioParams, OptionsOIVolumeParams,
    PutCallRatioParams, TakerFlowParams, TakerVolumeParams, TradingData,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_support_coins_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/rubik/stat/trading-data/support-coin")
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/mocks/trading_data/get_support_coins.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: TradingData = Okx::new_with_config(None, None, None, &config);

        let response = public_data.get_support_coins().unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data.contract[0], "ADA".to_string())
    }

    #[test]
    fn get_taker_volume_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/rubik/stat/taker-volume")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("ccy=BTC&.*instType=SPOT".into()))
            .with_body_from_file("tests/mocks/trading_data/get_taker_volume.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: TradingData = Okx::new_with_config(None, None, None, &config);

        let params = TakerVolumeParams {
            ccy: "BTC".to_string(),
            inst_type: "SPOT".to_string(),
            ..Default::default()
        };
        let response = public_data.get_taker_volume(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0][0], "1630425600000".to_string());
    }

    #[test]
    fn get_margin_lending_ratio_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/rubik/stat/margin/loan-ratio")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("ccy=BTC".into()))
            .with_body_from_file("tests/mocks/trading_data/get_margin_lending_ratio.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: TradingData = Okx::new_with_config(None, None, None, &config);

        let params = MarginLendingRatioParams {
            ccy: "BTC".to_string(),
            ..Default::default()
        };

        let response = public_data.get_margin_lending_ratio(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0][0], "1630492800000".to_string());
    }

    #[test]
    fn get_long_short_ratio_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock(
            "GET",
            "/api/v5/rubik/stat/contracts/long-short-account-ratio",
        )
        .with_header("content-type", "application/json")
        .match_query(Matcher::Regex("ccy=BTC".into()))
        .with_body_from_file("tests/mocks/trading_data/get_long_short_ratio.json")
        .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: TradingData = Okx::new_with_config(None, None, None, &config);

        let params = LongShortRatioParams {
            ccy: "BTC".to_string(),
            ..Default::default()
        };

        let response = public_data.get_long_short_ratio(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0][0], "1630502100000".to_string());
    }

    #[test]
    fn get_contracts_oi_volume_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/rubik/stat/contracts/open-interest-volume")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("ccy=BTC".into()))
            .with_body_from_file("tests/mocks/trading_data/get_contracts_oi_volume.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: TradingData = Okx::new_with_config(None, None, None, &config);

        let params = ContractsOIVolumeParams {
            ccy: "BTC".to_string(),
            ..Default::default()
        };

        let response = public_data.get_contracts_oi_volume(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0][0], "1630502400000".to_string());
    }

    #[test]
    fn get_options_oi_volume_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/rubik/stat/option/open-interest-volume")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("ccy=BTC".into()))
            .with_body_from_file("tests/mocks/trading_data/get_options_oi_volume.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: TradingData = Okx::new_with_config(None, None, None, &config);

        let params = OptionsOIVolumeParams {
            ccy: "BTC".to_string(),
            ..Default::default()
        };

        let response = public_data.get_options_oi_volume(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0][0], "1630368000000".to_string());
    }

    #[test]
    fn get_put_call_ratio_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock(
            "GET",
            "/api/v5/rubik/stat/option/open-interest-volume-ratio",
        )
        .with_header("content-type", "application/json")
        .match_query(Matcher::Regex("ccy=BTC".into()))
        .with_body_from_file("tests/mocks/trading_data/get_put_call_ratio.json")
        .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: TradingData = Okx::new_with_config(None, None, None, &config);

        let params = PutCallRatioParams {
            ccy: "BTC".to_string(),
            ..Default::default()
        };

        let response = public_data.get_put_call_ratio(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0][1], "2.7261".to_string());
    }

    #[test]
    fn get_taker_flow_test() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/rubik/stat/option/taker-block-volume")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("ccy=BTC".into()))
            .with_body_from_file("tests/mocks/trading_data/get_taker_flow.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: TradingData = Okx::new_with_config(None, None, None, &config);

        let params = TakerFlowParams {
            ccy: "BTC".to_string(),
            ..Default::default()
        };

        let response = public_data.get_taker_flow(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[6], "40.7".to_string());
    }
}
