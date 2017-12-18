use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use crypto::hash::digest::YDigest64;
use crypto::hash::sha::YSHA512;
use crypto::mac::YMACCode;
use crypto::elliptic::keys::{YSecretKey, YPublicKey};
use crypto::encryption::ecies::YECIES;
use amount::YAmount;
use std::io::{Write, Read, Cursor};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct YData {
    pub data: Vec<u8>,
    pub checksum: YDigest64,
    pub tag: YMACCode,
}

impl YData {
    pub fn new(sk: &YSecretKey, other: &YPublicKey, plain: &[u8]) -> YResult<YData> {
        if sk.g != other.g {
            let msg = String::from("Invalid generator");
            return Err(YErrorKind::InvalidPoint(msg).into());
        }
        let ecies = YECIES::new(sk.clone());
        let (data, tag) = ecies.encrypt_and_authenticate(other, plain)?;
        let digest = YSHA512::hash(data.as_slice());
        Ok(YData {
            data: data,
            checksum: digest,
            tag: tag,
        })
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = Vec::new();
        let size = self.data.len() as u32;
        buf.write_u32::<BigEndian>(size)?;
        buf.write(self.data.as_slice())?;
        buf.write(&self.checksum.to_bytes()[..])?;
        buf.write(&self.tag.to_bytes()[..])?;
        Ok(buf)
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YData> {
        if b.len() < 100 {
            return Err(YErrorKind::InvalidLength.into());
        }

        let mut reader = Cursor::new(b);

        let mut data = YData::default();

        let size = reader.read_u32::<BigEndian>()?;

        let mut data_buf = Vec::new();
        for _ in 0..size as usize {
            data_buf.push(0);
        }

        reader.read_exact(data_buf.as_mut_slice())?;
        data.data = data_buf;

        let mut checksum_buf = [0u8; 64];
        reader.read_exact(&mut checksum_buf[..])?;
        data.checksum = YDigest64::from_bytes(&checksum_buf[..])?;

        let mut tag_buf = [0u8; 64];
        reader.read_exact(&mut tag_buf)?;
        data.tag = YMACCode::from_bytes(&tag_buf[..])?;

        data.check()?;

        Ok(data)
    }

    pub fn from_hex(s: &str) -> YResult<YData> {
        let buf = s.from_hex()?;
        YData::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn verify(&self, sk: &YSecretKey, other: &YPublicKey) -> YResult<bool> {
        let ecies = YECIES::new(sk.clone());
        ecies.verify(other, self.data.as_slice(), self.tag)
    }

    pub fn verify_and_decrypt(&self, sk: &YSecretKey, other: &YPublicKey) -> YResult<Vec<u8>> {
        let ecies = YECIES::new(sk.clone());
        ecies.verify_and_decrypt(other, self.data.as_slice(), self.tag)
    }

    pub fn amount(&self) -> YResult<YAmount> {
        YAmount::from_u64(self.data.len() as u64)
    }

    pub fn drop(mut self) -> YData {
        self.data = Vec::new();
        self
    }

    pub fn check(&self) -> YResult<()> {
        let digest = YSHA512::hash(self.data.as_slice());
        if self.checksum != digest {
            return Err(YErrorKind::InvalidChecksum.into());
        }
        Ok(())
    }
}
