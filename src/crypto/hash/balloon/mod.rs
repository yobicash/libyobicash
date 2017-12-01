use byteorder::{LittleEndian, WriteBytesExt};
use crypto::hash::digest::*;
use crypto::hash::sha::*;
use errors::*;

pub type YBalloonBlockBuffer32 = Vec<YDigest32>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct YBalloon256 {
    salt: YDigest32,
    s_cost: u32,
    t_cost: u32,
    delta: u32,
}

impl YBalloon256 {
    pub fn new(
        salt: YDigest32,
        s_cost: u32,
        t_cost: u32,
        delta: u32) -> YResult<YBalloon256> {
        if delta < 3 {
            return Err(YErrorKind::InvalidBalloonDelta.into());
        }
        Ok(YBalloon256 {
            salt: salt,
            s_cost: s_cost,
            t_cost: t_cost,
            delta: delta,
        })
    }

    pub fn check(&self) -> YResult<()> {
        if self.delta < 3 {
            return Err(YErrorKind::InvalidBalloonDelta.into());
        }
        Ok(())
    }

    // only counting the memory used for hashing
    pub fn memory(&self) -> u64 {
        let a = self.s_cost as u64;
        let b = self.t_cost as u64;
        let c = self.delta as u64;
        32*(2*a + (b-1)*(a-1)*(1 + 2*(c-1)))
    }

    pub fn hash(&self, msg: &[u8]) -> YResult<YDigest32> {
        self.check()?;

        let mut cnt = 0u32;
        let mut buf = YBalloonBlockBuffer32::new();

        for _ in 0..self.s_cost {
            buf.push(YDigest32::default())
        }

        let mut buf_0 = Vec::new();
        buf_0.write_u32::<LittleEndian>(cnt)?;
        cnt = cnt + 1;
        buf_0.extend_from_slice(msg);
        buf_0.extend_from_slice(self.salt.to_bytes().as_slice());

        buf[0] = YSHA256::hash(buf_0.as_slice());

        for m in 1..self.s_cost as usize {

            let mut buf_m_1 = Vec::new();
            buf_m_1.write_u32::<LittleEndian>(cnt)?;
            cnt = cnt + 1;
            buf_m_1.extend_from_slice(buf[m-1].to_bytes().as_slice());

            buf[m] = YSHA256::hash(buf_m_1.as_slice());

            for t in 0..(self.t_cost-1) as usize {
                for m in 0..(self.s_cost-1) as usize {

                    let prev = buf[(m-1 as usize) % self.s_cost as usize];
                    let mut buf_m_2 = Vec::new();
                    buf_m_2.write_u32::<LittleEndian>(cnt)?;
                    cnt = cnt + 1;
                    buf_m_2.extend_from_slice(prev.to_bytes().as_slice());
                    buf_m_2.extend_from_slice(buf[m].to_bytes().as_slice());
                    buf[m] = YSHA256::hash(buf_m_2.as_slice());

                    for i in 0..(self.delta-1) as usize {
                        // NB: block obtained by hashing: count it to get the actual spent memory
                        let mut buf_idx_block = Vec::new();
                        buf_idx_block.write_u32::<LittleEndian>(t as u32)?;
                        buf_idx_block.write_u32::<LittleEndian>(m as u32)?;
                        buf_idx_block.write_u32::<LittleEndian>(i as u32)?;
                        let idx_block = YSHA256::hash(buf_idx_block.as_slice());

                        let mut buf_i_1 = Vec::new();
                        buf_i_1.write_u32::<LittleEndian>(cnt)?;
                        cnt = cnt + 1;
                        buf_i_1.extend_from_slice(self.salt.to_bytes().as_slice());
                        buf_i_1.extend_from_slice(idx_block.to_bytes().as_slice());
                        let mut other: u32 = YSHA256::hash(buf_i_1.as_slice()).to_u32();
                        let mut buf_i_2 = Vec::new();
                        buf_i_2.write_u32::<LittleEndian>(cnt)?;
                        cnt = cnt + 1;
                        buf_i_2.extend_from_slice(buf[m].to_bytes().as_slice());
                        buf_i_2.extend_from_slice(buf[other as usize].to_bytes().as_slice());
                        buf[m] = YSHA256::hash(buf_i_2.as_slice());
                    }
                }
            }
        }

        Ok(buf[(self.s_cost-1) as usize])
    }
}

pub type YBalloonBlockBuffer64 = Vec<YDigest64>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct YBalloon512 {
    salt: YDigest64,
    s_cost: u32,
    t_cost: u32,
    delta: u32,
}

impl YBalloon512 {
    pub fn new(
        salt: YDigest64,
        s_cost: u32,
        t_cost: u32,
        delta: u32) -> YResult<YBalloon512> {
        if delta < 3 {
            return Err(YErrorKind::InvalidBalloonDelta.into());
        }
        Ok(YBalloon512 {
            salt: salt,
            s_cost: s_cost,
            t_cost: t_cost,
            delta: delta,
        })
    }

    pub fn check(&self) -> YResult<()> {
        if self.delta < 3 {
            return Err(YErrorKind::InvalidBalloonDelta.into());
        }
        Ok(())
    }

    // only counting the memory used for hashing
    pub fn memory(&self) -> u64 {
        let a = self.s_cost as u64;
        let b = self.t_cost as u64;
        let c = self.delta as u64;
        64*(2*a + (b-1)*(a-1)*(1 + 3*(c-1)))
    }

    pub fn hash(&self, msg: &[u8]) -> YResult<YDigest64> {
        self.check()?;

        let mut cnt = 0u32;
        let mut buf = YBalloonBlockBuffer64::new();

        for _ in 0..self.s_cost {
            buf.push(YDigest64::default())
        }

        let mut buf_0 = Vec::new();
        buf_0.write_u32::<LittleEndian>(cnt)?;
        cnt = cnt + 1;
        buf_0.extend_from_slice(msg);
        buf_0.extend_from_slice(self.salt.to_bytes().as_slice());

        buf[0] = YSHA512::hash(buf_0.as_slice());

        for m in 1..self.s_cost as usize {

            let mut buf_m_1 = Vec::new();
            buf_m_1.write_u32::<LittleEndian>(cnt)?;
            cnt = cnt + 1;
            buf_m_1.extend_from_slice(buf[m-1].to_bytes().as_slice());

            buf[m] = YSHA512::hash(buf_m_1.as_slice());

            for t in 0..(self.t_cost-1) as usize {
                for m in 0..(self.s_cost-1) as usize {

                    let prev = buf[(m-1 as usize) % self.s_cost as usize];
                    let mut buf_m_2 = Vec::new();
                    buf_m_2.write_u32::<LittleEndian>(cnt)?;
                    cnt = cnt + 1;
                    buf_m_2.extend_from_slice(prev.to_bytes().as_slice());
                    buf_m_2.extend_from_slice(buf[m].to_bytes().as_slice());
                    buf[m] = YSHA512::hash(buf_m_2.as_slice());

                    for i in 0..(self.delta-1) as usize {
                        // NB: block obtained by hashing
                        let mut buf_idx_block = Vec::new();
                        buf_idx_block.write_u32::<LittleEndian>(t as u32)?;
                        buf_idx_block.write_u32::<LittleEndian>(m as u32)?;
                        buf_idx_block.write_u32::<LittleEndian>(i as u32)?;
                        let idx_block = YSHA512::hash(buf_idx_block.as_slice());

                        let mut buf_i_1 = Vec::new();
                        buf_i_1.write_u32::<LittleEndian>(cnt)?;
                        cnt = cnt + 1;
                        buf_i_1.extend_from_slice(self.salt.to_bytes().as_slice());
                        buf_i_1.extend_from_slice(idx_block.to_bytes().as_slice());
                        let mut other: u32 = YSHA512::hash(buf_i_1.as_slice()).to_u32();
                        let mut buf_i_2 = Vec::new();
                        buf_i_2.write_u32::<LittleEndian>(cnt)?;
                        cnt = cnt + 1;
                        buf_i_2.extend_from_slice(buf[m].to_bytes().as_slice());
                        buf_i_2.extend_from_slice(buf[other as usize].to_bytes().as_slice());
                        buf[m] = YSHA512::hash(buf_i_2.as_slice());
                    }
                }
            }
        }

        Ok(buf[(self.s_cost-1) as usize])
    }
}
