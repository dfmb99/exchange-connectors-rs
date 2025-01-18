use binance::commons::util::build_request;
use binance::commons::util::build_signed_request_custom;

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::collections::BTreeMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn build_request_empty() {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let result = build_request(parameters);
        assert!(result.is_empty());
    }

    #[test]
    fn build_request_not_empty() {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("recvWindow".into(), "1234".to_string());
        let result = build_request(parameters);
        assert_eq!(result, format!("recvWindow={}", 1234));
    }

    #[test]
    fn build_signed_request() {
        let now = SystemTime::now();
        let recv_window = 1234;

        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let timestamp =
            since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000;

        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let result = build_signed_request_custom(parameters, recv_window, now).unwrap();

        assert_eq!(
            result,
            format!("recvWindow={recv_window}&timestamp={timestamp}")
        );
    }
}
