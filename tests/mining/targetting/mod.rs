use libyobicash::mining::targetting::*;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn check_target_bits_succ() {
    let bits = MAX_BITS;
    let res = check_target_bits(bits);
    assert!(res.is_ok())
}

#[test]
fn check_target_bits_fail() {
    let bits = MAX_BITS + 1;
    let res = check_target_bits(bits);
    assert!(res.is_err())
}

#[test]
fn target_from_bits_succ() {
    let bits = MIN_BITS;
    let res = target_from_bits(bits);
    assert!(res.is_ok())
}

#[test]
fn target_from_bits_fail() {
    let bits = MIN_BITS - 1;
    let res = target_from_bits(bits);
    assert!(res.is_err())
}

#[test]
fn target_bits_succ() {
    let bits = MIN_BITS;
    let target = target_from_bits(bits).unwrap();
    let bits1 = target_bits(&target).unwrap();
    assert_eq!(bits, bits1)
}

#[test]
fn target_bits_fail() {
    let target = randombytes(HASH_SIZE +1).unwrap();
    let res = target_bits(&target);
    assert!(res.is_err())
}

#[test]
fn retarget_bits_succ() {
    let old_bits = MAX_BITS / 2;
    let old_t = 0;
    let new_t = 40;
    let confirm_t = 20;
    let new_bits = retarget_bits(old_bits, old_t, new_t, confirm_t).unwrap();
    assert!(new_bits < old_bits)
}

#[test]
fn retarget_bits_fail() {
    let old_bits = MAX_BITS / 2;
    let old_t = 40;
    let new_t = 0;
    let confirm_t = 20;
    let res = retarget_bits(old_bits, old_t, new_t, confirm_t);
    assert!(res.is_err())
}

#[test]
fn target_compare_succ() {
    let old_bits = MAX_BITS / 2;
    let old_target = target_from_bits(old_bits).unwrap();
    let old_t = 0;
    let new_t = 40;
    let confirm_t = 20;
    let new_bits = retarget_bits(old_bits, old_t, new_t, confirm_t).unwrap();
    let new_target = target_from_bits(new_bits).unwrap();
    assert!(new_target < old_target)
}
