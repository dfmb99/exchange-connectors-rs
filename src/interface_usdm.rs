use crate::futures::account::FuturesAccount;
use crate::futures::general::FuturesGeneral;
use crate::futures::market::FuturesMarket;
use crate::interface_usdm_data::UsdmData;
use crate::ws_usdm::WsInterface;

struct UsdmInterface {
    general: FuturesGeneral,
    account: FuturesAccount,
    market: FuturesMarket,
    ws: WsInterface,
    data: UsdmData,
}

impl UsdmInterface {}
