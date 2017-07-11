use libc::{size_t, c_int, c_ulonglong};
use itertools::Itertools;
use errors::*;
use length::check_length;
use crypto::utils::init;
use crypto::utils::check_binary_size;
use std::ops::Index;
use std::iter::Iterator;

pub const SEED_SIZE: usize = 32;

pub type Seed = Vec<u8>;

pub fn check_seed_size(sig: &Seed) -> Result<()> {
   check_binary_size(sig.as_slice(), SEED_SIZE as u32) 
}

pub const SECRET_KEY_SIZE: usize = 64;

pub type SecretKey = Vec<u8>;

pub fn check_secret_key_size(sk: &SecretKey) -> Result<()> {
   check_binary_size(sk.as_slice(), SECRET_KEY_SIZE as u32) 
}

pub const PUBLIC_KEY_SIZE: usize = 32;

pub type PublicKey = Vec<u8>;

pub fn check_public_key_size(pk: &PublicKey) -> Result<()> {
   check_binary_size(pk.as_slice(), PUBLIC_KEY_SIZE as u32) 
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

#[link(name = "sodium")]
extern {
    pub fn crypto_sign_ed25519_bytes() -> size_t;
    
    pub fn crypto_sign_ed25519_seedbytes() -> size_t;
    
    pub fn crypto_sign_ed25519_publickeybytes() -> size_t;
    
    pub fn crypto_sign_ed25519_secretkeybytes() -> size_t;
    
    pub fn crypto_sign_ed25519_keypair(
        pk: *mut [u8; PUBLIC_KEY_SIZE],
        sk: *mut [u8; SECRET_KEY_SIZE]) -> c_int;
    
    pub fn crypto_sign_ed25519_seed_keypair(
        pk: *mut [u8; PUBLIC_KEY_SIZE],
        sk: *mut [u8; SECRET_KEY_SIZE],
        seed: *const [u8; SEED_SIZE]) -> c_int;
    
    pub fn crypto_sign_ed25519_detached(
        sig: *mut [u8; SIGNATURE_SIZE],
        siglen: *mut c_ulonglong,
        m: *const u8,
        mlen: c_ulonglong,
        sk: *const [u8; SECRET_KEY_SIZE]) -> c_int;

    pub fn crypto_sign_ed25519_verify_detached(
        sig: *const [u8; SIGNATURE_SIZE],
        m: *const u8,
        mlen: c_ulonglong,
        pk: *const [u8; PUBLIC_KEY_SIZE]) -> c_int;
}

pub fn _gen_keypair() -> (PublicKey, SecretKey) {
    unsafe {
        let mut _pk = [0u8; PUBLIC_KEY_SIZE];
        let mut _sk = [0u8; SECRET_KEY_SIZE];
        crypto_sign_ed25519_keypair(&mut _pk, &mut _sk);
        let mut pk = Vec::new();
        let mut sk = Vec::new();
        pk.extend_from_slice(&_pk[..]);
        sk.extend_from_slice(&_sk[..]);
        (pk, sk)
    }
}

pub fn _keypair_from_seed(seed: &[u8; SEED_SIZE]) -> (PublicKey, SecretKey) {
    unsafe {
        let mut _pk = [0u8; PUBLIC_KEY_SIZE];
        let mut _sk = [0u8; SECRET_KEY_SIZE];
        crypto_sign_ed25519_seed_keypair(&mut _pk,
                                         &mut _sk,
                                         seed);
        let mut pk = Vec::new();
        let mut sk = Vec::new();
        pk.extend_from_slice(&_pk[..]);
        sk.extend_from_slice(&_sk[..]);
        (pk, sk)
    }
}

pub fn _sign_detached(msg: &[u8; MESSAGE_SIZE], sk: &[u8; SECRET_KEY_SIZE]) -> Signature {
    unsafe {
        let mut sig = [0u8; SIGNATURE_SIZE];
        let mut siglen: c_ulonglong = 0;
        crypto_sign_ed25519_detached(&mut sig,
                                     &mut siglen,
                                     msg.as_ptr(),
                                     msg.len() as c_ulonglong,
                                     sk);
        assert_eq!(siglen, SIGNATURE_SIZE as c_ulonglong);
        let mut v = Vec::new();
        v.extend_from_slice(&sig[..]);
        v
    }
}

pub fn _verify_detached(sig: &[u8; SIGNATURE_SIZE],
                        msg: &[u8; MESSAGE_SIZE],
                        pk: &[u8; PUBLIC_KEY_SIZE]) -> bool {
    unsafe {
        0 == crypto_sign_ed25519_verify_detached(sig,
                                                 msg.as_ptr(),
                                                 msg.len() as c_ulonglong,
                                                 pk)
    }
}

pub fn generate_keypair() -> Result<(PublicKey, SecretKey)> {
    init()?;
    let keys = _gen_keypair();
    Ok(keys)
}

pub fn generate_keypair_from_seed(seed: &Seed) -> Result<(PublicKey, SecretKey)> {
    check_seed_size(seed)?;
    let mut _seed = [0u8; SEED_SIZE];
    for i in 0..SEED_SIZE {
        _seed[i] = seed[i]
    }
    let keys = _keypair_from_seed(&_seed);
    Ok(keys)
}

pub fn sign(msg: &Message, sk: &SecretKey) -> Result<Signature> {
    init()?;
    check_message_size(msg)?;
    check_secret_key_size(sk)?;
    let mut _msg = [0u8; MESSAGE_SIZE];
    for i in 0..MESSAGE_SIZE {
        _msg[i] = msg[i]
    }
    let mut _sk = [0u8; SECRET_KEY_SIZE];
    for i in 0..SECRET_KEY_SIZE {
        _sk[i] = sk[i]
    }
    let signature = _sign_detached(&_msg, &_sk);
    Ok(signature)
}

pub fn verify_signature(sig: &Signature, msg: &Message, pk: &PublicKey) -> Result<bool> {
    init()?;
    check_signature_size(sig)?;
    check_message_size(msg)?;
    check_public_key_size(pk)?;
    let mut _sig = [0u8; SIGNATURE_SIZE];
    for i in 0..SIGNATURE_SIZE {
        _sig[i] = sig[i]
    }
    let mut _msg = [0u8; MESSAGE_SIZE];
    for i in 0..MESSAGE_SIZE {
        _msg[i] = msg[i]
    }
    let mut _pk = [0u8; PUBLIC_KEY_SIZE];
    for i in 0..PUBLIC_KEY_SIZE {
        _pk[i] = pk[i]
    }
    let verified = _verify_detached(&_sig, &_msg, &_pk);
    Ok(verified)
}

#[derive(Clone, Debug)]
pub struct SecretKeys {
    length: u32,
    idx: u32,
    items: Vec<SecretKey>,
}

impl SecretKeys {
    pub fn new(items: &Vec<SecretKey>) -> Result<SecretKeys> {
        check_length(items)?;
        let len = items.len();
        Ok(SecretKeys {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: SecretKey) {
        self.items.push(item)
    }

    pub fn check(&self) -> Result<()> {
        let len = self.length;
        if self.idx >= len {
            return Err(ErrorKind::IndexOutOfRange.into());
        }
        if len != self.items.len() as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }
}

impl Index<usize> for SecretKeys {
    type Output = SecretKey;

    fn index(&self, idx: usize) -> &SecretKey {
        self.items.index(idx)
    }
}

impl Iterator for SecretKeys {
    type Item = SecretKey;

    fn next(&mut self) -> Option<SecretKey> {
        match self.check() {
            Ok(_) => {
                let item = self.items[self.idx as usize].to_owned();
                self.idx += 1;
                Some(item)
            },
            Err(_) => { None },
        }
    }
}

pub fn unique_secret_keys(sks: &Vec<SecretKey>) -> Result<Vec<SecretKey>> {
    Ok(SecretKeys::new(sks)?.unique().collect())
}

pub fn check_unique_secret_keys(sks: &Vec<SecretKey>) -> Result<()> {
    let uniques: Vec<SecretKey> = SecretKeys::new(sks)?.unique().collect();
    if uniques.len() != sks.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct PublicKeys {
    length: u32,
    idx: u32,
    items: Vec<PublicKey>,
}

impl PublicKeys {
    pub fn new(items: &Vec<PublicKey>) -> Result<PublicKeys> {
        check_length(items)?;
        let len = items.len();
        Ok(PublicKeys {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: PublicKey) {
        self.items.push(item)
    }

    pub fn check(&self) -> Result<()> {
        let len = self.length;
        if self.idx >= len {
            return Err(ErrorKind::IndexOutOfRange.into());
        }
        if len != self.items.len() as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }
}

impl Index<usize> for PublicKeys {
    type Output = PublicKey;

    fn index(&self, idx: usize) -> &PublicKey {
        self.items.index(idx)
    }
}

impl Iterator for PublicKeys {
    type Item = PublicKey;

    fn next(&mut self) -> Option<PublicKey> {
        match self.check() {
            Ok(_) => {
                let item = self.items[self.idx as usize].to_owned();
                self.idx += 1;
                Some(item)
            },
            Err(_) => { None },
        }
    }
}

pub fn unique_public_keys(pks: &Vec<PublicKey>) -> Result<Vec<PublicKey>> {
    Ok(PublicKeys::new(pks)?.unique().collect())
}

pub fn check_unique_public_keys(pks: &Vec<PublicKey>) -> Result<()> {
    let uniques: Vec<PublicKey> = PublicKeys::new(pks)?.unique().collect();
    if uniques.len() != pks.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Signatures {
    length: u32,
    idx: u32,
    items: Vec<Signature>,
}

impl Signatures {
    pub fn new(items: &Vec<Signature>) -> Result<Signatures> {
        check_length(items)?;
        let len = items.len();
        Ok(Signatures {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Signature) {
        self.items.push(item)
    }

    pub fn check(&self) -> Result<()> {
        let len = self.length;
        if self.idx >= len {
            return Err(ErrorKind::IndexOutOfRange.into());
        }
        if len != self.items.len() as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }
}

impl Index<usize> for Signatures {
    type Output = Signature;

    fn index(&self, idx: usize) -> &Signature {
        self.items.index(idx)
    }
}

impl Iterator for Signatures {
    type Item = Signature;

    fn next(&mut self) -> Option<Signature> {
        match self.check() {
            Ok(_) => {
                let item = self.items[self.idx as usize].to_owned();
                self.idx += 1;
                Some(item)
            },
            Err(_) => { None },
        }
    }
}

pub fn unique_signatures(sigs: &Vec<Signature>) -> Result<Vec<Signature>> {
    Ok(Signatures::new(sigs)?.unique().collect())
}

pub fn check_unique_signatures(sigs: &Vec<Signature>) -> Result<()> {
    let uniques: Vec<Signature> = Signatures::new(sigs)?.unique().collect();
    if uniques.len() != sigs.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
