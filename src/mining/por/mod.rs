use byteorder::{BigEndian, ReadBytesExt};
use errors::*;
use length::check_length;
use size::check_size;
use crypto::hash::Hash;
use crypto::hash::hash;
use crypto::hash::check_hash_size;
use crypto::merkle::merkle_root;
use crypto::merkle::verify_merkle_root;
use mining::targetting::check_target_bits;
use std::io::Cursor;

pub const SEGMENT_SIZE: usize = 32;

pub type Segment = Vec<u8>;

pub fn check_segment_size(seg: &Segment) -> Result<()> {
    check_size(seg)?;
    if seg.len() > SEGMENT_SIZE {
        return Err(ErrorKind::InvalidLength.into())
    }
    Ok(())
}

pub fn check_segments(segs: &Vec<Segment>) -> Result<()> {
    check_length(segs)?;
    for _ in 0..segs.len() {
        check_segment_size(&segs[0])?;
    }
    Ok(())
}

pub fn random_u32_from_seed(seed: &Hash, max: u32) -> Result<u32> {
    check_hash_size(seed)?;
    let mut c = Cursor::new(seed.to_owned());
    let n = c.read_u32::<BigEndian>()? % max;
    Ok(n)
}

pub fn segments_idxs(seed: &Hash, bits: u32, len: u32) -> Result<Vec<u32>> {
    check_hash_size(seed)?;
    check_target_bits(bits)?;
    let mut idxs: Vec<u32> = Vec::new();
    let mut _seed = seed.to_owned();
    for _ in 0..bits {
        // NB: allow repetitions. Case: len << bits
        let n = random_u32_from_seed(&_seed, len)?;
        _seed = hash(_seed.as_slice())?;
        idxs.push(n);
    }
    Ok(idxs)
}

pub fn read_segment(seed: &Hash, data: &Vec<u8>) -> Result<Segment> {
    check_hash_size(seed)?;
    check_size(data.as_slice())?;
    let len = data.len() as u32;
    let idx = random_u32_from_seed(seed, len)?;
    let start = idx as usize;
    let stop = if len >= (start + SEGMENT_SIZE) as u32 {
        start + SEGMENT_SIZE
    } else {
        len as usize
    };
    let sl = data.as_slice()[start..stop].as_ref();
    let mut seg = Vec::new();
    seg.extend_from_slice(sl);
    Ok(seg)
}

pub fn segments_to_hashes(segs: &Vec<Segment>) -> Result<Vec<Hash>> {
    check_segments(segs)?;
    let mut hashes = Vec::new();
    for i in 0..segs.len() {
        let h = hash(&segs[i])?;
        hashes.push(h);
    }
    Ok(hashes)
}

pub fn segments_root(segs: &Vec<Segment>) -> Result<Hash> {
    check_segments(segs)?;
    let leafs = segments_to_hashes(segs)?;
    merkle_root(&leafs)
}

pub fn verify_segments_root(segs: &Vec<Segment>, root: &Hash) -> Result<bool> {
    check_segments(segs)?;
    check_hash_size(root)?;
    let leafs = segments_to_hashes(segs)?;
    verify_merkle_root(&leafs, root)
}
