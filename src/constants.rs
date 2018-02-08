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

/// The version of the cryptocurrency.
pub const VERSION: &str = "0.1.1";

/// The starting date time.
pub const MINDATETIME: &str = "2018-01-18T00:00:00Z";

/// The maximum accepted error noise of time measures. The internet is messy.
pub const MAXTIMENOISE: i64 = 3_600;

/// The maximum amount of coins.
pub const MAXAMOUNT: f32 = 21_000_000.000_000_000_000_000;
