extern crate bitfinex;

use bitfinex::websocket::derivs_ws::DerivsWs;
use bitfinex::commons::pairs::TESTBTCPERP;
use std::thread;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();
    let api_key = "5QytTTlYGhLHzo1nT17O2baW3A12DBaPzydzu3aWvEy".to_string();
    let api_secret = "LYrjDqa7TOvxDjlViaku3Ux6Ci7j7qfrAV1lp8vo9DZ".to_string();
    DerivsWs::new(TESTBTCPERP.to_string(), api_key, api_secret);
    loop {
        thread::yield_now();
    }
}
