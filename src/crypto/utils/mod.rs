use sodiumoxide::init as _init;
use sodiumoxide::randombytes;
use size::check_size;
use errors::*;

pub fn init() -> YResult<()> {
    if !_init() {
        return Err(YErrorKind::NotThreadSafe.into());
    }
    Ok(())
}

pub fn randombytes(len: usize) -> YResult<Vec<u8>> {
    init()?;
    let bin = randombytes::randombytes(len);
    Ok(bin)
}

pub fn check_binary_size(bin: &[u8], tlen: u32) -> YResult<()> {
    check_size(bin)?;
    let len = bin.len();
    if len != tlen as usize {
        return Err(YErrorKind::InvalidLength.into());
    }
    Ok(())
}
