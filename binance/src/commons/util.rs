use crate::commons::errors::UtilError;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn build_request(parameters: BTreeMap<String, String>) -> String {
    let mut request = String::new();
    for (key, value) in parameters {
        request.push_str(&format!("{key}={value}&"));
    }
    request.pop();
    request
}

pub fn build_signed_request(
    parameters: BTreeMap<String, String>,
    recv_window: u64,
) -> Result<String, UtilError> {
    build_signed_request_custom(parameters, recv_window, SystemTime::now())
}

pub fn build_signed_request_custom(
    mut parameters: BTreeMap<String, String>,
    recv_window: u64,
    start: SystemTime,
) -> Result<String, UtilError> {
    if recv_window > 0 {
        parameters.insert("recvWindow".to_string(), recv_window.to_string());
    }

    let timestamp = get_timestamp(start)?;
    parameters.insert("timestamp".to_string(), timestamp.to_string());
    Ok(build_request(parameters))
}

pub fn to_i64(v: &Value) -> Result<i64, UtilError> {
    v.as_i64()
        .ok_or_else(|| UtilError::JsonParseError("Failed to parse i64".to_string()))
}

pub fn to_f64(v: &Value) -> Result<f64, UtilError> {
    let str_val = v
        .as_str()
        .ok_or_else(|| UtilError::JsonParseError("Failed to get string value".to_string()))?;
    Ok(str_val.parse()?)
}

fn get_timestamp(start: SystemTime) -> Result<u64, UtilError> {
    let since_epoch = start.duration_since(UNIX_EPOCH)?;
    Ok(since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000)
}
