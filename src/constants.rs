// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `constants` module provides the constants used throughout the library.

/// The name of the cryptocurrency.
pub const NAME: &str = "Yobicash";

/// The code of the cryptocurrency.
pub const CODE: &str = "YBC";

/// The version of the cryptocurrency library.
pub const VERSION: &str = "0.3.1";

/// The starting date time.
pub const MIN_DATETIME: &str = "2018-01-18T00:00:00Z";

/// The maximum accepted error noise of time measures. The internet is messy.
pub const MAX_TIMENOISE: i64 = 3_600;

/// The genesis output amount.
pub const GENESIS_AMOUNT: u32 = 21;

/// The minimum data duration in hours.
pub const MIN_DATA_DURATION: u32 = 1;

/// The mainnet witness.
pub const MAINWITNESS: &str = "1e9f288451e2beb8b5c7ae598c4ca0cfe88722a8d0c44b5ff1d42c6fde17b7f6";

/// The testnet witness.
pub const TESTWITNESS: &str = "893bca8b0c490032e53c0349ca922418b2349af16e573230680a36e38ea08a47";

/// The mainnet port.
pub const MAINPORT: u16 = 2112;

/// The testnet port.
pub const TESTPORT: u16 = 3113;

/// The regtest port.
pub const REGTESTPORT: u16 = 4114;

/// Confirmation time, which is the target number of seconds between
/// two blocks.
pub const CONFIRMATION_TIME: u32 = 10;

/// Retarget time, which is the number of blocks before a retarget.
pub const RETARGET_TIME: u32 = 10;

/// Mining interest rate.
pub const INTEREST_RATE: f32 = 1.4;

/// Minimum difficulty.
pub const MIN_DIFFICULTY: u32 = 3;

/// Maximum difficulty.
pub const MAX_DIFFICULTY: u32 = 63;

/// Difficulty used to mine the genesis block header.
pub const GENESIS_DIFFICULTY: u32 = 3;

/// Memory used to mine the genesis block header.
pub const GENESIS_MEMORY: u32 = 64;

/// Heights to pass before a coinbase is spendable.
pub const MATURITY_TIME: u32 = 60_480; // 1 week
