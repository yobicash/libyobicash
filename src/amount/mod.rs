use serialize::hex::{FromHex, ToHex};
use utils::biguint::YBigUint;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Rem, RemAssign};
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Error as FmtError;
use errors::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Serialize, Deserialize)]
pub struct YAmount(pub YBigUint);

impl YAmount {
    pub fn new(amount: &YBigUint) -> YAmount {
        YAmount(amount.clone())
    }

    pub fn parse(s: &str) -> YResult<YAmount> {
        let n = YBigUint::parse(s)?;
        Ok(YAmount(n))
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn to_u64(&self) -> YResult<u64> {
        self.0.to_u64()
    }

    pub fn from_u64(n: u64) -> YResult<YAmount> {
        Ok(YAmount(YBigUint::from_u64(n)?))
    }

    pub fn zero() -> YAmount {
        YAmount(YBigUint::zero())
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn one() -> YAmount {
        YAmount(YBigUint::one())
    }

    pub fn to_big_endian(&self) -> Vec<u8> {
        self.0.to_big_endian()
    }

    pub fn from_big_endian(b: &[u8]) -> YAmount {
        YAmount(YBigUint::from_big_endian(b))
    }

    pub fn to_little_endian(&self) -> Vec<u8> {
        self.0.to_little_endian()
    }

    pub fn from_little_endian(b: &[u8]) -> YAmount {
        YAmount(YBigUint::from_little_endian(b))
    }

    pub fn from_bytes(b: &[u8]) -> YAmount {
        YAmount::from_big_endian(b)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_big_endian()
    }

    pub fn from_hex(s: &str) -> YResult<YAmount> {
        let buf = s.from_hex()?;
        Ok(YAmount::from_bytes(buf.as_slice()))
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }
}

impl Debug for YAmount {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}

impl Display for YAmount {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}

impl Add for YAmount {
    type Output = YAmount;

    fn add(self, other: YAmount) -> YAmount {
        YAmount(self.0.add(other.0))
    }
}

impl AddAssign for YAmount {
    fn add_assign(&mut self, other: YAmount) {
        *self = self.clone().add(other);
    }
}

impl Sub for YAmount {
    type Output = YAmount;

    fn sub(self, other: YAmount) -> YAmount {
        YAmount(self.0.sub(other.0))
    }
}

impl SubAssign for YAmount {
    fn sub_assign(&mut self, other: YAmount) {
        *self = self.clone().sub(other);
    }
}

impl Mul for YAmount {
    type Output = YAmount;

    fn mul(self, other: YAmount) -> YAmount {
        YAmount(self.0.mul(other.0))
    }
}

impl MulAssign for YAmount {
    fn mul_assign(&mut self, other: YAmount) {
        *self = self.clone().mul(other);
    }
}

impl Div for YAmount {
    type Output = YAmount;

    fn div(self, other: YAmount) -> YAmount {
        YAmount(self.0.div(other.0))
    }
}

impl DivAssign for YAmount {
    fn div_assign(&mut self, other: YAmount) {
        *self = self.clone().div(other);
    }
}

impl Rem for YAmount {
    type Output = YAmount;

    fn rem(self, other: YAmount) -> YAmount {
        YAmount(self.0.rem(other.0))
    }
}

impl RemAssign for YAmount {
    fn rem_assign(&mut self, other: YAmount) {
        *self = self.clone().rem(other);
    }
}
