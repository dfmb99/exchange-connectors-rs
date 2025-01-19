use serde::Deserialize;
use thiserror::Error;
use std::num::ParseFloatError;
use std::io;
use url::ParseError;
use reqwest::header::InvalidHeaderValue;
use reqwest::StatusCode;

#[derive(Debug, Deserialize)]
pub struct OkxContentError {
    pub code: String,
    pub msg: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("OKX API error: {0:?}")]
    OkxError(OkxContentError),
    
    #[error("Request error: {0}")]
    ReqError(#[from] reqwest::Error),
    
    #[error("Invalid header value: {0}")]
    InvalidHeaderError(#[from] InvalidHeaderValue),
    
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Float parsing error: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    
    #[error("URL parsing error: {0}")]
    UrlParserError(#[from] ParseError),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tungstenite::Error),
    
    #[error("Timestamp error: {0}")]
    TimestampError(#[from] std::time::SystemTimeError),

    #[error("Unkown status code {0}")]
    UnkownStatusCode(StatusCode)
}

// Type alias for Result with our custom error type
pub type Result<T> = std::result::Result<T, Error>;