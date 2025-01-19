use mockito::{Matcher, Server};
use okex::commons::config::Config;
use okex::commons::utils::TradeMode;
use okex::rest::account::{
    Account, AccountPositionRiskParams, BalanceParams, BillsDetailsParams, FeeRatesParams,
    GetLeverageParams, PositionModeParams, PositionsParams, SetLeverageParams,
};
use okex::rest::api::Okx;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_balance_test() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v5/account/balance")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("ccy=USDT,BTC".into()))
            .with_body_from_file("tests/mocks/account/get_balance.json")
            .create();

        let config = Config::default().set_rest_endpoint(server.url());
        let account: Account = Okx::new_with_config(None, None, None, &config);

        let params = BalanceParams {
            ccy: Some("USDT,BTC".into()),
        };
        let account = account.get_balance(&params).unwrap();

        mock.assert();

        assert_eq!(account.code, "0".to_string());
        assert_eq!(account.data[0].details[0].ccy, "USDT".to_string());
        assert_eq!(account.data[0].details[1].ccy, "BTC".to_string());
    }

    #[test]
    fn get_positions_test() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v5/account/positions")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instId=ETH-USD-210430".into()))
            .with_body_from_file("tests/mocks/account/get_positions.json")
            .create();

        let config = Config::default().set_rest_endpoint(server.url());
        let account: Account = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = PositionsParams {
            inst_id: Some("ETH-USD-210430".into()),
            ..Default::default()
        };
        let account = account.get_positions(&params).unwrap();

        mock.assert();

        assert_eq!(account.code, "0".to_string());
        assert_eq!(account.data[0].inst_id, "ETH-USD-210430".to_string());
        assert_eq!(
            account.data[0].close_order_algo[0].algo_id,
            "123".to_string()
        );
    }

    #[test]
    fn get_acc_position_risk_test() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v5/account/account-position-risk")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=".into()))
            .with_body_from_file("tests/mocks/account/get_acc_position_risk.json")
            .create();

        let config = Config::default().set_rest_endpoint(server.url());
        let account: Account = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = AccountPositionRiskParams::default();
        let account = account.get_acc_position_risk(&params).unwrap();

        mock.assert();

        assert_eq!(account.code, "0".to_string());
        assert_eq!(account.data[0].ts, "1620282889345".to_string());
    }

    #[test]
    fn set_position_mode_test() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/api/v5/account/set-position-mode")
            .with_header("content-type", "application/json")
            .match_body(r#"{"posMode":"long_short_mode"}"#)
            .with_body_from_file("tests/mocks/account/set_position_mode.json")
            .create();

        let config = Config::default().set_rest_endpoint(server.url());
        let account: Account = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = PositionModeParams {
            pos_mode: "long_short_mode".into(),
        };
        let account = account.set_position_mode(&params).unwrap();

        mock.assert();

        assert_eq!(account.code, "0".to_string());
        assert_eq!(account.data[0].pos_mode, "long_short_mode".to_string());
    }

    #[test]
    fn set_leverage_test() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/api/v5/account/set-leverage")
            .with_header("content-type", "application/json")
            .match_body(r#"{"instId":"BTC-USDT-SWAP","lever":"30","mgnMode":"isolated"}"#)
            .with_body_from_file("tests/mocks/account/set_leverage.json")
            .create();

        let config = Config::default().set_rest_endpoint(server.url());
        let account: Account = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = SetLeverageParams {
            inst_id: Some("BTC-USDT-SWAP".into()),
            lever: "30".into(),
            mgn_mode: TradeMode::Isolated,
            ..Default::default()
        };
        let account = account.set_leverage(&params).unwrap();

        mock.assert();

        assert_eq!(account.code, "0".to_string());
        assert_eq!(account.data[0].lever, "30".to_string());
    }

    #[test]
    fn get_leverage_test() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v5/account/leverage-info")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex(
                "instId=BTC-USDT-200626&mgnMode=cross".into(),
            ))
            .with_body_from_file("tests/mocks/account/get_leverage.json")
            .create();

        let config = Config::default().set_rest_endpoint(server.url());
        let account: Account = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = GetLeverageParams {
            inst_id: "BTC-USDT-200626".into(),
            mgn_mode: TradeMode::Cross,
        };
        let account = account.get_leverage(&params).unwrap();

        mock.assert();

        assert_eq!(account.code, "0".to_string());
        assert_eq!(account.data[0].lever, "10".to_string());
    }

    #[test]
    fn get_bills_details_test() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v5/account/bills")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SPOT".into()))
            .with_body_from_file("tests/mocks/account/get_bills_details.json")
            .create();

        let config = Config::default().set_rest_endpoint(server.url());
        let account: Account = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = BillsDetailsParams {
            inst_type: Some("SPOT".into()),
            ..Default::default()
        };
        let account = account.get_bills_details(&params).unwrap();

        mock.assert();

        assert_eq!(account.code, "0".to_string());
        assert_eq!(account.data[0].bill_id, "374241568911437826".to_string());
    }

    #[test]
    fn get_fee_rates_test() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/api/v5/account/trade-fee")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("instType=SPOT&instId=BTC-USDT".into()))
            .with_body_from_file("tests/mocks/account/get_fee_rates.json")
            .create();

        let config = Config::default().set_rest_endpoint(server.url());
        let account: Account = Okx::new_with_config(None, None, None, &config);
        let _ = env_logger::try_init();

        let params = FeeRatesParams {
            inst_type: "SPOT".into(),
            inst_id: Some("BTC-USDT".into()),
            ..Default::default()
        };
        let account = account.get_fee_rates(&params).unwrap();

        mock.assert();

        assert_eq!(account.code, "0".to_string());
        assert_eq!(account.data[0].inst_type.to_string(), "SPOT".to_string());
    }
}
