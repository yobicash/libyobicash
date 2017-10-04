use curve25519_dalek::edwards::{ExtendedPoint, CompressedEdwardsY};
use curve25519_dalek::edwards::{Identity, IsIdentity};
use curve25519_dalek::edwards::ValidityCheck;
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use subtle::Equal;
use rand::random;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::Neg;
use std::ops::{Mul, MulAssign};
use errors::*;
use crypto::elliptic::scalar::YScalar;

#[derive(Copy, Clone, Debug)]
pub struct YPoint(pub ExtendedPoint);

impl Default for YPoint {
  fn default() -> YPoint {
    YPoint(ED25519_BASEPOINT_POINT)
  }
}

impl YPoint {
  pub fn random() -> YResult<YPoint> {
    let mut b = [0u8; 32];
    for i in 0..32 {
      b[i] = random::<u8>();
    }
    YPoint::from_bytes(&b)
  }

  pub fn from_bytes(b: &[u8]) -> YResult<YPoint> {
    if b.len() != 32 {
      return Err(YErrorKind::InvalidLength.into());
    }
    let mut _b = [0u8; 32];
    for i in 0..32 {
      _b[i] = b[i];
    }
    let compressed = CompressedEdwardsY(_b);
    if let Some(point) = compressed.decompress() {
      Ok(YPoint(point))
    } else {
      let reason = String::from("Failed decompressing the point");
      return Err(YErrorKind::InvalidPoint(reason).into());
    }
  }

  pub fn to_bytes(&self) -> [u8; 32] {
    self.0.compress_edwards().to_bytes()
  }
}

impl PartialEq for YPoint {
  fn eq(&self, other: &YPoint) -> bool {
    self.0.ct_eq(&other.0) == 0u8
  }
}

impl Eq for YPoint {}

impl<'a, 'b> Add<&'b YPoint> for &'a YPoint {
  type Output = YPoint;

  fn add(self, other: &'b YPoint) -> YPoint {
    YPoint(self.0.add(&other.0))
  }
}

impl<'b> AddAssign<&'b YPoint> for YPoint {
  fn add_assign(&mut self, other: &'b YPoint) {
    self.0.add_assign(&other.0)
  }
}

impl<'a> Neg for &'a YPoint {
  type Output = YPoint;

  fn neg(self) -> YPoint {
    YPoint(self.0.neg())
  }
}

impl<'a, 'b> Sub<&'b YPoint> for &'a YPoint {
  type Output = YPoint;

  fn sub(self, other: &'b YPoint) -> YPoint {
    YPoint(self.0.sub(&other.0))
  }
}

impl<'b> SubAssign<&'b YPoint> for YPoint {
  fn sub_assign(&mut self, other: &'b YPoint) {
    self.0.sub_assign(&other.0)
  }
}

impl<'a, 'b> Mul<&'b YScalar> for &'a YPoint {
  type Output = YPoint;

  fn mul(self, other: &'b YScalar) -> YPoint {
    YPoint(self.0.mul(&other.0))
  }
}

impl<'b> MulAssign<&'b YScalar> for YPoint {
  fn mul_assign(&mut self, other: &'b YScalar) {
    self.0.mul_assign(&other.0)
  }
}

impl Identity for YPoint {
  fn identity() -> YPoint {
    YPoint(ExtendedPoint::identity())
  }
}

impl IsIdentity for YPoint {
  fn is_identity(&self) -> bool {
    self.0.is_identity()    
  }
}

// NB: not CT
impl ValidityCheck for YPoint {
  fn is_valid(&self) -> bool {
    self.0.is_valid()
  }
}
