// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `timestamp` module provides the timestamp types and methods.

use chrono::{DateTime, TimeZone, Utc};
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use hex;

use constants::{MINDATETIME, MAXTIMENOISE};
use error::ErrorKind;
use result::Result;
use traits::{BinarySerialize, HexSerialize, Validate};

use std::fmt;

/// A timestamp is an integer representing the number of seconds elapsed since
/// the `Epoch` time (1970-01-01:00:00:00.0000...).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Creates a UTC unix `Timestamp` from a given date.
    pub fn from_date(year: i32,
                     month: u32,
                     day: u32,
                     hours: u32,
                     mins: u32,
                     secs: u32) -> Result<Timestamp>
    {
        if day > 31 {
            return Err(ErrorKind::InvalidTime.into());
        }
        
        if hours > 24 {
            return Err(ErrorKind::InvalidTime.into());
        }
        
        if mins > 60 {
            return Err(ErrorKind::InvalidTime.into());
        }
        
        if secs > 60 {
            return Err(ErrorKind::InvalidTime.into());
        }
        
        let dt = Utc.ymd(year, month, day)
            .and_hms(mins, hours, secs);

        let _timestamp = dt.timestamp();

        Ok(Timestamp(_timestamp))
    }

    /// Returns the minimum `Timestamp`.
    pub fn min_value() -> Timestamp {
        Timestamp::parse(MINDATETIME).unwrap()
    }

    /// Creates a `Timestamp` from a UTC date time string in rfc3339 format.
    /// e.g.: `2018-01-18T00:00:00Z`
    pub fn parse(s: &str) -> Result<Timestamp> {
        let dt = s.parse::<DateTime<Utc>>()?;

        let _timestamp = dt.timestamp();

        Ok(Timestamp(_timestamp))
    }

    /// Creates a `Timestamp` from a string.
    pub fn from_string(s: &str) -> Result<Timestamp> {
        Ok(Timestamp(i64::from_str_radix(s, 10)?))
    }

    /// Converts the `Timestamp` to string.
    pub fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }

    /// Returns the current time timestamp.
    pub fn now() -> Timestamp {
        Timestamp(Utc::now().timestamp())
    }

    /// Returns the `Timestamp` with the maximum time noise.
    pub fn with_noise(&self) -> Timestamp {
        Timestamp(self.0 + MAXTIMENOISE)
    }

    /// Returns the time difference between this `Timestamp` and an other.
    pub fn diff(&self, other: Timestamp) -> i64 {
        self.0 - other.0
    }
}

impl BinarySerialize for Timestamp {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.write_i64::<BigEndian>(self.0)?;

        Ok(buf)
    }

    fn from_bytes(b: &[u8]) -> Result<Timestamp> {
        let len = b.len();
        if len != 8 {
            return Err(ErrorKind::InvalidLength.into());
        }

        let _timestamp = BigEndian::read_i64(b);
        Ok(Timestamp(_timestamp))
    }
}

impl HexSerialize for Timestamp {
    fn from_hex(s: &str) -> Result<Timestamp> {
        if s.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }
    
        Timestamp::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}

impl Validate for Timestamp {
    fn validate(&self) -> Result<()> {
        if *self < Timestamp::min_value().with_noise() {
            return Err(ErrorKind::InvalidTimestamp.into());
        }
     
        if *self > Timestamp::now().with_noise() {
            return Err(ErrorKind::InvalidTimestamp.into());
        }

        Ok(())
    }
}

impl Default for Timestamp {
    fn default() -> Timestamp {
        Timestamp::now()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
