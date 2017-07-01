use num_bigint::{BigUint, ParseBigIntError};
use num_traits::{Num, Zero, One};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YAmount(pub BigUint);

impl YAmount {
    pub fn new(m: u32) -> Self {
        YAmount(BigUint::from(m))
    }

    pub fn from_slice(sl: &[u8]) -> Self {
        YAmount(BigUint::from_bytes_be(sl))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }
}

impl fmt::Display for YAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::LowerHex for YAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for YAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Binary for YAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for YAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub type ParseYAmountError = ParseBigIntError;

impl FromStr for YAmount {
    type Err = ParseYAmountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigUint::from_str(s).map(|bg| YAmount(bg))
    }
}

impl Add for YAmount {
    type Output = YAmount;
    fn add(self, other: YAmount) -> Self::Output {
        YAmount(self.0.add(other.0))
    }
}

impl Sub for YAmount {
    type Output = YAmount;
    fn sub(self, other: YAmount) -> Self::Output {
        YAmount(self.0.sub(other.0))
    }
}

impl Mul for YAmount {
    type Output = YAmount;
    fn mul(self, other: YAmount) -> Self::Output {
        YAmount(self.0.mul(other.0))
    }
}

impl Div for YAmount {
    type Output = YAmount;
    fn div(self, other: YAmount) -> Self::Output {
        YAmount(self.0.div(other.0))
    }
}

impl Rem for YAmount {
    type Output = YAmount;
    fn rem(self, other: YAmount) -> Self::Output {
        YAmount(self.0.rem(other.0))
    }
}

impl Zero for YAmount {
    fn zero() -> Self {
        YAmount(BigUint::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()    
    } 
}

impl One for YAmount {
    fn one() -> Self {
        YAmount(BigUint::one())
    }
}

impl Num for YAmount {
    type FromStrRadixErr = ParseYAmountError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        BigUint::from_str_radix(s, radix).map(|bg| YAmount(bg))
    }
}

impl Serialize for YAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_bytes(self.to_vec().as_slice())
    }
}

struct YAmountVisitor;

impl<'a> Visitor<'a> for YAmountVisitor {
    type Value = YAmount;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a binary serialized biguint")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<YAmount, E>
        where E: de::Error
    {
       Ok(YAmount::from_slice(value))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<YAmount, E>
        where E: de::Error
    {
       Ok(YAmount::from_slice(value.as_slice()))
    }
}

impl<'a> Deserialize<'a> for YAmount {
    fn deserialize<D>(deserializer: D) -> Result<YAmount, D::Error>
        where D: Deserializer<'a>
    {
        deserializer.deserialize_byte_buf(YAmountVisitor)
    }
}

