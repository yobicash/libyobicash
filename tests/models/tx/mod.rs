use chrono::Duration;
use libyobicash::models::tx::*;
use libyobicash::models::wallet::Wallet;
use libyobicash::models::address::hash_to_address;
use libyobicash::models::input::Input;
use libyobicash::models::amount::Amount;
use libyobicash::models::signers::Signers;
use libyobicash::models::content::Content;
use libyobicash::models::output::Output;
use libyobicash::models::outpoint::*;
use libyobicash::mining::por::*;
use libyobicash::mining::pow::balloon_nonce_from_u32;
use libyobicash::crypto::sign::PUBLICKEY_SIZE;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

#[test]
fn new_tx_succ() {
    let res = Tx::new();
    assert!(res.is_ok())
}

#[test]
fn set_time_succ() {
    let mut tx = Tx::new().unwrap();
    let mut time = tx.get_time();
    let d = Duration::hours(1);
    time = time.checked_sub_signed(d).unwrap();
    let res = tx.set_time(&time);
    assert!(res.is_ok())
}

#[test]
fn set_time_fail() {
    let mut tx = Tx::new().unwrap();
    let mut time = tx.get_time();
    let d = Duration::hours(1);
    time = time.checked_add_signed(d).unwrap();
    let res = tx.set_time(&time);
    assert!(res.is_err())
}

#[test]
fn set_version_succ() {
    let mut tx = Tx::new().unwrap();
    let mut version = tx.get_version();
    if version.major > 0 {
        version.major = version.major -1;
    } else if version.minor > 0 {
        version.minor = version.minor -1;
    } else if version.patch > 0 {
        version.patch = version.patch -1;
    } else {
        panic!("Invalid default version")
    }
    let res = tx.set_version(&version);
    assert!(res.is_ok())
}

#[test]
fn set_version_fail() {
    let mut tx = Tx::new().unwrap();
    let mut version = tx.get_version();
    version.major = version.major +1;
    let res = tx.set_version(&version);
    assert!(res.is_err())
}

#[test]
fn set_signers_succ() {
    let mut tx = Tx::new().unwrap();
    let pk1 = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight1 = 10;
    let mut pk2 = pk1.to_owned();
    pk2[0] = pk2[0] % 2 + 1;
    let weight2 = 200;
    let threshold = weight2;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&pk1, weight1).unwrap()
        .add_signer(&pk2, weight2).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let res = tx.set_signers(&signers);
    assert!(res.is_ok())
}

#[test]
fn set_signers_fail() {
    let mut tx = Tx::new().unwrap();
    let pk1 = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight1 = 10;
    let mut pk2 = pk1.to_owned();
    pk2[0] = pk2[0] % 2 + 1;
    let weight2 = 200;
    let threshold = weight2;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&pk1, weight1).unwrap()
        .add_signer(&pk2, weight2).unwrap()
        .set_threshold(threshold).unwrap();
    let res = tx.set_signers(&signers);
    assert!(res.is_err())
}

#[test]
fn add_input_succ() {
    let mut tx = Tx::new().unwrap();
    let len = 10;
    let max_idx = 100000;
    for _ in 0..len {
        let tx_id = randombytes(HASH_SIZE).unwrap();
        let idx = random_u32_from_seed(&tx_id, max_idx).unwrap();
        let input = Input::new(&tx_id, idx).unwrap();
        input.check().unwrap();
        let res = tx.add_input(&input);
        assert!(res.is_ok());
    }
    assert_eq!(len, tx.get_inputs_len())
}

#[test]
fn add_output_succ() {
    let mut tx = Tx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let len = 10;
    let max_amount = 100;
    let mut get_outputs_amount = Amount::new(0);
    for _ in 0..len {
        let seed = randombytes(HASH_SIZE).unwrap();
        let to = hash_to_address(&seed).unwrap();
        let amount = random_u32_from_seed(&seed, max_amount).unwrap();
        let _amount = Amount::new(amount);
        let data = randombytes(amount as usize).unwrap();
        let content = Content::new(&creators, &data).unwrap()
            .sign(&wallet).unwrap()
            .finalize().unwrap();
        let output = Output::new(&_amount, &to, &content).unwrap();
        output.check().unwrap();
        let res = tx.add_output(&output);
        assert!(res.is_ok());
        get_outputs_amount = get_outputs_amount + _amount;
    }
    assert_eq!(len, tx.get_outputs_len());
    assert_eq!(get_outputs_amount, tx.get_outputs_amount())
}

#[test]
fn set_fee_succ() {
    let mut tx = Tx::new().unwrap();
    let fee = Amount::new(10);
    tx.set_fee(&fee);
    assert_eq!(tx.get_tot_amount(), fee)
}

#[test]
fn check_balance_succ() {
    let mut tx = Tx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let len = 10;
    let inputs_amount = 110;
    let mut tot_amount = Amount::new(0);
    for _ in 0..len {
        let seed = randombytes(HASH_SIZE).unwrap();
        let to = hash_to_address(&seed).unwrap();
        let amount = 10;
        let _amount = Amount::new(amount);
        let data = randombytes(amount as usize).unwrap();
        let content = Content::new(&creators, &data).unwrap()
            .sign(&wallet).unwrap()
            .finalize().unwrap();
        let output = Output::new(&_amount, &to, &content).unwrap();
        output.check().unwrap();
        let res = tx.add_output(&output);
        assert!(res.is_ok());
        tot_amount = tot_amount + _amount;
    }
    let fee = Amount::new(10);
    tx.set_fee(&fee);
    tot_amount = tot_amount + fee;
    assert_eq!(tot_amount, tx.get_tot_amount());
    let res = tx.check_balance(&Amount::new(inputs_amount));
    assert!(res.is_ok())
}

#[test]
fn sign_succ() {
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let tx = Tx::new().unwrap()
        .set_signers(&signers).unwrap()
        .sign(&wallet1).unwrap()
        .sign(&wallet3).unwrap()
        .finalize().unwrap();
    let res = tx.check_signatures();
    assert!(res.is_ok())
}

#[test]
fn sign_fail() {
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let res = Tx::new().unwrap()
        .set_signers(&signers).unwrap()
        .sign(&wallet1).unwrap()
        .sign(&wallet2).unwrap()
        .finalize();
    assert!(res.is_err())
}

#[test]
fn finalize_succ() {
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let mut tx = Tx::new().unwrap();
    let len = 10;
    let max_idx = 100000;
    for _ in 0..len {
        let tx_id = randombytes(HASH_SIZE).unwrap();
        let idx = random_u32_from_seed(&tx_id, max_idx).unwrap();
        let input = Input::new(&tx_id, idx).unwrap();
        input.check().unwrap();
        tx.add_input(&input).unwrap();
    }
    for _ in 0..len {
        let seed = randombytes(HASH_SIZE).unwrap();
        let to = hash_to_address(&seed).unwrap();
        let amount = 10;
        let _amount = Amount::new(amount);
        let data = randombytes(amount as usize).unwrap();
        let content = Content::new(&creators, &data).unwrap()
            .sign(&wallet).unwrap()
            .finalize().unwrap();
        let output = Output::new(&_amount, &to, &content).unwrap();
        output.check().unwrap();
        tx.add_output(&output).unwrap();
    }
    let fee = Amount::new(10);
    tx.set_fee(&fee);
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    tx.set_signers(&signers).unwrap();
    tx.sign(&wallet1).unwrap();
    tx.sign(&wallet3).unwrap();
    tx.check_signatures().unwrap();
    let res = tx.finalize();
    assert!(res.is_ok())
}

#[test]
fn check_succ() {
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let mut tx = Tx::new().unwrap();
    let len = 10;
    let max_idx = 100000;
    for _ in 0..len {
        let tx_id = randombytes(HASH_SIZE).unwrap();
        let idx = random_u32_from_seed(&tx_id, max_idx).unwrap();
        let input = Input::new(&tx_id, idx).unwrap();
        input.check().unwrap();
        tx.add_input(&input).unwrap();
    }
    for _ in 0..len {
        let seed = randombytes(HASH_SIZE).unwrap();
        let to = hash_to_address(&seed).unwrap();
        let amount = 10;
        let _amount = Amount::new(amount);
        let data = randombytes(amount as usize).unwrap();
        let content = Content::new(&creators, &data).unwrap()
            .sign(&wallet).unwrap()
            .finalize().unwrap();
        let output = Output::new(&_amount, &to, &content).unwrap();
        output.check().unwrap();
        tx.add_output(&output).unwrap();
    }
    let fee = Amount::new(10);
    tx.set_fee(&fee);
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let res = tx.set_signers(&signers).unwrap()
        .sign(&wallet1).unwrap()
        .sign(&wallet3).unwrap()
        .finalize().unwrap()
        .check();
    assert!(res.is_ok())
}

#[test]
fn new_coinbase_succ() {
    let wallet = Wallet::new().unwrap();
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut to = Signers::new().unwrap();
    to = to
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    to.check().unwrap();
    let c_amount = Amount::new(10);
    let res = Tx::coinbase(&wallet, &to, &c_amount, &Vec::new());
    assert!(res.is_ok())
}

#[test]
fn new_coinbase_fail() {
    let wallet = Wallet::new().unwrap();
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut to = Signers::new().unwrap();
    to = to
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    to.check().unwrap();
    let c_amount = Amount::new(10);
    let res = Tx::coinbase(&wallet, &to, &c_amount, &vec![1]);
    assert!(res.is_err())
}

#[test]
fn unique_txs_succ() {
    let len = 10;
    let mut txs: Vec<Tx> = Vec::new();
    for i in 0..len {
        let mut tx = Tx::new().unwrap();
        let time = tx.get_time() - Duration::hours(i);
        tx.set_time(&time).unwrap();
        txs.push(tx);
    }
    let res = check_unique_txs(&txs);
    assert!(res.is_ok())
}

#[test]
fn unique_txs_fail() {
    let tx = Tx::new().unwrap();
    let len = 10;
    let txs: Vec<Tx> = repeat(tx).take(len).collect();
    let res = check_unique_txs(&txs);
    assert!(res.is_err())
}

#[test]
fn check_doublespending_succ() {
    let seed = randombytes(HASH_SIZE).unwrap();
    let wallet = Wallet::from_seed(&seed).unwrap();
    let weight = 10;
    let threshold = 5;
    let mut to = Signers::new().unwrap();
    to = to
        .add_signer(&wallet.public_key, weight).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    to.check().unwrap();
    let to_address = to.get_address();
    let mut outpoints = OutPoints::new(&Vec::new()).unwrap();
    let len = 10;
    for i in 0..len {
        let wallet = Wallet::new().unwrap();
        let amount = 10;
        let data = randombytes(amount).unwrap();
        let creators = Signers::new().unwrap()
            .add_signer(&wallet.public_key, 1).unwrap()
            .finalize().unwrap();
        let content = Content::new(&creators, &data).unwrap()
            .sign(&wallet).unwrap()
            .finalize().unwrap();
        let tx_id = balloon_nonce_from_u32(i).unwrap();
        let idx = 0;
        let output = Output::new(&Amount::new(amount as u32), &to_address, &content).unwrap();
        let outpoint = OutPoint::new(&tx_id, idx, &output).unwrap();
        outpoints.push(outpoint);
    }
    // NB: WEIRD THINGS HAPPENING HERE
    println!("outpoints: {:?}", outpoints);
    println!("oupoints.to_inputs(): {:?}", outpoints.to_inputs());
    let inputs = outpoints.to_inputs().unwrap();
    println!("inputs: {:?}", inputs);
    let mut tx = Tx::new().unwrap();
    for i in 0..inputs.len() {
        tx.add_input(&inputs[i]).unwrap();
    }
    let tot_amount = outpoints.tot_amount();
    let output = Output::no_content(&tot_amount, &to_address).unwrap();
    tx.add_output(&output).unwrap();
    let fee = Amount::new(0);
    tx.set_fee(&fee);
    tx.set_signers(&to).unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap()
        .check().unwrap();
    let res = tx.check_doublespending(&outpoints.to_raw());
    println!("res: {:?}", res);
    assert!(res.is_ok())
}

#[test]
fn check_doublespending_fail() {}

#[test]
fn tx_from_outpoints_succ() {}

#[test]
fn tx_from_outpoints_fail() {}
