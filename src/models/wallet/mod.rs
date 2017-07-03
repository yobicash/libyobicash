use errors::*;
use crypto::sign::{Seed, PublicKey, SecretKey};
use crypto::sign::check_seed_size;
use crypto::sign::generate_keypair;
use crypto::sign::generate_keypair_from_seed;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Wallet {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl Wallet {
    pub fn new() -> Result<Self> {
        let (pk, sk) = generate_keypair()?;
        Ok(Wallet {
            public_key: pk,
            secret_key: sk,
        })
    }

    pub fn from_seed(seed: &Seed) -> Result<Self> {
        check_seed_size(seed)?;
        let (pk, sk) = generate_keypair_from_seed(seed)?;
        Ok(Wallet {
            public_key: pk,
            secret_key: sk,
        })
    }
}
