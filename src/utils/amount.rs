// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `amount` module provides the amount type and methods.

use rmp_serde as messagepack;
use hex;
use rug::Rational;
use rug::ops::Pow;

use constants::GENESIS_AMOUNT;
use error::ErrorKind;
use result::Result;
use traits::{BinarySerialize, HexSerialize};

use std::fmt;
use std::cmp::Eq;
use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// An `Amount` is a bounded integer used for amounts and balances.
#[derive(Clone, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Amount(Rational);

impl Amount {
    /// Creates a new `Amount`.
    pub fn new() -> Amount {
        Amount(Rational::new())
    }

    /// Returns the zero `Amount`.
    pub fn zero() -> Amount {
        Amount::new()
    }

    /// Returns the unit `Amount`.
    pub fn one() -> Amount {
        Amount(1u32.into())
    }

    /// Power operation on an `Amount`.
    pub fn pow(&self, exp: i32) -> Amount {
        Amount(self.0.clone().pow(exp))
    }

    /// Returns the genesis `Amount`.
    pub fn genesis_value() -> Amount {
        Amount::from(GENESIS_AMOUNT)
    }

    /// Converts the `Amount` to string.
    pub fn to_string(&self) -> String {
        self.0.to_string_radix(10)
    }

    /// Creates an `Amount` from a string.
    pub fn from_string(s: &str) -> Result<Amount> {
        Ok(Amount(Rational::from_str_radix(s, 10)?))
    }
}

impl BinarySerialize for Amount {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::encode::to_vec(self)?;

        Ok(buf)
    }

    fn from_bytes(b: &[u8]) -> Result<Amount> {
        let version = messagepack::decode::from_slice(b)?;

        Ok(version)
    }
}

impl HexSerialize for Amount {
    fn from_hex(s: &str) -> Result<Amount> {
        if s.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }
    
        Amount::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Amount {
    fn eq(&self, other: &Amount) -> bool {
        self.0.eq(&other.0) 
    }
}

impl Eq for Amount {}

impl From<u32> for Amount {
    fn from(n: u32) -> Amount {
        Amount(Rational::from_f32(n as f32).unwrap())
    }
}

impl From<u64> for Amount {
    fn from(n: u64) -> Amount {
        Amount(Rational::from_f64(n as f64).unwrap())
    }
}

impl From<f32> for Amount {
    fn from(n: f32) -> Amount {
        Amount(Rational::from_f32(n).unwrap())
    }
}

impl From<f64> for Amount {
    fn from(n: f64) -> Amount {
        Amount(Rational::from_f64(n).unwrap())
    }
}

impl From<(u32, u32)> for Amount {
    fn from(parts: (u32, u32)) -> Amount {
        Amount(Rational::from(parts))
    }
}

impl From<(u64, u64)> for Amount {
    fn from(parts: (u64, u64)) -> Amount {
        Amount(Rational::from(parts))
    }
}

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Amount {
        let mut output = self.0.clone();
        output += rhs.0.clone();

        Amount(output)
    }
}

impl<'a> Add<&'a Amount> for Amount {
    type Output = Amount;

    fn add(self, rhs: &Amount) -> Amount {
        let mut output = self.0.clone();
        output += rhs.0.clone();

        Amount(output)
    }
}

impl<'a, 'b> Add<&'b Amount> for &'a Amount {
    type Output = Amount;

    fn add(self, rhs: &Amount) -> Amount {
        let mut output = self.0.clone();
        output += rhs.0.clone();

        Amount(output)
    }
}

impl AddAssign<Amount> for Amount {
    fn add_assign(&mut self, rhs: Amount) {
        self.0 += rhs.0.clone()
    }
}

impl<'a> AddAssign<&'a Amount> for Amount {
    fn add_assign(&mut self, rhs: &Amount) {
        self.0 += rhs.0.clone()
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Amount) -> Amount {
        let mut output = self.0.clone();
        output -= rhs.0.clone();

        Amount(output)
    }
}

impl<'a> Sub<&'a Amount> for Amount {
    type Output = Amount;

    fn sub(self, rhs: &Amount) -> Amount {
        let mut output = self.0.clone();
        output -= rhs.0.clone();

        Amount(output)
    }
}

impl<'a, 'b> Sub<&'b Amount> for &'a Amount {
    type Output = Amount;

    fn sub(self, rhs: &Amount) -> Amount {
        let mut output = self.0.clone();
        output -= rhs.0.clone();

        Amount(output)
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Amount) {
        self.0 -= rhs.0.clone()
    }
}

impl<'a> SubAssign<&'a Amount> for Amount {
    fn sub_assign(&mut self, rhs: &Amount) {
        self.0 -= rhs.0.clone()
    }
}

impl Mul for Amount {
    type Output = Amount;

    fn mul(self, rhs: Amount) -> Amount {
        let mut output = self.0.clone();
        output *= rhs.0.clone();

        Amount(output)
    }
}

impl<'a> Mul<&'a Amount> for Amount {
    type Output = Amount;

    fn mul(self, rhs: &Amount) -> Amount {
        let mut output = self.0.clone();
        output *= rhs.0.clone();

        Amount(output)
    }
}

impl<'a, 'b> Mul<&'b Amount> for &'a Amount {
    type Output = Amount;

    fn mul(self, rhs: &Amount) -> Amount {
        let mut output = self.0.clone();
        output *= rhs.0.clone();

        Amount(output)
    }
}

impl MulAssign<Amount> for Amount {
    fn mul_assign(&mut self, rhs: Amount) {
        self.0 *= rhs.0.clone()
    }
}

impl<'a> MulAssign<&'a Amount> for Amount {
    fn mul_assign(&mut self, rhs: &Amount) {
        self.0 *= rhs.0.clone()
    }
}

impl Div for Amount {
    type Output = Amount;

    fn div(self, rhs: Amount) -> Amount {
        let mut output = self.0.clone();
        output /= rhs.0.clone();

        Amount(output)
    }
}

impl<'a> Div<&'a Amount> for Amount {
    type Output = Amount;

    fn div(self, rhs: &Amount) -> Amount {
        let mut output = self.0.clone();
        output /= rhs.0.clone();

        Amount(output)
    }
}

impl<'a, 'b> Div<&'b Amount> for &'a Amount {
    type Output = Amount;

    fn div(self, rhs: &Amount) -> Amount {
        let mut output = self.0.clone();
        output /= rhs.0.clone();

        Amount(output)
    }
}

impl DivAssign<Amount> for Amount {
    fn div_assign(&mut self, rhs: Amount) {
        self.0 /= rhs.0.clone()
    }
}

impl<'a> DivAssign<&'a Amount> for Amount {
    fn div_assign(&mut self, rhs: &Amount) {
        self.0 /= rhs.0.clone()
    }
}
