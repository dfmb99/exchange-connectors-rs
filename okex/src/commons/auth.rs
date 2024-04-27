use base64::engine::general_purpose;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Generate signature
pub fn get_signature(
    secret_key: &str, timestamp: &str, method: &str, request_path: &str, body: &str,
) -> String {
    let mut signed_key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
    let data = format!("{timestamp}{method}{request_path}{body}");
    signed_key.update(data.as_bytes());
    general_purpose::STANDARD.encode(signed_key.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let sig = get_signature(
            "123",
            "2020-12-08T09:08:57.715Z",
            "GET",
            "/api/v5/account/balance",
            "",
        );
        assert_eq!(
            sig,
            "3PMX5upw5mbYhVovlpA3YPhdjzDAZ1GbOzKYLBdaHLo=".to_string()
        );
    }
}
