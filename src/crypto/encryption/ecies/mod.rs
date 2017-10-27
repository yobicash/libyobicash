use errors::*;
use crypto::elliptic::point::YPoint;
use crypto::elliptic::keys::{YSecretKey, YPublicKey};
use crypto::encryption::symmetric::YSymmetricEncryption;
use crypto::mac::{YMAC, YMACCode};
use crypto::key::YKey64;
use crypto::hash::YHash64;

pub struct YECIES(pub YSecretKey);

impl YECIES {
    pub fn new(sk: YSecretKey) -> YECIES {
        YECIES(sk)
    }

    pub fn from_g(g: YPoint) -> YECIES {
        YECIES(YSecretKey::from_g(g))
    }

    pub fn public_key(&self) -> YPublicKey {
        self.0.public_key()
    }

    pub fn shared_key(&self, other: &YPublicKey) -> YResult<YKey64> {
        let _key = self.0.shared_key(other)?;
        let h = YHash64::hash(_key.to_bytes().as_slice());
        YKey64::from_bytes(h.to_bytes().as_slice())
    }

    pub fn encrypt(&self, other: &YPublicKey, plain: &[u8]) -> YResult<Vec<u8>> {
        let key = self.shared_key(other)?.reduce();
        let cyph = YSymmetricEncryption::encrypt(key, plain)?;
        Ok(cyph)
    }

    pub fn decrypt(&self, other: &YPublicKey, cyph: &[u8]) -> YResult<Vec<u8>> {
        let key = self.shared_key(other)?.reduce();
        let plain = YSymmetricEncryption::decrypt(key, cyph)?;
        Ok(plain)
    }

    pub fn authenticate(&self, other: &YPublicKey, cyph: &[u8]) -> YResult<YMACCode> {
        let key = self.shared_key(other)?;
        let tag = YMAC::mac(key, cyph);
        Ok(tag)
    }

    pub fn verify(&self, other: &YPublicKey, cyph: &[u8], tag: YMACCode) -> YResult<bool> {
        let key = self.shared_key(other)?;
        let mut mac = YMAC::new(key);
        mac.update(cyph);
        Ok(mac.verify(tag))
    }

    pub fn encrypt_and_authenticate(
        &self,
        other: &YPublicKey,
        plain: &[u8],
    ) -> YResult<(Vec<u8>, YMACCode)> {
        let cyph = self.encrypt(other, plain)?;
        let tag = self.authenticate(other, cyph.as_slice())?;
        Ok((cyph, tag))
    }

    pub fn verify_and_decrypt(
        &self,
        other: &YPublicKey,
        cyph: &[u8],
        tag: YMACCode,
    ) -> YResult<Vec<u8>> {
        let verified = self.verify(other, cyph, tag)?;
        if verified {
            self.decrypt(other, cyph)
        } else {
            Err(YErrorKind::InvalidCyph.into())
        }
    }
}
