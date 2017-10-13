pub mod aes_gcm;

use errors::*;
use crypto::key::YKey32;
use crypto::encryption::symmetric::aes_gcm::*;

pub struct YSymmetricEncryption;

impl YSymmetricEncryption {
    pub fn encrypt(key: YKey32, plain: &[u8]) -> YResult<Vec<u8>> {
        let mut cipher = AESGCM256::new(key.0);
        cipher.encrypt(plain)
    }

    pub fn decrypt(key: YKey32, ciph: &[u8]) -> YResult<Vec<u8>> {
        let mut cipher = AESGCM256::new(key.0);
        cipher.decrypt(ciph)
    }
}
