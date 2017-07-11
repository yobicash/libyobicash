use libyobicash::crypto::sign::*;
use libyobicash::crypto::hash::hash;
use libyobicash::crypto::utils::randombytes;

#[test]
fn test_crypto_sign_ed25519_bytes() {
    let signature_bytes = unsafe {
        crypto_sign_ed25519_bytes() as usize
    };
    assert_eq!(signature_bytes, SIGNATURE_SIZE)
}

#[test]
fn test_crypto_sign_ed25519_seedbytes() {
    let seed_bytes = unsafe {
        crypto_sign_ed25519_seedbytes() as usize
    };
    assert_eq!(seed_bytes, SEED_SIZE)
}

#[test]
fn test_crypto_sign_ed25519_publickeybytes() {
    let publickey_bytes = unsafe {
        crypto_sign_ed25519_publickeybytes() as usize
    };
    assert_eq!(publickey_bytes, PUBLIC_KEY_SIZE)
}

#[test]
fn test_crypto_sign_ed25519_secretkeybytes() {
    let secretkey_bytes = unsafe {
        crypto_sign_ed25519_secretkeybytes() as usize
    };
    assert_eq!(secretkey_bytes, SECRET_KEY_SIZE)
}

#[test]
fn seed_size_succ() {
    let seed = randombytes(SEED_SIZE).unwrap();
    let res = check_seed_size(&seed);
    assert!(res.is_ok())
}

#[test]
fn seed_size_fail() {
    let seed = randombytes(SEED_SIZE+1).unwrap();
    let res = check_seed_size(&seed);
    assert!(res.is_err())
}

#[test]
fn sk_size_succ() {
    let sk = randombytes(SECRET_KEY_SIZE).unwrap();
    let res = check_secret_key_size(&sk);
    assert!(res.is_ok())
}

#[test]
fn sk_size_fail() {
    let sk = randombytes(SECRET_KEY_SIZE-1).unwrap();
    let res = check_secret_key_size(&sk);
    assert!(res.is_err())
}

#[test]
fn pk_size_succ() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let res = check_public_key_size(&pk);
    assert!(res.is_ok())
}

#[test]
fn pk_size_fail() {
    let pk = randombytes(PUBLIC_KEY_SIZE+1).unwrap();
    let res = check_public_key_size(&pk);
    assert!(res.is_err())
}

#[test]
fn msg_size_succ() {
    let msg = randombytes(MESSAGE_SIZE).unwrap();
    let res = check_message_size(&msg);
    assert!(res.is_ok())
}

#[test]
fn msg_size_fail() {
    let msg = randombytes(MESSAGE_SIZE-1).unwrap();
    let res = check_message_size(&msg);
    assert!(res.is_err())
}

#[test]
fn sig_size_succ() {
    let sig = randombytes(SIGNATURE_SIZE).unwrap();
    let res = check_signature_size(&sig);
    assert!(res.is_ok())
}

#[test]
fn sig_size_fail() {
    let sig = randombytes(SIGNATURE_SIZE+1).unwrap();
    let res = check_signature_size(&sig);
    assert!(res.is_err())
}

#[test]
fn sign_succ() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let msg = hash(&data).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (_, sk) = generate_keypair_from_seed(&seed).unwrap();
    let sig = sign(&msg, &sk).unwrap();
    let res = check_signature_size(&sig);
    assert!(res.is_ok())
}

#[test]
fn sign_faulty_message_fail() {
    let msg = randombytes(MESSAGE_SIZE+1).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (_, sk) = generate_keypair_from_seed(&seed).unwrap();
    let res = sign(&msg, &sk);
    assert!(res.is_err())
}

#[test]
fn sign_faulty_sk_fail() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let msg = hash(&data).unwrap();
    let sk = randombytes(SECRET_KEY_SIZE-1).unwrap();
    let res = sign(&msg, &sk);
    assert!(res.is_err())
}

#[test]
fn sign_verify_succ() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let msg = hash(&data).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (pk, sk) = generate_keypair_from_seed(&seed).unwrap();
    let sig = sign(&msg, &sk).unwrap();
    let res = verify_signature(&sig, &msg, &pk).unwrap();
    assert!(res)
}

#[test]
fn sign_verify_wrong_sig_fail() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let msg = hash(&data).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (pk, sk) = generate_keypair_from_seed(&seed).unwrap();
    let mut sig = sign(&msg, &sk).unwrap();
    sig[0] = sig[0] % 2 + 1;
    let res = verify_signature(&sig, &msg, &pk).unwrap();
    assert!(!res)
}

#[test]
fn sign_verify_faulty_sig_fail() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let msg = hash(&data).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (pk, _) = generate_keypair_from_seed(&seed).unwrap();
    let sig = randombytes(SIGNATURE_SIZE+1).unwrap();
    let res = verify_signature(&sig, &msg, &pk);
    assert!(res.is_err())
}

#[test]
fn sign_verify_wrong_message_fail() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let mut msg = hash(&data).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (pk, sk) = generate_keypair_from_seed(&seed).unwrap();
    let sig = sign(&msg, &sk).unwrap();
    msg = hash(&msg).unwrap();
    let res = verify_signature(&sig, &msg, &pk).unwrap();
    assert!(!res)
}

#[test]
fn sign_verify_faulty_message_fail() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let mut msg = hash(&data).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (pk, sk) = generate_keypair_from_seed(&seed).unwrap();
    let sig = sign(&msg, &sk).unwrap();
    msg = randombytes(MESSAGE_SIZE-1).unwrap();
    let res = verify_signature(&sig, &msg, &pk);
    assert!(res.is_err())
}

#[test]
fn sign_verify_wrong_pk_fail() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let msg = hash(&data).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (mut pk, sk) = generate_keypair_from_seed(&seed).unwrap();
    let sig = sign(&msg, &sk).unwrap();
    pk[0] = pk[0] % 2 + 1;
    let res = verify_signature(&sig, &msg, &pk).unwrap();
    assert!(!res)
}

#[test]
fn sign_verify_faulty_pk_fail() {
    let len = 1000000; 
    let data = randombytes(len).unwrap();
    let msg = hash(&data).unwrap();
    let seed = randombytes(SEED_SIZE).unwrap();
    let (_, sk) = generate_keypair_from_seed(&seed).unwrap();
    let sig = sign(&msg, &sk).unwrap();
    let pk = randombytes(PUBLIC_KEY_SIZE+1).unwrap();
    let res = verify_signature(&sig, &msg, &pk);
    assert!(res.is_err())
}
