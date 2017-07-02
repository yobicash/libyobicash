use errors::*;
use crypto::hash::Hash;
use crypto::hash::HASH_SIZE;
use crypto::hash::check_hash_size;

pub type Address = Vec<u8>;

pub const ADDRESS_PREFIX: u8 = 7;

pub fn hash_to_address(h: &Hash) -> YResult<Address> {
    check_hash_size(h)?;
    let mut addr = vec![ADDRESS_PREFIX];
    addr.extend_from_slice(h.as_slice());
    Ok(addr)
}

pub fn check_address_size(addr: &Address) -> YResult<()> {
    if addr.len() != HASH_SIZE + 1 {
        return Err(YErrorKind::InvalidSize.into())
    }
    Ok(())
}

pub fn check_address(addr: &Address) -> YResult<()> {
    check_address_size(addr)?;
    if addr[0] != ADDRESS_PREFIX {
        return Err(YErrorKind::InvalidAddress.into())
    }
    Ok(())
}
