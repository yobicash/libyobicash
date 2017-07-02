#![recursion_limit = "1024"] // for error_chain
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate sodiumoxide;
extern crate rand;
extern crate byteorder;
extern crate num_bigint;
extern crate num_traits;
extern crate semver;
extern crate chrono;

pub mod errors;
pub mod length;
pub mod size;
pub mod crypto;
pub mod wallet;
pub mod address;
pub mod signers;
pub mod amount;
pub mod input;
pub mod output;
pub mod tx;
pub mod mining;
pub mod block;

pub const NAME: &str = "Yobicash";

pub const VERSION: &str = "0.1.0";

pub const CONFIRMATION_TIME: u32 = 20; // 20 seconds
