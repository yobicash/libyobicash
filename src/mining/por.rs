use rand::{Rng, StdRng, SeedableRng};
use MAX_LEN;
use check_size;
use check_length;
use errors::*;
use crypto::utils::check_binary_size;
use crypto::hash::Hash;
use crypto::hash::check_hash_size;
use crypto::merkle::merkle_root;
use crypto::merkle::verify_merkle_root;
use mining::target::check_target_bits;

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

pub fn segments_rng(seed: &Hash) -> YResult<StdRng> {
    check_hash_size(seed)?;
    let _seed: Vec<usize> = seed.iter().map(|el| *el as usize).collect();
    let rng = StdRng::from_seed(_seed.as_slice());
    Ok(rng)
}

pub fn segments_idxs(seed: &Hash, bits: u32, len: u32) -> YResult<Vec<u32>> {
    check_hash_size(seed)?;
    check_target_bits(bits)?;
    if len > MAX_LEN as u32 {
        return Err(YErrorKind::InvalidLength.into());
    }
    let mut rng = segments_rng(seed)?;
    let mut idxs: Vec<u32> = Vec::new();
    for _ in 0..bits {
        idxs.push(rng.gen_range(0, len));
    }
    Ok(idxs)
}

pub fn segment_start_idx(seed: &Hash, len: u32) -> YResult<u32> {
    check_hash_size(seed)?;
    if len > MAX_LEN as u32 {
        return Err(YErrorKind::InvalidLength.into());
    }
    let mut rng = segments_rng(seed)?;
    let stop = len - (SEGMENT_SIZE as u32);
    let idx = rng.gen_range(0, stop);
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
