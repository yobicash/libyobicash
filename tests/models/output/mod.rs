use libyobicash::models::output::*;
use libyobicash::models::address::hash_to_address;
use libyobicash::models::amount::Amount;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn new_output_succ() {
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let res = Output::new(&Amount::new(amount as u32), &to, &data);
    assert!(res.is_ok())
}

#[test]
fn new_output_fail() {
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let amount = 10;
    let data = randombytes(amount-1).unwrap();
    let res = Output::new(&Amount::new(amount as u32), &to, &data);
    assert!(res.is_err())
}

#[test]
fn check_output_succ() {
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let output = Output::new(&Amount::new(amount as u32), &to, &data).unwrap();
    let res = output.check();
    assert!(res.is_ok())
}
