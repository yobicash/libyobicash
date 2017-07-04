use libyobicash::models::amount::*;

#[test]
fn add_amounts_succ() {
    let a = Amount::new(1);
    let b = Amount::new(2);
    let c = Amount::new(3);
    let c1 = a + b;
    assert_eq!(c, c1)
}

#[test]
fn sub_amounts_succ() {
    let a = Amount::new(3);
    let b = Amount::new(2);
    let c = Amount::new(1);
    let c1 = a - b;
    assert_eq!(c, c1)
}

#[test]
fn mul_amounts_succ() {
    let a = Amount::new(1);
    let b = Amount::new(2);
    let c = Amount::new(2);
    let c1 = a * b;
    assert_eq!(c, c1)
}

#[test]
fn div_amounts_succ() {
    let a = Amount::new(3);
    let b = Amount::new(2);
    let c = Amount::new(1);
    let c1 = a / b;
    assert_eq!(c, c1)
}

#[test]
fn rem_amounts_succ() {
    let a = Amount::new(3);
    let b = Amount::new(2);
    let c = Amount::new(1);
    let c1 = a % b;
    assert_eq!(c, c1)
}
