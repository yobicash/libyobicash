use rand::random;
use libyobicash::utils::time::YTime;

#[test]
fn time_now_succ() {
    let now = YTime::now();
    let later = YTime::now();
    assert!(now <= later)
}

#[test]
fn time_parts_succ() {
    let years = random::<u64>() % 3000;
    let months = random::<u64>() % 11 + 1;
    let days = random::<u64>() % 27 + 1;
    let hours = random::<u64>() % 24;
    let minutes = random::<u64>() % 60;
    let seconds = random::<u64>() % 60;
    let t = YTime::new(years, months, days, hours, minutes, seconds);
    assert_eq!(t.years(), years);
    assert_eq!(t.months(), months);
    assert_eq!(t.days(), days);
    assert_eq!(t.hours(), hours);
    assert_eq!(t.minutes(), minutes);
    assert_eq!(t.seconds(), seconds)
}

#[test]
fn time_to_timestamp_succ() {
    let timestamp: u64 = random::<u32>() as u64;
    let t = YTime::from_timestamp(timestamp);
    assert_eq!(t.to_timestamp(), timestamp)
}

#[test]
fn time_little_endian_succ() {
    let t_a = YTime::now();
    let le_t = t_a.to_little_endian();
    let t_b = YTime::from_little_endian(&le_t[..]).unwrap();
    assert_eq!(t_a, t_b)
}

#[test]
fn time_big_endian_succ() {
    let t_a = YTime::now();
    let be_t = t_a.to_big_endian();
    let t_b = YTime::from_big_endian(&be_t[..]).unwrap();
    assert_eq!(t_a, t_b)
}

#[test]
fn time_bytes_succ() {
    let t_a = YTime::now();
    let b_t = t_a.to_bytes();
    let t_b = YTime::from_bytes(&b_t[..]).unwrap();
    assert_eq!(t_a, t_b)
}
