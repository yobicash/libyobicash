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
pub const VERSION: &str = "0.2.1";

/// The starting date time.
pub const MINDATETIME: &str = "2018-01-18T00:00:00Z";

/// The maximum accepted error noise of time measures. The internet is messy.
pub const MAXTIMENOISE: i64 = 3_600;

/// The maximum amount of coins.
pub const MAXAMOUNT: f32 = 21_000_000.000_000_000_000_000;
    
/// The default maximum number of concurrently connected peers.
pub const DEFAULT_MAX_CONNECTIONS: u32 = 8;

/// The default maximum size per message.
pub const DEFAULT_MAX_SIZE: u32 = 1<<20;

/// The default recursion limit.
pub const DEFAULT_MAX_RECURSION: u32 = 6;

/// The default base fee per byte.
pub const DEFAULT_BASE_FEE: f32 = 0.0;

/// The default base difficulty per connection.
pub const DEFAULT_BASE_DIFFICULTY: u32 = 0;

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
