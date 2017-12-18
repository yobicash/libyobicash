use num_traits::{ToPrimitive, FromPrimitive};
use num_traits::identities::{Zero, One};
use num_bigint::BigUint;
use serialize::hex::{FromHex, ToHex};
use serde::ser;
use serde::de;
use std::str::FromStr;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Rem, RemAssign};
use std::fmt;
use errors::*;
use utils::random::YRandom;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct YBigUint(pub BigUint);

impl YBigUint {
    pub fn random() -> YBigUint {
        let mut b = [0u8; 32];
        YRandom::bytes_mut(&mut b);
        YBigUint::from_bytes(&b[..])
    }

    pub fn parse(s: &str) -> YResult<YBigUint> {
        Ok(YBigUint(BigUint::from_str(s)?))
    }

    pub fn to_string(&self) -> String {
        self.0.to_str_radix(10)
    }

    pub fn to_u64(&self) -> YResult<u64> {
        match self.0.to_u64() {
            Some(n) => { Ok(n) },
            None => {
                Err(YErrorKind::BigUintOutOfBound.into())
            },
        }
    }

    pub fn from_u64(n: u64) -> YResult<YBigUint> {
        match BigUint::from_u64(n) {
            Some(bu) => { Ok(YBigUint(bu)) },
            None => {
                Err(YErrorKind::BigUintOutOfBound.into())
            },
        }
    }

    pub fn zero() -> YBigUint {
        YBigUint(BigUint::zero())
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn one() -> YBigUint {
        YBigUint(BigUint::one())
    }

    pub fn to_big_endian(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }

    pub fn from_big_endian(b: &[u8]) -> YBigUint {
        YBigUint(BigUint::from_bytes_be(b))
    }

    pub fn to_little_endian(&self) -> Vec<u8> {
        self.0.to_bytes_le()
    }

    pub fn from_little_endian(b: &[u8]) -> YBigUint {
        YBigUint(BigUint::from_bytes_le(b))
    }

    pub fn from_bytes(b: &[u8]) -> YBigUint {
        YBigUint::from_big_endian(b)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_big_endian()
    }

    pub fn from_hex(s: &str) -> YResult<YBigUint> {
        let buf = s.from_hex()?;
        Ok(YBigUint::from_bytes(buf.as_slice()))
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }
}

impl fmt::Debug for YBigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for YBigUint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

impl Add for YBigUint {
    type Output = YBigUint;

    fn add(self, other: YBigUint) -> YBigUint {
        YBigUint(self.0.add(other.0))
    }
}

impl AddAssign for YBigUint {
    fn add_assign(&mut self, other: YBigUint) {
        *self = self.clone().add(other);
    }
}

impl Sub for YBigUint {
    type Output = YBigUint;

    fn sub(self, other: YBigUint) -> YBigUint {
        YBigUint(self.0.sub(other.0))
    }
}

impl SubAssign for YBigUint {
    fn sub_assign(&mut self, other: YBigUint) {
        *self = self.clone().sub(other);
    }
}

impl Mul for YBigUint {
    type Output = YBigUint;

    fn mul(self, other: YBigUint) -> YBigUint {
        YBigUint(self.0.mul(other.0))
    }
}

impl MulAssign for YBigUint {
    fn mul_assign(&mut self, other: YBigUint) {
        *self = self.clone().mul(other);
    }
}

impl Div for YBigUint {
    type Output = YBigUint;

    fn div(self, other: YBigUint) -> YBigUint {
        YBigUint(self.0.div(other.0))
    }
}

impl DivAssign for YBigUint {
    fn div_assign(&mut self, other: YBigUint) {
        *self = self.clone().div(other);
    }
}

impl Rem for YBigUint {
    type Output = YBigUint;

    fn rem(self, other: YBigUint) -> YBigUint {
        YBigUint(self.0.rem(other.0))
    }
}

impl RemAssign for YBigUint {
    fn rem_assign(&mut self, other: YBigUint) {
        *self = self.clone().rem(other);
    }
}

impl ser::Serialize for YBigUint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct YBigUintVisitor;

impl <'a> de::Visitor<'a> for YBigUintVisitor {
    type Value = YBigUint;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A string representing a numerical value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        YBigUint::parse(value)
            .map_err(de::Error::custom)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where E: de::Error
    {
        YBigUint::parse(value.as_str())
            .map_err(de::Error::custom)
    }
}


impl<'a> de::Deserialize<'a> for YBigUint {
    fn deserialize<D>(deserializer: D) -> Result<YBigUint, D::Error>
        where D: de::Deserializer<'a>
    {
        deserializer.deserialize_bytes(YBigUintVisitor)
    }
}
