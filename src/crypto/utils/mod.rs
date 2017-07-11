use libc::{c_int, size_t};
use size::check_size;
use errors::*;
use std::iter::repeat;

#[link(name = "sodium")]
extern {
    pub fn sodium_init() -> c_int;
    pub fn randombytes_buf(buf: *mut u8, size: size_t);
}

fn _init() -> bool {
    unsafe {
        sodium_init() != -1
    }
}

pub fn _randombytes(size: usize) -> Vec<u8> {
    unsafe {
        let mut buf: Vec<u8> = repeat(0u8).take(size).collect();
        let pbuf = buf.as_mut_ptr();
        randombytes_buf(pbuf, size);
        buf
    }
}

pub fn init() -> Result<()> {
    if !_init() {
        return Err(ErrorKind::NotThreadSafe.into());
    }
    Ok(())
}

pub fn randombytes(len: usize) -> Result<Vec<u8>> {
    init()?;
    let bin = _randombytes(len);
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
