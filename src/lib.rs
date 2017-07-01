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

use self::errors::*;

pub const NAME: &str = "Yobicash";

pub const VERSION: &str = "0.1.0";

pub const MAX_LEN: usize = std::u32::MAX as usize; // for word32 machines

pub fn check_length<T>(xs: &[T]) -> YResult<()> {
    let len = xs.len();
    if len > MAX_LEN {
        return Err(YErrorKind::InvalidLength.into());
    }
    Ok(())
}

pub const MAX_SIZE: usize = std::u32::MAX as usize; // for word32 machines

pub fn check_size(bin: &[u8]) -> YResult<()> {
    let len = bin.len();
    if len > MAX_SIZE {
        return Err(YErrorKind::InvalidSize.into());
    }
    Ok(())
}

pub const CONFIRMATION_TIME: u32 = 20; // 20 seconds
