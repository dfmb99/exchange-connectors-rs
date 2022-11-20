use std::sync::{Arc, RwLock};
use crate::model::KlineSummaries;

type KlineData = Arc<RwLock<KlineSummaries>>;

pub struct UsdmData {
    last_day_klines: KlineData
}

impl UsdmData {
    pub fn new() -> UsdmData {
        UsdmData {
            last_day_klines: Arc::new(RwLock::new(KlineSummaries::AllKlineSummaries(Vec::default())))
        }
    }

    pub fn get_last_day_klines(&self) -> KlineSummaries {
        self.last_day_klines.read().unwrap().clone()
    }

    pub fn set_last_day_klines(&mut self, klines: KlineSummaries) {
        *self.last_day_klines.write().unwrap() = klines;
    }

}
