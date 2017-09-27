use crypto::digest::YDigest;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;

#[derive(Copy, Clone, Debug)]
pub struct YInput {
  id: YDigest,
  idx: u64,
  height: u64,
  g: YPoint,
  t: YPoint,
  r: YScalar,
}
