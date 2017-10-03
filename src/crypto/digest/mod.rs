use typenum::consts::U64;
use generic_array::GenericArray;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YDigest(pub GenericArray<u8, U64>);

impl YDigest {
  pub fn from_bytes(b: [u8; 64]) -> YDigest {
    YDigest(*GenericArray::from_slice(&b[..]))
  }

  pub fn to_bytes(&self) -> [u8; 64] {
    let mut b = [0u8; 64];
    for i in 0..64 {
      b[i] = self.0[i]
    }
    b
  }
}

