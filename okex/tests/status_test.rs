use dotenv::dotenv;
use mockito::{Matcher, mock};
use okx::commons::config::Config;
use okx::rest::api::Okx;
use okx::rest::status::{Status, SystemStatusParams};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_system_status() {
        dotenv().ok();
        let _ = env_logger::try_init();

        let mock = mock("GET", "/api/v5/system/status")
            .with_header("content-type", "application/json")
            .match_query(Matcher::Regex("state=canceled".into()))
            .with_body_from_file("tests/mocks/status/get_system_status.json")
            .create();

        let config = Config::default().set_rest_endpoint(mockito::server_url());
        let public_data: Status = Okx::new_with_config(None, None, None, &config);

        let params = SystemStatusParams {
            state: Some("canceled".into()),
        };
        let response = public_data.get_system_status(&params).unwrap();

        mock.assert();

        assert_eq!(response.code, "0".to_string());
        assert_eq!(response.data[0].end, "1672823520000".to_string())
    }
}
