#![allow(non_snake_case)]

use curve25519_dalek::edwards::{ExtendedPoint, CompressedEdwardsY};
use curve25519_dalek::edwards::{Identity, IsIdentity};
use curve25519_dalek::edwards::ValidityCheck;
use curve25519_dalek::field::FieldElement32;
use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use subtle::Equal;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use utils::random::YRandom;
use crypto::elliptic::scalar::YScalar;

#[derive(Copy, Clone, Debug)]
pub struct YPoint(pub ExtendedPoint);

impl Default for YPoint {
    fn default() -> YPoint {
        YPoint(ED25519_BASEPOINT_POINT)
    }
}

impl YPoint {
    pub fn random() -> YPoint {
        let mut b = [0u8; 32];
        YRandom::bytes_mut(&mut b);
        let mut p = YPoint::from_bytes(&b);
        while p.is_err() {
            YRandom::bytes_mut(&mut b);
            p = YPoint::from_bytes(&b);
        }
        p.unwrap()
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

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&self.0.compress().as_bytes()[..]);
        b
    }

    pub fn from_hex(s: &str) -> YResult<YPoint> {
        let buf = s.from_hex()?;
        YPoint::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }

    pub fn x_field(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.0.X.to_bytes()[..]);
        buf
    }

    pub fn y_field(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.0.Y.to_bytes()[..]);
        buf
    }

    pub fn from_fields(x: &[u8], y: &[u8]) -> YResult<YPoint> {
        if x.len() != 32 {
            return Err(YErrorKind::InvalidLength.into());
        }
        if y.len() != 32 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut x_buf = [0u8; 32];
        for i in 0..32 {
            x_buf[i] = x[i];
        }
        let mut y_buf = [0u8; 32];
        for i in 0..32 {
            y_buf[i] = y[i];
        }
        let X = FieldElement32::from_bytes(&x_buf);
        let Y = FieldElement32::from_bytes(&y_buf);
        let Z = FieldElement32::one();
        let T = &X*&Y;
        let p = YPoint(ExtendedPoint {
            X: X,
            Y: Y,
            Z: Z,
            T: T,
        });
        Ok(p)
    }
}

pub fn diffie_hellman(sk: &YScalar, pk: &YPoint) -> [u8; 32] {
    let dh = (&sk.0*&pk.0.to_montgomery()).compress();
    dh.to_bytes()
}

impl PartialEq for YPoint {
    fn eq(&self, other: &YPoint) -> bool {
        self.0.ct_eq(&other.0) == 1u8
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
