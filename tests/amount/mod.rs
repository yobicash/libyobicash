use libyobicash::amount::*;

#[test]
fn add_amounts_succ() {
    let a = YAmount::new(1);
    let b = YAmount::new(2);
    let c = YAmount::new(3);
    let c1 = a + b;
    assert_eq!(c, c1)
}

#[test]
fn sub_amounts_succ() {
    let a = YAmount::new(3);
    let b = YAmount::new(2);
    let c = YAmount::new(1);
    let c1 = a - b;
    assert_eq!(c, c1)
}

#[test]
fn mul_amounts_succ() {
    let a = YAmount::new(1);
    let b = YAmount::new(2);
    let c = YAmount::new(2);
    let c1 = a * b;
    assert_eq!(c, c1)
}

#[test]
fn div_amounts_succ() {
    let a = YAmount::new(3);
    let b = YAmount::new(2);
    let c = YAmount::new(1);
    let c1 = a / b;
    assert_eq!(c, c1)
}

#[test]
fn rem_amounts_succ() {
    let a = YAmount::new(3);
    let b = YAmount::new(2);
    let c = YAmount::new(1);
    let c1 = a % b;
    assert_eq!(c, c1)
}
