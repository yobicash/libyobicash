use serialize::hex::{FromHex, ToHex};
use libyobicash::crypto::key::YKey64;
use libyobicash::crypto::kdf::YKDF;

fn test_vectors() -> Vec<(String, String, String, String)> {
  vec![
    ("403e03bb5e1dbbb6521bf2bc9c76beb773ad994ccc638c001f07aa824335563e98b223e96013e82a3c7fdeba860900ab58b964aea7c30928727e6ee6616786fd".to_string(),
    "8907a1e986c7cdce207bd8a00c8e21b8db085c678ec766772a0dd599ccfd160210c63f52180d8f4aeed72bbb9c98fdbf7a42068d12f940d3e989e1bca9a6362f30d03491c92e66520584928b6e893b3f4bf4f08731af749efa5cf7d1b3033f469ba841ee".to_string(),
    "4914d4b565ebdb3cadf8c25ef1767b59af273cf74ee8cb022e25eb27e093983d0ce6ee24dbd3ebfcb3479050b4bc5783bd1433bd3a7ff6d32a6f6cb8045647d3".to_string(),
    "9c9efc7bc1e58efd853a1579ba9daed64196800beede420505e5c5055c9e9b0e57ec5c7378d7a78b3fefa85df211a54bd83741b35a7b3f99303fbb0215aa4047".to_string()),
    ("714e059b65db81e8f56f3b5f82b460d87ecbed0ec7d1e5785083803374f4d5c9d87c2bd0bc5eca8dc17a675bf8ff5de9a4f7490a9ccc5d293f0b415bda2e6b8f1075b1295885bcd349d3138ebbc15e56b815209753adb99a82598ff665a4459317c92d66cbac98741c94a4b4e9e185a113f261bb2e2464a935288cd3884cbb1e".to_string(),
    "c634aec84c74cf65e1cebea803fe30589978d06a".to_string(),
    "".to_string(),
    "764809b2f3f3a6f1260dc3f82deae81f05ee752ed5dfe7954054a853d18aa429f98c52e1e97f0724e879218bc90352b19c33f56f5b7c12ac634c09cde610e372".to_string())]
}

#[test]
fn kdf_test_vectors_succ() {
  for v in test_vectors() {
    let salt = v.0.as_str().from_hex().unwrap();
    let ikm = v.1.as_str().from_hex().unwrap();
    let info = v.2.as_str().from_hex().unwrap();
    let res = YKDF::kdf(salt.as_slice(), ikm.as_slice(), info.as_slice());
    let test = YKey64::from_hex(v.3.as_str()).unwrap();
    assert_eq!(res, test)
  }
}
