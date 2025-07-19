use reqwest;
use std;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitfinexError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Service Unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Request Error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Invalid header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket error: {0}")]
    WsError(#[from] Box<tungstenite::Error>),

    #[error("Channel send error: {0}")]
    SendError(String),

    #[error("Connection disconnected: {0}")]
    Disconnected(String),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Auth error: {0}")]
    AuthError(String),
}

impl From<tungstenite::Error> for WebSocketError {
    fn from(err: tungstenite::Error) -> Self {
        WebSocketError::WsError(Box::new(err))
    }
}

pub type Result<T> = std::result::Result<T, BitfinexError>;
