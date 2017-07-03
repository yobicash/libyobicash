use libyobicash::mining::por::*;
use libyobicash::mining::targetting::MIN_BITS;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::hash::check_hash_size;
use libyobicash::crypto::utils::randombytes;

#[test]
fn check_segment_succ() {
    let seg = randombytes(SEGMENT_SIZE/2).unwrap();
    let res = check_segment_size(&seg);
    assert!(res.is_ok())
}

#[test]
fn check_segment_fail() {
    let seg = randombytes(SEGMENT_SIZE*2).unwrap();
    let res = check_segment_size(&seg);
    assert!(res.is_err())
}

#[test]
fn random_u32_succ() {
    let seed = randombytes(HASH_SIZE).unwrap();
    let max = 10;
    let num = random_u32_from_seed(&seed, max).unwrap();
    assert!(num < max)
}

#[test]
fn random_u32_fail() {
    let seed = randombytes(HASH_SIZE+1).unwrap();
    let max = 10;
    let res = random_u32_from_seed(&seed, max);
    assert!(res.is_err())
}

#[test]
fn segments_idxs_succ() {
    let seed = randombytes(HASH_SIZE).unwrap();
    let bits = MIN_BITS*2;
    let len = 10;
    let idxs = segments_idxs(&seed, bits, len).unwrap();
    assert_eq!(idxs.len() as u32, bits)
}

#[test]
fn segments_idxs_fail() {
    let seed = randombytes(HASH_SIZE+1).unwrap();
    let bits = MIN_BITS*2;
    let len = 10;
    let res = segments_idxs(&seed, bits, len);
    assert!(res.is_err())
}

#[test]
fn read_segment_succ() {
    let seed = randombytes(HASH_SIZE).unwrap();
    let size = 10;
    let data = randombytes(size).unwrap();
    let seg = read_segment(&seed, &data).unwrap();
    let res = check_segment_size(&seg);
    assert!(res.is_ok())
}

#[test]
fn read_segment_fail() {
    let seed = randombytes(HASH_SIZE+1).unwrap();
    let size = 10;
    let data = randombytes(size).unwrap();
    let res = read_segment(&seed, &data);
    assert!(res.is_err())
}

#[test]
fn segments_to_hashes_succ() {
    let len = 10;
    let mut segs = Vec::new();
    for _ in 0..len {
        let seg = randombytes(SEGMENT_SIZE).unwrap();
        segs.push(seg);
    }
    let hashes = segments_to_hashes(&segs).unwrap();
    for _ in 0..len {
        let res = check_hash_size(&hashes[0]);
        assert!(res.is_ok());
    }
}

#[test]
fn segments_to_hashes_fail() {
    let len = 10;
    let mut segs = Vec::new();
    for _ in 0..len {
        let seg = randombytes(SEGMENT_SIZE+1).unwrap();
        segs.push(seg);
    }
    let res = segments_to_hashes(&segs);
    assert!(res.is_err())
}

#[test]
fn segments_root_succ() {
    let len = 10;
    let mut segs = Vec::new();
    for _ in 0..len {
        let seg = randombytes(SEGMENT_SIZE).unwrap();
        segs.push(seg);
    }
    let sr = segments_root(&segs).unwrap();
    let res = verify_segments_root(&segs, &sr).unwrap();
    assert!(res)
}

#[test]
fn segments_root_fail() {
    let len = 10;
    let mut segs = Vec::new();
    for _ in 0..len {
        let seg = randombytes(SEGMENT_SIZE).unwrap();
        segs.push(seg);
    }
    let sr = randombytes(SEGMENT_SIZE-1).unwrap();
    let res = verify_segments_root(&segs, &sr);
    assert!(res.is_err())
}
