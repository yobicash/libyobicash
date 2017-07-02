use errors::*;
use std::u32;

pub const MAX_SIZE: usize = u32::MAX as usize; // for word32 machines

pub fn check_size(bin: &[u8]) -> YResult<()> {
    let len = bin.len();
    if len > MAX_SIZE {
        return Err(YErrorKind::InvalidSize.into());
    }
    Ok(())
}
