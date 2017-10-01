use bigint::uint::{U512, FromDecStrErr};
use std::ops::Not;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Rem, RemAssign};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct YBigUint(pub U512);

impl YBigUint {
  pub fn parse(s: &str) -> Result<YBigUint, FromDecStrErr> {
    let bu = U512::from_dec_str(s)?;
    Ok(YBigUint(bu))
  }

  // NB: panics
  pub fn as_u64(&self) -> u64 {
    self.0.as_u64()
  }

  pub fn from_u64(n: u64) -> Result<YBigUint, FromDecStrErr> {
    YBigUint::parse(format!("{}", n).as_str())
  }

  pub fn zero() -> YBigUint {
    YBigUint(U512::zero())
  }

  pub fn is_zero(&self) -> bool {
    self.0.is_zero()
  }

  pub fn one() -> YBigUint {
    YBigUint(U512::one())
  }

  pub fn max_value() -> YBigUint {
    YBigUint(U512::max_value())
  }

  pub fn pow(self, exp: YBigUint) -> YBigUint {
    YBigUint(self.0.pow(exp.0))
  }

  pub fn to_big_endian(&self) -> Vec<u8> {
    let mut be: Vec<u8> = Vec::new();
    self.0.to_big_endian(be.as_mut_slice());
    be
  }

  pub fn from_big_endian(b: &[u8]) -> YBigUint {
    YBigUint(U512::from_big_endian(b))
  }

  pub fn to_little_endian(&self) -> Vec<u8> {
    let mut be: Vec<u8> = Vec::new();
    self.0.to_little_endian(be.as_mut_slice());
    be
  }

  pub fn from_little_endian(b: &[u8]) -> YBigUint {
    YBigUint(U512::from_little_endian(b))
  }
}

impl Not for YBigUint {
  type Output = YBigUint;

  fn not(self) -> YBigUint {
    YBigUint(self.0.not())
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
    *self = self.add(other);
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
    *self = self.sub(other);
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
    *self = self.mul(other);
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
    *self = self.div(other);
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
    *self = self.rem(other);
  }
}
