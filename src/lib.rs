extern crate typenum;
extern crate generic_array;
extern crate sha2;
extern crate hmac;
extern crate hkdf;
extern crate curve25519_dalek;
extern crate rand;
extern crate bigint;
extern crate subtle;
extern crate crypto as rust_crypto;

pub mod utils;
pub mod crypto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
