use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialize::hex::{FromHex, ToHex};
use crypto::hash::digest::*;
use crypto::hash::sha::*;
use errors::*;
use std::io::Cursor;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct YBalloonParams {
    pub s_cost: u32,
    pub t_cost: u32,
    pub delta: u32,
}

impl Default for YBalloonParams {
    fn default() -> YBalloonParams {
        YBalloonParams {
            s_cost: 1,
            t_cost: 1,
            delta: 3,
        }
    }
}

impl YBalloonParams {
    pub fn new(s_cost: u32, t_cost: u32, delta: u32) -> YResult<YBalloonParams> {
        if s_cost == 0 {
            return Err(YErrorKind::InvalidBalloonSCost.into());
        }
        if t_cost == 0 {
            return Err(YErrorKind::InvalidBalloonTCost.into());
        }
        if delta < 3 {
            return Err(YErrorKind::InvalidBalloonDelta.into());
        }
        Ok(YBalloonParams {
            s_cost: s_cost,
            t_cost: t_cost,
            delta: delta,
        })
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = Vec::new();
        buf.write_u32::<BigEndian>(self.s_cost)?;
        buf.write_u32::<BigEndian>(self.t_cost)?;
        buf.write_u32::<BigEndian>(self.delta)?;
        Ok(buf)
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YBalloonParams> {
        if b.len() != 12 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut reader = Cursor::new(b);
        let mut params = YBalloonParams::default();
        params.s_cost = reader.read_u32::<BigEndian>()?;
        params.t_cost = reader.read_u32::<BigEndian>()?;
        params.delta = reader.read_u32::<BigEndian>()?;
        Ok(params)
    }

    pub fn from_hex(s: &str) -> YResult<YBalloonParams> {
        let buf = s.from_hex()?;
        YBalloonParams::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn check(&self) -> YResult<()> {
        if self.s_cost == 0 {
            return Err(YErrorKind::InvalidBalloonSCost.into());
        }
        if self.t_cost == 0 {
            return Err(YErrorKind::InvalidBalloonTCost.into());
        }
        if self.delta < 3 {
            return Err(YErrorKind::InvalidBalloonDelta.into());
        }
        Ok(())
    }
}

type YBalloonBlockBuffer32 = Vec<YDigest32>;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct YBalloon256 {
    pub salt: YDigest32,
    pub params: YBalloonParams,
}

impl YBalloon256 {
    pub fn new(
        salt: YDigest32, params: YBalloonParams) -> YResult<YBalloon256> {
        params.check()?;
        Ok(YBalloon256 {
            salt: salt,
            params: params,
        })
    }

    pub fn check(&self) -> YResult<()> {
        self.params.check()
    }

    // only counting the memory used for hashing
    pub fn memory(&self) -> u64 {
        let a = self.params.s_cost as u64;
        let b = self.params.t_cost as u64;
        let c = self.params.delta as u64;
        32*(a + (b-1)*(1 + 2*(c-1)))
    }

    pub fn hash(&self, msg: &[u8]) -> YResult<YDigest32> {
        self.check()?;

        let mut cnt = 0u32;
        let mut buf = YBalloonBlockBuffer32::new();

        for _ in 0..self.params.s_cost {
            buf.push(YDigest32::default())
        }

        let mut buf_0 = Vec::new();
        buf_0.write_u32::<BigEndian>(cnt)?;
        cnt = cnt + 1;
        buf_0.extend_from_slice(msg);
        buf_0.extend_from_slice(self.salt.to_bytes().as_slice());

        buf[0] = YSHA256::hash(buf_0.as_slice());

        for m in 1..self.params.s_cost as usize {

            let mut buf_m_1 = Vec::new();
            buf_m_1.write_u32::<BigEndian>(cnt)?;
            cnt = cnt + 1;
            buf_m_1.extend_from_slice(buf[m-1].to_bytes().as_slice());

            buf[m] = YSHA256::hash(buf_m_1.as_slice());
        }

        // TODO: fix the algo online, contact the guys (t > 0)
        for t in 0..(self.params.t_cost-1) as usize {
            // TODO: fix the algo online, contact the guys
            for m in 1..(self.params.s_cost-1) as usize {

                let prev = buf[(m-1 as usize) % self.params.s_cost as usize];
                let mut buf_m_2 = Vec::new();
                buf_m_2.write_u32::<BigEndian>(cnt)?;
                cnt = cnt + 1;
                buf_m_2.extend_from_slice(prev.to_bytes().as_slice());
                buf_m_2.extend_from_slice(buf[m].to_bytes().as_slice());
                buf[m] = YSHA256::hash(buf_m_2.as_slice());

                for i in 0..(self.params.delta-1) as usize {
                    // NB: block obtained by hashing: count it to get the actual spent memory
                    let mut buf_idx_block = Vec::new();
                    buf_idx_block.write_u32::<BigEndian>(t as u32)?;
                    buf_idx_block.write_u32::<BigEndian>(m as u32)?;
                    buf_idx_block.write_u32::<BigEndian>(i as u32)?;
                    let idx_block = YSHA256::hash(buf_idx_block.as_slice());

                    let mut buf_i_1 = Vec::new();
                    buf_i_1.write_u32::<BigEndian>(cnt)?;
                    cnt = cnt + 1;
                    buf_i_1.extend_from_slice(self.salt.to_bytes().as_slice());
                    buf_i_1.extend_from_slice(idx_block.to_bytes().as_slice());
                    // TODO: should we hear those guys even here?
                    let mut other: u32 = YSHA256::hash(buf_i_1.as_slice()).to_u32() % self.params.s_cost;
                    let mut buf_i_2 = Vec::new();
                    buf_i_2.write_u32::<BigEndian>(cnt)?;
                    cnt = cnt + 1;
                    buf_i_2.extend_from_slice(buf[m].to_bytes().as_slice());
                    buf_i_2.extend_from_slice(buf[other as usize].to_bytes().as_slice());
                    buf[m] = YSHA256::hash(buf_i_2.as_slice());
                }
            }
        }

        Ok(buf[(self.params.s_cost-1) as usize])
    }
}

type YBalloonBlockBuffer64 = Vec<YDigest64>;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct YBalloon512 {
    pub salt: YDigest64,
    pub params: YBalloonParams,
}

impl YBalloon512 {
    pub fn new(
        salt: YDigest64, params: YBalloonParams) -> YResult<YBalloon512> {
        params.check()?;
        Ok(YBalloon512 {
            salt: salt,
            params: params,
        })
    }

    pub fn check(&self) -> YResult<()> {
        self.params.check()
    }

    // only counting the memory used for hashing
    pub fn memory(&self) -> u64 {
        let a = self.params.s_cost as u64;
        let b = self.params.t_cost as u64;
        let c = self.params.delta as u64;
        64*(a + (b-1)*(1 + 2*(c-1)))
    }

    pub fn hash(&self, msg: &[u8]) -> YResult<YDigest64> {
        self.check()?;

        let mut cnt = 0u32;
        let mut buf = YBalloonBlockBuffer64::new();

        for _ in 0..self.params.s_cost {
            buf.push(YDigest64::default())
        }

        let mut buf_0 = Vec::new();
        buf_0.write_u32::<BigEndian>(cnt)?;
        cnt = cnt + 1;
        buf_0.extend_from_slice(msg);
        buf_0.extend_from_slice(self.salt.to_bytes().as_slice());

        buf[0] = YSHA512::hash(buf_0.as_slice());

        for m in 1..self.params.s_cost as usize {

            let mut buf_m_1 = Vec::new();
            buf_m_1.write_u32::<BigEndian>(cnt)?;
            cnt = cnt + 1;
            buf_m_1.extend_from_slice(buf[m-1].to_bytes().as_slice());

            buf[m] = YSHA512::hash(buf_m_1.as_slice());
        }

        // TODO: fix the algo online, contact the guys (t > 0)
        for t in 0..(self.params.t_cost-1) as usize {
            // TODO: fix the algo online, contact the guys
            for m in 1..(self.params.s_cost-1) as usize {

                let prev = buf[(m-1 as usize) % self.params.s_cost as usize];
                let mut buf_m_2 = Vec::new();
                buf_m_2.write_u32::<BigEndian>(cnt)?;
                cnt = cnt + 1;
                buf_m_2.extend_from_slice(prev.to_bytes().as_slice());
                buf_m_2.extend_from_slice(buf[m].to_bytes().as_slice());
                buf[m] = YSHA512::hash(buf_m_2.as_slice());

                for i in 0..(self.params.delta-1) as usize {
                    // NB: block obtained by hashing
                    let mut buf_idx_block = Vec::new();
                    buf_idx_block.write_u32::<BigEndian>(t as u32)?;
                    buf_idx_block.write_u32::<BigEndian>(m as u32)?;
                    buf_idx_block.write_u32::<BigEndian>(i as u32)?;
                    let idx_block = YSHA512::hash(buf_idx_block.as_slice());

                    let mut buf_i_1 = Vec::new();
                    buf_i_1.write_u32::<BigEndian>(cnt)?;
                    cnt = cnt + 1;
                    buf_i_1.extend_from_slice(self.salt.to_bytes().as_slice());
                    buf_i_1.extend_from_slice(idx_block.to_bytes().as_slice());
                    // TODO: should we hear those guys even here?
                    let mut other: u32 = YSHA512::hash(buf_i_1.as_slice()).to_u32() % self.params.s_cost;
                    let mut buf_i_2 = Vec::new();
                    buf_i_2.write_u32::<BigEndian>(cnt)?;
                    cnt = cnt + 1;
                    buf_i_2.extend_from_slice(buf[m].to_bytes().as_slice());
                    buf_i_2.extend_from_slice(buf[other as usize].to_bytes().as_slice());
                    buf[m] = YSHA512::hash(buf_i_2.as_slice());
                }
            }
        }

        Ok(buf[(self.params.s_cost-1) as usize])
    }
}
