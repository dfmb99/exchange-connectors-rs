use std::sync::{Arc, RwLock};
use crate::rest::model::KlineSummaries;

type KlineData = Arc<RwLock<KlineSummaries>>;

#[derive(Clone)]
pub struct UsdmConfig {
    pub retry_on_err: bool,
    pub retry_timeout: u64,
    pub rest_update_interval: u64,
}

impl Default for UsdmConfig {
    fn default() -> UsdmConfig {
        UsdmConfig {
            retry_on_err: true,
            retry_timeout: 300,          // milliseconds
            rest_update_interval: 60000, // milliseconds
        }
    }
}

impl UsdmConfig {
    pub fn set_retry_on_err(mut self, retry_on_err: bool) -> Self {
        self.retry_on_err = retry_on_err;
        self
    }

    pub fn set_retry_timeout(mut self, retry_timeout: u64) -> Self {
        self.retry_timeout = retry_timeout;
        self
    }

    pub fn set_rest_update_interval(mut self, rest_update_interval: u64) -> Self {
        self.rest_update_interval = rest_update_interval;
        self
    }
}

#[derive(Clone)]
pub struct UsdmData {
    last_day_klines: KlineData,
}

impl Default for UsdmData {
    fn default() -> UsdmData {
        UsdmData {
            last_day_klines: Arc::new(RwLock::new(KlineSummaries::AllKlineSummaries(
                Vec::default(),
            ))),
        }
    }
}

impl UsdmData {
    pub fn get_last_day_klines(&self) -> KlineSummaries {
        self.last_day_klines.read().unwrap().clone()
    }

    pub fn set_last_day_klines(&mut self, klines: KlineSummaries) {
        *self.last_day_klines.write().unwrap() = klines;
    }
}
