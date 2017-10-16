use curve25519_dalek::edwards::IsIdentity;
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

    pub fn derive_shared_key(&self, other: &YPublicKey) -> YResult<YKey64> {
        let _key = &other.pk * &self.0.sk;
        if !_key.is_identity() {
            let h = YHash64::hash(&_key.x_field()[..]);
            YKey64::from_bytes(h.to_bytes().as_slice())
        } else {
            let reason = String::from("point at infinity");
            Err(YErrorKind::InvalidPoint(reason).into())
        }
    }

    pub fn encrypt(&self, other: &YPublicKey, plain: &[u8]) -> YResult<Vec<u8>> {
        let key = self.derive_shared_key(other)?.reduce();
        let cyph = YSymmetricEncryption::encrypt(key, plain)?;
        Ok(cyph)
    }

    pub fn decrypt(&self, other: &YPublicKey, cyph: &[u8]) -> YResult<Vec<u8>> {
        let key = self.derive_shared_key(other)?.reduce();
        let plain = YSymmetricEncryption::decrypt(key, cyph)?;
        Ok(plain)
    }

    pub fn authenticate(&self, other: &YPublicKey, cyph: &[u8]) -> YResult<YMACCode> {
        let key = self.derive_shared_key(other)?;
        let tag = YMAC::mac(key, cyph);
        Ok(tag)
    }

    pub fn verify(&self, other: &YPublicKey, cyph: &[u8], tag: YMACCode) -> YResult<bool> {
        let key = self.derive_shared_key(other)?;
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
