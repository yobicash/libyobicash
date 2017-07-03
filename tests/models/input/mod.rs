use libyobicash::models::input::*;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn new_input_succ() {
    let tx_id = randombytes(HASH_SIZE).unwrap();
    let idx = 10;
    let res = Input::new(&tx_id, idx);
    assert!(res.is_ok())
}

#[test]
fn new_input_fail() {
    let tx_id = randombytes(HASH_SIZE-1).unwrap();
    let idx = 10;
    let res = Input::new(&tx_id, idx);
    assert!(res.is_err())
}

#[test]
fn check_input_succ() {
    let tx_id = randombytes(HASH_SIZE).unwrap();
    let idx = 10;
    let input = Input::new(&tx_id, idx).unwrap();
    let res = input.check();
    assert!(res.is_ok())
}
