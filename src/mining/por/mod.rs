use byteorder::{BigEndian, ReadBytesExt};
use errors::*;
use length::MAX_LEN;
use length::check_length;
use size::check_size;
use crypto::utils::check_binary_size;
use crypto::hash::Hash;
use crypto::hash::hash;
use crypto::hash::check_hash_size;
use crypto::merkle::merkle_root;
use crypto::merkle::verify_merkle_root;
use mining::targetting::check_target_bits;
use std::io::Cursor;

pub const SEGMENT_SIZE: usize = 32;

pub type Segment = Vec<u8>;

pub fn check_segment_size(seg: &Segment) -> YResult<()> {
   check_binary_size(seg.as_slice(), SEGMENT_SIZE as u32) 
}

pub fn check_segments(segs: &Vec<Segment>) -> YResult<()> {
    check_length(segs)?;
    for _ in 0..segs.len() {
        check_segment_size(&segs[0])?;
    }
    Ok(())
}

pub fn read_u32_from_seed(seed: &Hash, max: u32) -> YResult<u32> {
    check_hash_size(seed)?;
    let mut c = Cursor::new(seed.to_owned());
    let n = c.read_u32::<BigEndian>()? % max;
    Ok(n)
}

pub fn segments_idxs(seed: &Hash, bits: u32, len: u32) -> YResult<Vec<u32>> {
    check_hash_size(seed)?;
    check_target_bits(bits)?;
    if len > MAX_LEN as u32 {
        return Err(YErrorKind::InvalidLength.into());
    }
    let mut idxs: Vec<u32> = Vec::new();
    let mut idxs_len = 0;
    let mut _seed = seed.to_owned();
    'push_idxs: for _ in 0..bits {
        let n = read_u32_from_seed(seed, len)?;
        for i in 0..idxs_len {
            if n == idxs[i] {
                continue 'push_idxs;
            }
        }
        _seed = hash(_seed.as_slice())?;
        idxs.push(n);
        idxs_len += 1;
    }
    Ok(idxs)
}

pub fn segment_start_idx(seed: &Hash, len: u32) -> YResult<u32> {
    check_hash_size(seed)?;
    if len > MAX_LEN as u32 {
        return Err(YErrorKind::InvalidLength.into());
    }
    let stop = len - (SEGMENT_SIZE as u32);
    let idx = read_u32_from_seed(seed, stop)?;
    Ok(idx)
}

pub fn read_segment(seed: &Hash, data: &Vec<u8>) -> YResult<Segment> {
    check_hash_size(seed)?;
    check_size(data.as_slice())?;
    let len = data.len() as u32;
    let idx = segment_start_idx(seed, len)?;
    let start = idx as usize;
    let stop = start + SEGMENT_SIZE;
    let sl = data.as_slice()[start..stop].as_ref();
    let mut seg = Vec::new();
    seg.extend_from_slice(sl);
    Ok(seg)
}

pub fn segments_root(segs: &Vec<Segment>) -> YResult<Hash> {
    check_segments(segs)?;
    merkle_root(segs)
}

pub fn verify_segments_root(segs: &Vec<Segment>, root: &Hash) -> YResult<bool> {
    check_segments(segs)?;
    check_hash_size(root)?;
    verify_merkle_root(segs, root)
}

pub fn check_segments_root(segs: &Vec<Segment>, root: &Hash) -> YResult<()> {
    if !verify_merkle_root(segs, root)? {
        return Err(YErrorKind::InvalidSegmentsRoot.into());
    }
    Ok(())
}
