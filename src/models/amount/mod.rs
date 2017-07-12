use num_bigint::{BigUint, ParseBigIntError};
use num_traits::{Num, Zero, One};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
pub struct Amount(pub BigUint);

impl Amount {
    pub fn new(m: u32) -> Self {
        Amount(BigUint::from(m))
    }

    pub fn to_u32(&self) -> Result<u32> {
        unreachable!() // TODO
    }

    pub fn from_slice(sl: &[u8]) -> Self {
        Amount(BigUint::from_bytes_be(sl))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::LowerHex for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Binary for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub type ParseAmountError = ParseBigIntError;

impl FromStr for Amount {
    type Err = ParseAmountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigUint::from_str(s).map(|bg| Amount(bg))
    }
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, other: Amount) -> Self::Output {
        Amount(self.0.add(other.0))
    }
}

impl Sub for Amount {
    type Output = Amount;
    fn sub(self, other: Amount) -> Self::Output {
        Amount(self.0.sub(other.0))
    }
}

impl Mul for Amount {
    type Output = Amount;
    fn mul(self, other: Amount) -> Self::Output {
        Amount(self.0.mul(other.0))
    }
}

impl Div for Amount {
    type Output = Amount;
    fn div(self, other: Amount) -> Self::Output {
        Amount(self.0.div(other.0))
    }
}

impl Rem for Amount {
    type Output = Amount;
    fn rem(self, other: Amount) -> Self::Output {
        Amount(self.0.rem(other.0))
    }
}

impl Zero for Amount {
    fn zero() -> Self {
        Amount(BigUint::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()    
    } 
}

impl One for Amount {
    fn one() -> Self {
        Amount(BigUint::one())
    }
}

impl Num for Amount {
    type FromStrRadixErr = ParseAmountError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        BigUint::from_str_radix(s, radix).map(|bg| Amount(bg))
    }
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_bytes(self.to_vec().as_slice())
    }
}

struct AmountVisitor;

impl<'a> Visitor<'a> for AmountVisitor {
    type Value = Amount;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a binary serialized biguint")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Amount, E>
        where E: de::Error
    {
       Ok(Amount::from_slice(value))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Amount, E>
        where E: de::Error
    {
       Ok(Amount::from_slice(value.as_slice()))
    }
}

impl<'a> Deserialize<'a> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Amount, D::Error>
        where D: Deserializer<'a>
    {
        deserializer.deserialize_byte_buf(AmountVisitor)
    }
}

