use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::str;

#[derive(Clone, Debug)]
pub enum AuthData {
    Data { key: String, secret: String },
    None,
}

/// Generate a request signature compatible with BitMEX.
pub fn generate_signature(
    secret: &str,
    method: &str,
    path: &str,
    expires: &str,
    data: &str,
) -> String {
    let input_msg = &(method.to_owned() + path + expires + data);
    // Create new HMAC instance
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    // Add message to digest
    mac.update(input_msg.as_bytes());

    // Get the result
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    hex::encode(code_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature() {
        let sig = generate_signature(
            "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO",
            "POST",
            "/api/v1/order",
            "1518064238",
            "{\"symbol\":\"XBTM15\",\"price\":219.0,\"clOrdID\":\"mm_bitmex_1a/oemUeQ4CAJZgP3fjHsA\",\"orderQty\":98}"
        );
        assert_eq!(
            sig,
            "1749cd2ccae4aa49048ae09f0b95110cee706e0944e6a14ad0b3a8cb45bd336b"
        );

        let sig = generate_signature(
            "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO",
            "GET",
            "/api/v1/instrument?filter=%7B%22symbol%22%3A+%22XBTM15%22%7D",
            "1518064237",
            "",
        );
        assert_eq!(
            sig,
            "e2f422547eecb5b3cb29ade2127e21b858b235b386bfa45e1c1756eb3383919f"
        );
    }
}
