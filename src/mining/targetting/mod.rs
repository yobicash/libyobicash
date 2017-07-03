use byteorder::{ByteOrder, BigEndian};
use errors::*;
use crypto::hash::HASH_SIZE;
use crypto::hash::check_hash_size;
use mining::utils::*;

use std::num::Wrapping;

pub const MIN_BITS: u32 = 1;
pub const MAX_BITS: u32 = 256;

pub fn check_target_bits(bits: u32) -> Result<()> {
    if bits < MIN_BITS {
        return Err(ErrorKind::InvalidBits.into());
    }
    if bits > MAX_BITS {
        return Err(ErrorKind::InvalidBits.into());
    }
    Ok(())
}

pub fn target_from_bits(bits: u32) -> Result<Vec<u8>> {
    check_target_bits(bits)?;
    let min_target_u32 = Wrapping(u32::max_value());
    let target_u32 = (min_target_u32 >> (bits as usize)).0;
    let mut t_sl = [0u8; 4];
    BigEndian::write_u32(&mut t_sl, target_u32);
    let mut _target: Vec<u8> = Vec::new();
    _target.extend_from_slice(&mut t_sl);
    _target.append(&mut filled_vec(255u8, HASH_SIZE-4));
    let mut v = Vec::new();
    v.extend_from_slice(_target.as_slice());
    Ok(v)
}

pub fn target_bits(target: &Vec<u8>) -> Result<u32> {
    check_hash_size(target)?;
    let t_sl = &target.as_slice()[0..8];
    let target_u32 = BigEndian::read_u32(t_sl);
    let bits = target_u32.leading_zeros() as u32;
    Ok(bits)
}

pub fn retarget_bits(old_bits: u32, old_t: u64, new_t: u64, confirm_t: u32) -> Result<u32> {
    check_target_bits(old_bits)?;
    if new_t <= old_t {
        return Err(ErrorKind::InvalidTime.into());
    }
    let old_confirm_t = new_t - old_t;
    let mut bits = ((old_bits as u64) / old_confirm_t * (confirm_t as u64)) as u32;
    if bits < MIN_BITS {
        bits = MIN_BITS;
    } else if bits > MAX_BITS {
        bits = MAX_BITS;
    }
    Ok(bits)
}
