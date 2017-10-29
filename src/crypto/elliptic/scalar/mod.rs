use sha2::Sha512;
use curve25519_dalek::scalar::Scalar;
use rand::thread_rng;
use subtle::Equal;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Index, IndexMut};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use utils::biguint::YBigUint;

#[derive(Copy, Clone, Debug)]
pub struct YScalar(pub Scalar);

impl Default for YScalar {
    fn default() -> YScalar {
        YScalar::zero()
    }
}

impl YScalar {
    pub fn zero() -> YScalar {
        YScalar(Scalar::zero())
    }

    pub fn one() -> YScalar {
        YScalar(Scalar::one())
    }

    pub fn random() -> YScalar {
        YScalar(Scalar::random(&mut thread_rng()))
    }

    pub fn hash_from_bytes(b: &[u8]) -> YScalar {
        YScalar(Scalar::hash_from_bytes::<Sha512>(b))
    }

    // NB: scalars are 32 bytes bytearrays in little endian
    pub fn from_bytes(b: &[u8]) -> YResult<YScalar> {
        if b.len() != 32 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut scalar = Scalar::zero();
        for i in 0..32 {
            scalar.0[i] = b[i];
        }
        Ok(YScalar(scalar))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&self.0.as_bytes()[..]);
        b
    }

    pub fn from_hex(s: &str) -> YResult<YScalar> {
        let buf = s.from_hex()?;
        YScalar::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }

    pub fn from_biguint(n: &YBigUint) -> YResult<YScalar> {
        YScalar::from_bytes(n.to_little_endian().as_slice())
    }

    pub fn to_biguint(&self) -> YBigUint {
        YBigUint::from_little_endian(&self.0.as_bytes()[..])
    }

    pub fn from_u64(n: u64) -> YResult<YScalar> {
        YScalar::from_biguint(&YBigUint::from_u64(n))
    }

    // NB: panics in case of failure
    pub fn to_u64(&self) -> u64 {
        self.to_biguint().to_u64()
    }

    pub fn invert(&self) -> YScalar {
        YScalar(self.0.invert())
    }

    pub fn multiply_add(a: &YScalar, b: &YScalar, c: &YScalar) -> YScalar {
        YScalar(Scalar::multiply_add(&a.0, &b.0, &c.0))
    }
}

impl PartialEq for YScalar {
    fn eq(&self, other: &YScalar) -> bool {
        self.0.ct_eq(&other.0) == 1u8
    }
}

impl Eq for YScalar {
}

impl<'a, 'b> Add<&'b YScalar> for &'a YScalar {
    type Output = YScalar;

    fn add(self, other: &'b YScalar) -> YScalar {
        YScalar(self.0.add(&other.0))
    }
}

impl<'b> AddAssign<&'b YScalar> for YScalar {
    fn add_assign(&mut self, other: &'b YScalar) {
        self.0.add_assign(&other.0)
    }
}

impl<'a, 'b> Sub<&'b YScalar> for &'a YScalar {
    type Output = YScalar;

    fn sub(self, other: &'b YScalar) -> YScalar {
        YScalar(self.0.sub(&other.0))
    }
}

impl<'b> SubAssign<&'b YScalar> for YScalar {
    fn sub_assign(&mut self, other: &'b YScalar) {
        self.0.sub_assign(&other.0)
    }
}

impl<'a, 'b> Mul<&'b YScalar> for &'a YScalar {
    type Output = YScalar;

    fn mul(self, other: &'b YScalar) -> YScalar {
        YScalar(self.0.mul(&other.0))
    }
}

impl<'b> MulAssign<&'b YScalar> for YScalar {
    fn mul_assign(&mut self, other: &'b YScalar) {
        self.0.mul_assign(&other.0)
    }
}

impl Index<usize> for YScalar {
    type Output = u8;

    fn index(&self, idx: usize) -> &u8 {
        self.0.index(idx)
    }
}

impl IndexMut<usize> for YScalar {
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        self.0.index_mut(idx)
    }
}
