use sodiumoxide::crypto::sign as _sign;
use errors::*;
use crypto::utils::init;
use crypto::utils::check_binary_size;

pub const SEED_SIZE: usize = 32;

pub type Seed = Vec<u8>;

pub fn check_seed_size(sig: &Seed) -> Result<()> {
   check_binary_size(sig.as_slice(), SEED_SIZE as u32) 
}

pub const SECRETKEY_SIZE: usize = 64;

pub type SecretKey = Vec<u8>;

pub fn check_secret_key_size(sk: &SecretKey) -> Result<()> {
   check_binary_size(sk.as_slice(), SECRETKEY_SIZE as u32) 
}

pub const PUBLICKEY_SIZE: usize = 32;

pub type PublicKey = Vec<u8>;

pub fn check_public_key_size(pk: &PublicKey) -> Result<()> {
   check_binary_size(pk.as_slice(), PUBLICKEY_SIZE as u32) 
}

pub const MESSAGE_SIZE: usize = 32;

pub type Message = Vec<u8>;

pub fn check_message_size(sig: &Message) -> Result<()> {
   check_binary_size(sig.as_slice(), MESSAGE_SIZE as u32) 
}

pub const SIGNATURE_SIZE: usize = 64;

pub type Signature = Vec<u8>;

pub fn check_signature_size(sig: &Signature) -> Result<()> {
   check_binary_size(sig.as_slice(), SIGNATURE_SIZE as u32) 
}

pub fn generate_keypair() -> Result<(PublicKey, SecretKey)> {
    init()?;
    let (_pk, _sk) = _sign::gen_keypair();
    Ok((_pk.as_ref().to_vec(), _sk.0[..].to_vec()))
}

pub fn generate_keypair_from_seed(seed: &Message) -> Result<(PublicKey, SecretKey)> {
    check_seed_size(seed)?;
    let _s = _sign::Seed::from_slice(seed.as_slice()).unwrap();
    let (_pk, _sk) = _sign::keypair_from_seed(&_s);
    Ok((_pk.as_ref().to_vec(), _sk.0[..].to_vec()))
}

pub fn sign(msg: &Message, sk: &SecretKey) -> Result<Signature> {
    init()?;
    check_message_size(msg)?;
    check_secret_key_size(sk)?;
    let _sk = _sign::SecretKey::from_slice(sk.as_slice()).unwrap();
    Ok(_sign::sign_detached(msg.as_slice(), &_sk).as_ref().to_vec())
}

pub fn verify_signature(sig: &Signature, msg: &Message, pk: &PublicKey) -> Result<bool> {
    init()?;
    check_signature_size(sig)?;
    check_message_size(msg)?;
    check_public_key_size(pk)?;
    let _pk = _sign::PublicKey::from_slice(pk.as_slice()).unwrap();
    let _sig = _sign::Signature::from_slice(sig.as_slice()).unwrap();
    Ok(_sign::verify_detached(&_sig, msg.as_slice(), &_pk))
}
