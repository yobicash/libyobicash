use byteorder::{ByteOrder, BigEndian};
use errors::*;
use crypto::hash::*;
use mining::utils::*;
use mining::targetting::check_target_bits;
use mining::targetting::target_from_bits;

pub const MIN_S_COST: u32 = 1;
pub const MIN_T_COST: u32 = 1;
pub const MIN_DELTA: u32 = 3;

pub fn check_s_cost(s_cost: u32) -> YResult<()> {
    if s_cost < MIN_S_COST {
        return Err(YErrorKind::InvalidSCost.into());
    }
    Ok(())
}

pub fn check_t_cost(t_cost: u32) -> YResult<()> {
    if t_cost < MIN_T_COST {
        return Err(YErrorKind::InvalidTCost.into());
    }
    Ok(())
}

pub fn check_delta(delta: u32) -> YResult<()> {
    if delta < MIN_DELTA {
        return Err(YErrorKind::InvalidDelta.into());
    }
    Ok(())
}

pub fn balloon_nonce_from_u32(n: u32) -> YResult<Hash> {
    let mut buf = [0; 4];
    BigEndian::write_u32(&mut buf, n);
    hash(&buf[..])
}

pub fn balloon_hash(seed: &Hash, nonce: &Hash, _s_cost: u32, _t_cost: u32, _delta: u32) -> YResult<Hash> {
        check_hash_size(seed)?;
        check_hash_size(nonce)?;
        check_s_cost(_s_cost)?;
        check_t_cost(_t_cost)?;
        check_delta(_delta)?;

        let s_cost = _s_cost as usize;
        let t_cost = _t_cost as usize;
        let delta = _delta as usize;

        if delta < 3 {
            return Err(YErrorKind::InvalidDelta.into());
        }

        let mut buf_it = Vec::new();
        buf_it.extend_from_slice([0u8; HASH_SIZE].as_ref());
        let mut buf: Vec<Hash> = filled_vec(buf_it, s_cost);

        let mut buf0: Vec<u8> = Vec::new();
        buf0.extend_from_slice(seed.to_owned().as_ref());
        buf0.extend_from_slice(nonce.to_owned().as_ref());
        buf[0] = hash(buf0.as_slice())?;

        for m in 1..s_cost {
            buf[m] = hash(buf[m-1].as_ref())?;

            for t in 0..t_cost {
                let prev: Vec<u8> = buf[(m-1) % s_cost].to_owned();
                let mut buf1b: Vec<u8> = Vec::new();
                buf1b.extend_from_slice(prev.to_owned().as_slice());
                buf1b.extend_from_slice(buf[m].to_owned().as_ref());
                buf[m] = hash(buf1b.as_slice())?;

                for i in 0..delta {
                    let idx_bchannel: Vec<u8> = ints_to_bchannel(t, m, i).into();
                    let mut other_seed: Vec<u8> = Vec::new();
                    other_seed.extend_from_slice(nonce.to_owned().as_ref());
                    other_seed.extend_from_slice(idx_bchannel.to_owned().as_slice());
                    let other: usize = to_int(hash(other_seed.as_slice())?.as_ref(), s_cost);
                    let mut buf2b: Vec<u8> = Vec::new();
                    buf2b.extend_from_slice(buf[m].to_owned().as_ref());
                    buf2b.extend_from_slice(buf[other].to_owned().as_ref());
                    buf[m] = hash(buf2b.as_slice())?;
                }
            }
        }

        let h = buf[s_cost-1].to_owned();
        Ok(h)
}

pub fn balloon_mine(target_bits: u32, seed: &Hash, s_cost: u32, t_cost: u32, delta: u32) -> YResult<Option<u32>> {
    check_target_bits(target_bits)?;
    check_hash_size(seed)?;
    check_s_cost(s_cost)?;
    check_t_cost(t_cost)?;
    check_delta(delta)?;

    let mut i: u32 = 0;
    let mut nonce: Option<u32> = None;
    let target = target_from_bits(target_bits)?;

    loop {
        let _nonce = balloon_nonce_from_u32(i)?;
        let digest = balloon_hash(seed, &_nonce, s_cost, t_cost, delta)?;
        if digest.to_owned() <= target.to_owned() {
            nonce = Some(i);
            break;
        }
        if i == u32::max_value() {
            break;
        }
        i += 1;
    }

    Ok(nonce)
}

pub fn balloon_verify(target_bits: u32, seed: &Hash, nonce: u32, s_cost: u32, t_cost: u32, delta: u32) -> YResult<bool> {
    check_target_bits(target_bits)?;
    check_hash_size(seed)?;
    check_s_cost(s_cost)?;
    check_t_cost(t_cost)?;
    check_delta(delta)?;
    let _nonce = balloon_nonce_from_u32(nonce)?;
    let res = balloon_hash(seed, &_nonce, s_cost, t_cost, delta)?;
    let target = target_from_bits(target_bits)?;
    let ok = res <= target.to_owned();
    Ok(ok)
}

pub fn balloon_memory(s_cost: u32, t_cost: u32, delta: u32) -> YResult<u32> {
    check_s_cost(s_cost)?;
    check_t_cost(t_cost)?;
    check_delta(delta)?;
    let mem = s_cost * ( 1 + (s_cost-1) * (1 + t_cost * (1 + 2 * delta)));
    Ok(mem)
}
