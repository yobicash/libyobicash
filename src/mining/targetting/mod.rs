use byteorder::{ByteOrder, BigEndian};
use chrono::{DateTime, Utc};
use errors::*;
use crypto::hash::*;
use mining::utils::*;

use std::num::Wrapping;

pub const MIN_BITS: u32 = 1;
pub const MAX_BITS: u32 = 256;

pub fn check_target_bits(bits: u32) -> YResult<()> {
    if bits < MIN_BITS {
        return Err(YErrorKind::InvalidBits.into());
    }
    if bits > MAX_BITS {
        return Err(YErrorKind::InvalidBits.into());
    }
    Ok(())
}

pub fn target_from_bits(bits: u32) -> YResult<Vec<u8>> {
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

pub fn target_bits(target: &Vec<u8>) -> YResult<u32> {
    if target.len() != HASH_SIZE {
        return Err(YErrorKind::InvalidLength.into());
    }

    let t_sl = &target.as_slice()[0..8];
    let target_u32 = BigEndian::read_u32(t_sl);
    let bits = target_u32.leading_zeros() as u32;
    Ok(bits)
}

pub fn retarget_bits(old: u32, old_t: DateTime<Utc>, new_t: DateTime<Utc>, confirm_t: u32) -> YResult<u32> {
    check_target_bits(old)?;
    
    if new_t <= old_t {
        return Err(YErrorKind::InvalidTime.into());
    }
   
    let old_stamp = old_t.timestamp() as u32;
    let new_stamp = new_t.timestamp() as u32;
    let old_confirm_time = new_stamp -  old_stamp;

    let bits = old / old_confirm_time * confirm_t;
    Ok(bits)
}
