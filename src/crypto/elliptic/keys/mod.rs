use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::{YPoint, diffie_hellman};
use crypto::key::YKey32;
use serialize::hex::{FromHex, ToHex};
use errors::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct YPublicKey {
    pub g: YPoint,
    pub pk: YPoint,
}

impl YPublicKey {
    pub fn new(g: YPoint, pk: YPoint) -> YPublicKey {
        YPublicKey { g: g, pk: pk }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.append(&mut self.g.to_bytes());
        buf.append(&mut self.pk.to_bytes());
        buf
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YPublicKey> {
        if b.len() != 64 {
            return Err(YErrorKind::InvalidLength.into());
        }

        let mut pk = YPublicKey::default();

        let g_buf = &b[0..32];
        pk.g = YPoint::from_bytes(g_buf)?;

        let pk_buf = &b[32..64];
        pk.pk = YPoint::from_bytes(pk_buf)?;

        Ok(pk)
    }

    pub fn from_hex(s: &str) -> YResult<YPublicKey> {
        let buf = s.from_hex()?;
        YPublicKey::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct YSecretKey {
    pub g: YPoint,
    pub sk: YScalar,
}

impl YSecretKey {
    pub fn new(g: YPoint, sk: YScalar) -> YSecretKey {
        YSecretKey { g: g, sk: sk }
    }

    pub fn random() -> YSecretKey {
        YSecretKey {
            g: YPoint::random(),
            sk: YScalar::random(),
        }
    }

    pub fn from_g(g: YPoint) -> YSecretKey {
        YSecretKey {
            g: g,
            sk: YScalar::random(),
        }
    }

    pub fn public_key(&self) -> YPublicKey {
        YPublicKey::new(self.g, &self.g * &self.sk)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.append(&mut self.g.to_bytes());
        buf.append(&mut self.sk.to_bytes());
        buf
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YSecretKey> {
        if b.len() != 64 {
            return Err(YErrorKind::InvalidLength.into());
        }

        let mut sk = YSecretKey::default();

        let g_buf = &b[0..32];
        sk.g = YPoint::from_bytes(g_buf)?;

        let sk_buf = &b[32..64];
        sk.sk = YScalar::from_bytes(sk_buf)?;

        Ok(sk)
    }

    pub fn from_hex(s: &str) -> YResult<YSecretKey> {
        let buf = s.from_hex()?;
        YSecretKey::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }

    pub fn shared_key(&self, pk: &YPublicKey) -> YResult<YKey32> {
        if self.g != pk.g {
            let msg = String::from("Invalid generator");
            return Err(YErrorKind::InvalidPoint(msg).into());
        }
        let key = diffie_hellman(&self.sk, &pk.pk);
        YKey32::from_bytes(&key[..])
    }
}
