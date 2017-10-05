use rand::random;
use libyobicash::crypto::digest::YDigest;

#[test]
fn digest_from_bytes_succ() {
  let mut b = [0u8; 64];
  for i in 0..64 {
    b[i] = random::<u8>();
  }
  let res = YDigest::from_bytes(&b[..]);
  assert!(res.is_ok())
}

#[test]
fn digest_from_bytes_fail() {
  let mut b = [0u8; 32];
  for i in 0..32 {
    b[i] = random::<u8>();
  }
  let res = YDigest::from_bytes(&b[..]);
  assert!(res.is_err())
}


