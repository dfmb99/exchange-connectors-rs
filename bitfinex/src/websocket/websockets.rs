use crate::commons::auth;
use crate::commons::errors::WebSocketError;
use crate::websocket::events::*;
use serde_json::{from_str, json};
use std::net::TcpStream;
use url::Url;

use tungstenite::connect;
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Message;

use std::sync::mpsc::{self, channel, SendError};

impl From<SendError<WsMessage>> for WebSocketError {
    fn from(err: SendError<WsMessage>) -> Self {
        WebSocketError::SendError(err.to_string())
    }
}

static INFO: &str = "info";
static SUBSCRIBED: &str = "subscribed";
static AUTH: &str = "auth";
static WEBSOCKET_URL: &str = "wss://api.bitfinex.com/ws/2";
static DEAD_MAN_SWITCH_FLAG: u8 = 4;

pub trait EventHandler {
    fn on_connect(&mut self, event: NotificationEvent);
    fn on_auth(&mut self, event: NotificationEvent);
    fn on_subscribed(&mut self, event: NotificationEvent);
    fn on_data_event(&mut self, event: DataEvent);
    fn on_error(&mut self, message: WebSocketError);
}

pub enum EventType {
    Funding,
    Trading,
}

#[derive(Debug)]
enum WsMessage {
    Close,
    Text(String),
}

pub struct WebSockets {
    socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    sender: Sender,
    rx: mpsc::Receiver<WsMessage>,
    event_handler: Option<Box<dyn EventHandler>>,
}

impl Default for WebSockets {
    fn default() -> WebSockets {
        let (tx, rx) = channel::<WsMessage>();
        let sender = Sender { tx };

        WebSockets {
            socket: None,
            sender,
            rx,
            event_handler: None,
        }
    }
}

impl WebSockets {
    pub fn connect(&mut self) -> Result<(), WebSocketError> {
        let url = Url::parse(WEBSOCKET_URL)?;
        self.socket = Some(connect(url.as_str())?);
        Ok(())
    }

    pub fn add_event_handler<H>(&mut self, handler: H)
    where
        H: EventHandler + 'static,
    {
        self.event_handler = Some(Box::new(handler));
    }

    pub fn auth<S>(
        &mut self,
        api_key: S,
        api_secret: S,
        dms: bool,
        filters: &[&str],
    ) -> Result<(), WebSocketError>
    where
        S: AsRef<str>,
    {
        let nonce = auth::generate_nonce().map_err(|e| WebSocketError::AuthError(e.to_string()))?;
        let auth_payload = format!("AUTH{nonce}");
        let signature = auth::sign_payload(api_secret.as_ref().as_bytes(), auth_payload.as_bytes())
            .map_err(|e| WebSocketError::AuthError(e.to_string()))?;

        let msg = json!({
            "event": "auth",
            "apiKey": api_key.as_ref(),
            "authSig": signature,
            "authNonce": nonce,
            "authPayload": auth_payload,
            "dms": if dms {Some(DEAD_MAN_SWITCH_FLAG)} else {None},
            "filters": filters,
        });

        self.sender.send(&msg.to_string())?;
        Ok(())
    }

    pub fn subscribe_ticker<S>(&mut self, symbol: S) -> Result<(), WebSocketError>
    where
        S: Into<String>,
    {
        let msg = json!({
            "event": "subscribe",
            "channel": "ticker",
            "symbol": symbol.into()
        });
        self.sender.send(&msg.to_string())
    }

    pub fn subscribe_trades<S>(&mut self, symbol: S) -> Result<(), WebSocketError>
    where
        S: Into<String>,
    {
        let msg = json!({
            "event": "subscribe",
            "channel": "trades",
            "symbol": symbol.into()
        });
        self.sender.send(&msg.to_string())
    }

    pub fn subscribe_candles<S>(&mut self, symbol: S, timeframe: S) -> Result<(), WebSocketError>
    where
        S: Into<String>,
    {
        let key = format!("trade:{}:{}", timeframe.into(), symbol.into());
        let msg = json!({
            "event": "subscribe",
            "channel": "candles",
            "key": key
        });
        self.sender.send(&msg.to_string())
    }

    pub fn subscribe_books<S, P, F>(
        &mut self,
        symbol: S,
        prec: P,
        freq: F,
        len: u32,
    ) -> Result<(), WebSocketError>
    where
        S: Into<String>,
        P: Into<String>,
        F: Into<String>,
    {
        let msg = json!({
            "event": "subscribe",
            "channel": "book",
            "symbol": symbol.into(),
            "prec": prec.into(),
            "freq": freq.into(),
            "len": len
        });
        self.sender.send(&msg.to_string())
    }

    pub fn event_loop(&mut self) -> Result<(), WebSocketError> {
        loop {
            if let Some(ref mut socket) = self.socket {
                // Handle pending messages
                while let Ok(msg) = self.rx.try_recv() {
                    match msg {
                        WsMessage::Text(text) => socket.0.send(Message::Text(text.into()))?,
                        WsMessage::Close => {
                            return socket.0.close(None).map_err(WebSocketError::from)
                        }
                    }
                }

                match self.rx.try_recv() {
                    Err(mpsc::TryRecvError::Disconnected) => {
                        return Err(WebSocketError::Disconnected(
                            "Channel disconnected".to_string(),
                        ))
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                    Ok(_) => {}
                }

                let message = socket.0.read()?;

                if let Some(ref mut handler) = self.event_handler {
                    match message {
                        Message::Text(text) => {
                            if text.contains(INFO) {
                                handler.on_connect(from_str(&text)?);
                            } else if text.contains(SUBSCRIBED) {
                                handler.on_subscribed(from_str(&text)?);
                            } else if text.contains(AUTH) {
                                handler.on_auth(from_str(&text)?);
                            } else {
                                let event: DataEvent = from_str(&text)?;
                                if !matches!(event, DataEvent::HeartbeatEvent(_, _)) {
                                    handler.on_data_event(event);
                                }
                            }
                        }
                        Message::Close(e) => {
                            return Err(WebSocketError::Disconnected(format!(
                                "Connection closed: {e:?}"
                            )))
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<WsMessage>,
}

impl Sender {
    pub fn send(&self, raw: &str) -> Result<(), WebSocketError> {
        self.tx.send(WsMessage::Text(raw.to_string()))?;
        Ok(())
    }

    pub fn shutdown(&self) -> Result<(), WebSocketError> {
        self.tx.send(WsMessage::Close)?;
        Ok(())
    }
}
