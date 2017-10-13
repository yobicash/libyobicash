use serialize::hex::FromHex;
use libyobicash::crypto::encryption::symmetric::aes_gcm::*;

fn test_vectors() -> Vec<(String, String, String)> {
    vec![
        (
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f".to_string(),
            "00112233445566778899aabbccddeeff".to_string(),
            "8ea2b7ca516745bfeafc49904b496089".to_string()
        ),
        (
            "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4".to_string(),
            "6bc1bee22e409f96e93d7e117393172a".to_string(),
            "f3eed1bdb5d2a03c064b5a7e3db181f8".to_string()
        ),
        (
            "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4".to_string(),
            "ae2d8a571e03ac9c9eb76fac45af8e51".to_string(),
            "591ccb10d410ed26dc5ba74a31362870".to_string()
        ),
        (
            "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4".to_string(),
            "30c81c46a35ce411e5fbc1191a0a52ef".to_string(),
            "b6ed21b99ca6f4f9f153e7b1beafed1d".to_string()
        ),
        (
            "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4".to_string(),
            "f69f2445df4f9b17ad2b417be66c3710".to_string(),
            "23304b7a39f9f3ff067d8d8f9e24ecc7".to_string()
        ),
    ]
}

#[test]
fn aes_gcm_encrypt_test_vectors() {
    for v in test_vectors() {
        let key = *AES256GCMKey::from_slice(v.0.from_hex().unwrap().as_slice());
        let mut cipher = AESGCM256::new(key);
        let res = cipher.encrypt(v.1.from_hex().unwrap().as_slice()).unwrap();
        let test = v.2.from_hex().unwrap();
        assert_eq!(res, test.as_slice())
    }
}

#[test]
fn aes_gcm_decrypt_test_vectors() {
    for v in test_vectors() {
        let key = *AES256GCMKey::from_slice(v.0.from_hex().unwrap().as_slice());
        let mut cipher = AESGCM256::new(key);
        let res = cipher.decrypt(v.2.from_hex().unwrap().as_slice()).unwrap();
        let test = v.1.from_hex().unwrap();
        assert_eq!(res, test.as_slice())
    }
}
