use errors::*;
use std::u32;

pub const MAX_LEN: usize = u32::MAX as usize; // for word32 machines

pub fn check_length<T>(xs: &[T]) -> YResult<()> {
    let len = xs.len();
    if len > MAX_LEN {
        return Err(YErrorKind::InvalidLength.into());
    }
    Ok(())
}
