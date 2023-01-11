/*#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]
#![allow(clippy::needless_doctest_main)]
*/

#![allow(clippy::needless_doctest_main)]

#[macro_use]
extern crate error_chain;

pub mod commons;
pub mod rest;
pub mod websocket;
pub mod interfaces;
