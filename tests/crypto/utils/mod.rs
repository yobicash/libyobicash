use libyobicash::crypto::utils::randombytes;
use libyobicash::crypto::utils::check_binary_size;

#[test]
fn randombytes_succ() {
    let len: usize = 10;
    let bin = randombytes(len).unwrap();
    assert_eq!(bin.len(), len)
}

#[test]
fn binary_size_succ() {
    let len: usize = 10;
    let bin = randombytes(len).unwrap();
    let res = check_binary_size(bin.as_slice(), len as u32);
    assert!(res.is_ok())
}

#[test]
fn binary_size_fail() {
    let len: usize = 10;
    let bin = randombytes(len).unwrap();
    let res = check_binary_size(bin.as_slice(), (len+1) as u32);
    assert!(res.is_err())
}
