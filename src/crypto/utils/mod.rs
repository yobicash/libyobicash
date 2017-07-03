use sodiumoxide::init as _init;
use sodiumoxide::randombytes;
use size::check_size;
use errors::*;

pub fn init() -> Result<()> {
    if !_init() {
        return Err(ErrorKind::NotThreadSafe.into());
    }
    Ok(())
}

pub fn randombytes(len: usize) -> Result<Vec<u8>> {
    init()?;
    let bin = randombytes::randombytes(len);
    Ok(bin)
}

pub fn check_binary_size(bin: &[u8], tlen: u32) -> Result<()> {
    check_size(bin)?;
    let len = bin.len();
    if len != tlen as usize {
        return Err(ErrorKind::InvalidLength.into());
    }
    Ok(())
}
