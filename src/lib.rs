#![recursion_limit = "1024"]

extern crate typenum;
extern crate generic_array;
extern crate sha2;
extern crate hmac;
extern crate hkdf;
extern crate curve25519_dalek;
extern crate rand;
extern crate num_traits;
extern crate num_bigint;
extern crate subtle;
extern crate semver;
extern crate chrono;
extern crate byteorder;
#[macro_use]
extern crate error_chain;
extern crate rustc_serialize as serialize;
extern crate libc;

pub mod errors;
pub mod utils;
pub mod crypto;
pub mod amount;
pub mod data;
pub mod output;
pub mod input;
pub mod utxo;
pub mod transaction;
pub mod coinbase;

pub const VERSION: &str = "0.1.0";

pub const MAX_AMOUNT: &str = "2048000000000000000000000";
