use crate::rest::api::{API, Futures};
use crate::rest::client::Client;
use crate::rest::model::{Success, UserDataStream};
use crate::commons::errors::*;

#[derive(Clone)]
pub struct FuturesUserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl FuturesUserStream {
    // User Stream
    pub fn start(&self) -> Result<UserDataStream> {
        self.client.post(API::Futures(Futures::UserDataStream))
    }

    pub fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client
            .put(API::Futures(Futures::UserDataStream), listen_key)
    }

    pub fn close(&self, listen_key: &str) -> Result<Success> {
        self.client
            .delete(API::Futures(Futures::UserDataStream), listen_key)
    }
}
