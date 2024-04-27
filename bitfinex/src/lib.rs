#![deny(
    unstable_features,
    unused_must_use,
    unused_imports,
    unused_import_braces
)]

#[macro_use]
extern crate error_chain;

extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate tungstenite;
extern crate url;

#[macro_use]
extern crate serde_derive;

pub mod commons;
pub mod interfaces;
pub mod rest;
pub mod websocket;
