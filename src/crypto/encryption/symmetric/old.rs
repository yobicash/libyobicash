use typenum::consts::U16;
use generic_array::GenericArray;
use rust_crypto::aes::{KeySize, ctr};
use rust_crypto::symmetriccipher::SynchronousStreamCipher;
use serialize::hex::{FromHex, ToHex};
use errors::*;
use crypto::key::YKey32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YIV(pub GenericArray<u8, U16>);

impl YIV {
    pub fn from_bytes(b: &[u8]) -> YResult<YIV> {
        if b.len() != 16 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(YIV(*GenericArray::from_slice(&b[..])))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(self.0.as_slice());
        b
    }

    pub fn from_hex(s: &str) -> YResult<YIV> {
        let buf = s.from_hex()?;
        YIV::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }
}

pub struct YSymmetricEncryption;

impl YSymmetricEncryption {
    pub fn encrypt(key: YKey32, iv: YIV, plain: &[u8]) -> Vec<u8> {
        let _key = &key.to_bytes()[..];
        let _iv = &iv.to_bytes()[..];
        let mut stream_cypher = ctr(KeySize::KeySize256, _key, _iv);
        let mut cypher: Vec<u8> = Vec::new();
        stream_cypher.process(plain, cypher.as_mut_slice());
        cypher
    }

    pub fn decrypt(key: YKey32, iv: YIV, cyph: &[u8]) -> Vec<u8> {
        YSymmetricEncryption::encrypt(key, iv, cyph)
    }
}
