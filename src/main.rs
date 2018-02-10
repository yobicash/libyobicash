#[macro_use]
extern crate serde_json;
extern crate yobicrypto;

use yobicrypto::{Scalar, ZKPWitness};
use yobicrypto::HexSerialize;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let testnet_instance = Scalar::random();
    let testnet_witness = ZKPWitness::new(testnet_instance).unwrap();

    let mainnet_instance = Scalar::random();
    let mainnet_witness = ZKPWitness::new(mainnet_instance).unwrap();

    let creds = json!({
        "testnet_instance": testnet_instance.to_hex().unwrap(),
        "testnet_witness": testnet_witness.to_hex().unwrap(),
        "mainnet_instance": mainnet_instance.to_hex().unwrap(),
        "mainnet_witness": mainnet_witness.to_hex().unwrap(),
    });

    let mut file = File::create("/home/chritchens/.yobicash.creds.json").unwrap();
    let content = serde_json::to_vec_pretty(&creds).unwrap();
    file.write_all(&content).unwrap();    
}
