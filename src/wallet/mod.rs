use errors::*;
use crypto::sign::{Seed, PublicKey, SecretKey};
use crypto::sign::check_seed_size;
use crypto::sign::generate_keypair;
use crypto::sign::generate_keypair_from_seed;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct YWallet {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl YWallet {
    pub fn new() -> YResult<Self> {
        let (pk, sk) = generate_keypair()?;
        Ok(YWallet {
            public_key: pk,
            secret_key: sk, // TODO: danger?
        })
    }

    pub fn from_seed(seed: &Seed) -> YResult<Self> {
        check_seed_size(seed)?;
        let (pk, sk) = generate_keypair_from_seed(seed)?;
        Ok(YWallet {
            public_key: pk,
            secret_key: sk, // TODO: danger?
        })
    }
}
