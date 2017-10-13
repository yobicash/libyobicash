use rand::random;
use libyobicash::crypto::hash::{YHash32, YHash64, YDigest32, YDigest64};

#[test]
fn digest32_from_bytes_succ() {
    let mut b = [0u8; 32];
    for i in 0..32 {
        b[i] = random::<u8>();
    }
    let res = YDigest32::from_bytes(&b[..]);
    assert!(res.is_ok())
}

#[test]
fn digest32_from_bytes_fail() {
    let mut b = [0u8; 64];
    for i in 0..64 {
        b[i] = random::<u8>();
    }
    let res = YDigest32::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn digest32_from_hex_succ() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b796974131";
    let res = YDigest32::from_hex(s);
    assert!(res.is_ok())
}

#[test]
fn digest32_from_hex_fail() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b79697413";
    let res = YDigest32::from_hex(s);
    assert!(res.is_err())
}

fn test_vectors_32() -> Vec<(String, String)> {
    vec![
    ("abc".to_string(),
    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".to_string()),
    ("".to_string(),
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()),
    ("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_string(),
    "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1".to_string()),
    ("abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu".to_string(),
    "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1".to_string()),
  ]
}

#[test]
fn hash32_test_vectors_succ() {
    for v in test_vectors_32() {
        let res = YHash32::hash(v.0.into_bytes().as_slice());
        let test = YDigest32::from_hex(v.1.as_str()).unwrap();
        assert_eq!(res, test)
    }
}

#[test]
fn digest64_from_bytes_succ() {
    let mut b = [0u8; 64];
    for i in 0..64 {
        b[i] = random::<u8>();
    }
    let res = YDigest64::from_bytes(&b[..]);
    assert!(res.is_ok())
}

#[test]
fn digest64_from_bytes_fail() {
    let mut b = [0u8; 32];
    for i in 0..32 {
        b[i] = random::<u8>();
    }
    let res = YDigest64::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn digest64_from_hex_succ() {
    let s = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
    let res = YDigest64::from_hex(s);
    assert!(res.is_ok())
}

#[test]
fn digest64_from_hex_fail() {
    let s = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3ef";
    let res = YDigest64::from_hex(s);
    assert!(res.is_err())
}

fn test_vectors_64() -> Vec<(String, String)> {
    vec![
    ("abc".to_string(),
    "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f".to_string()),
    ("".to_string(),
    "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e".to_string()),
    ("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".to_string(),
    "204a8fc6dda82f0a0ced7beb8e08a41657c16ef468b228a8279be331a703c33596fd15c13b1b07f9aa1d3bea57789ca031ad85c7a71dd70354ec631238ca3445".to_string()),
    ("abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu".to_string(),
    "8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909".to_string()),
  ]
}

#[test]
fn hash64_test_vectors_succ() {
    for v in test_vectors_64() {
        let res = YHash64::hash(v.0.into_bytes().as_slice());
        let test = YDigest64::from_hex(v.1.as_str()).unwrap();
        assert_eq!(res, test)
    }
}
