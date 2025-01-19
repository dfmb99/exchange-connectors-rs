use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}

#[derive(Error, Debug)]
pub enum BinanceError {
    #[error("Binance API error: {response:?}")]
    BinanceError { response: BinanceContentError },

    #[error("{name} at {index} is missing")]
    KlineValueMissing { index: usize, name: &'static str },

    #[error("Request error: {0}")]
    ReqError(#[from] reqwest::Error),

    #[error("Invalid header: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Float parsing error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("URL parsing error: {0}")]
    UrlParser(#[from] url::ParseError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    Websocket(#[from] tungstenite::Error),

    #[error("Timestamp error: {0}")]
    Timestamp(#[from] std::time::SystemTimeError),

    #[error("Failed to get timestamp")]
    TimestampError,

    #[error("Unkown status code {0}")]
    UnkownStatusCode(StatusCode),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Symbol not found")]
    SymbolNotFound,

    #[error("Util error: {0}")]
    Util(#[from] UtilError),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] WebSocketError),
}

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(String),

    #[error("WebSocket message error: {0}")]
    MessageError(String),

    #[error("Disconnected")]
    Disconnected,

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("WebSocket error: {0}")]
    WsError(#[from] tungstenite::Error),

    #[error("Loop closed")]
    LoopClosed,
}

#[derive(Error, Debug)]
pub enum UtilError {
    #[error("Failed to parse JSON value to number: {0}")]
    JsonParseError(String),

    #[error("Failed to parse string to float: {0}")]
    FloatParseError(#[from] std::num::ParseFloatError),

    #[error("Failed to get timestamp: {0}")]
    TimestampError(#[from] std::time::SystemTimeError),
}

pub type Result<T> = std::result::Result<T, BinanceError>;
