use libyobicash::models::input::*;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::hash::nonce_from_u32;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

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

#[test]
fn unique_inputs_succ() {
    let len = 10;
    let mut inputs: Vec<Input> = Vec::new();
    for i in 0..len {
        let tx_id = nonce_from_u32(i).unwrap();
        let input = Input::new(&tx_id, i).unwrap();
        inputs.push(input);
    }
    let res = check_unique_inputs(&inputs);
    assert!(res.is_ok())
}

#[test]
fn unique_inputs_fail() {
    let idx = 10;
    let tx_id = nonce_from_u32(idx).unwrap();
    let input = Input::new(&tx_id, idx).unwrap();
    let len = 10;
    let inputs: Vec<Input> = repeat(input).take(len).collect();
    let res = check_unique_inputs(&inputs);
    assert!(res.is_err())
}
