use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use crypto::hash::digest::YDigest64;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use crypto::zkp::schnorr_protocol::YSchnorrProtocolPublic;
use output::YOutput;
use std::io::{Write, Read, Cursor};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YInput {
    pub id: YDigest64,
    pub idx: u32,
    pub g: YPoint,
    pub t: YPoint,
    pub c: YScalar,
    pub r: YScalar,
}

impl YInput {
    pub fn new(
        id: YDigest64,
        idx: u32,
        prot: YSchnorrProtocolPublic,
    ) -> YInput {
        YInput {
            id: id,
            idx: idx,
            g: prot.g,
            t: prot.t,
            c: prot.c,
            r: prot.r,
        }
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.write(&self.id.to_bytes()[..])?;
        buf.write_u32::<BigEndian>(self.idx)?;
        buf.write(&self.g.to_bytes()[..])?;
        buf.write(&self.t.to_bytes()[..])?;
        buf.write(&self.c.to_bytes()[..])?;
        buf.write(&self.r.to_bytes()[..])?;
        Ok(buf)
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YInput> {
        if b.len() != 196 {
            return Err(YErrorKind::InvalidLength.into());
        }

        let mut input = YInput::default();

        let mut reader = Cursor::new(b);

        let mut id_buf = [0u8; 64];
        reader.read_exact(&mut id_buf[..])?;
        input.id = YDigest64::from_bytes(&id_buf[..])?;

        input.idx = reader.read_u32::<BigEndian>()?;

        let mut g_buf = [0u8; 32];
        reader.read_exact(&mut g_buf[..])?;
        input.g = YPoint::from_bytes(&g_buf[..])?;

        let mut t_buf = [0u8; 32];
        reader.read_exact(&mut t_buf[..])?;
        input.t = YPoint::from_bytes(&t_buf[..])?;

        let mut c_buf = [0u8; 32];
        reader.read_exact(&mut c_buf[..])?;
        input.c = YScalar::from_bytes(&c_buf[..])?;

        let mut r_buf = [0u8; 32];
        reader.read_exact(&mut r_buf[..])?;
        input.r = YScalar::from_bytes(&r_buf[..])?;

        Ok(input)
    }

    pub fn from_hex(s: &str) -> YResult<YInput> {
        let buf = s.from_hex()?;
        YInput::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn verify(&self, out: &YOutput) -> bool {
        let prot = YSchnorrProtocolPublic {
            g: self.g,
            w: out.recipient.pk,
            t: self.t,
            c: self.c,
            r: self.r,
        };
        prot.verify()
    }
}
