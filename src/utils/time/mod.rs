use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use chrono::{Timelike, Datelike};
use byteorder::{ByteOrder, LittleEndian, BigEndian};
use errors::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct YTime(pub DateTime<Utc>);

impl Default for YTime {
  fn default() -> YTime {
    YTime::now()
  }
}

impl YTime {
  pub fn new(y: u64, mm: u64, d: u64, h: u64, m: u64, s: u64) -> YTime {
    let date_time = NaiveDate::from_ymd(y as i32, mm as u32, d as u32)
                              .and_hms(h as u32, m as u32, s as u32);
    YTime(DateTime::<Utc>::from_utc(date_time, Utc))
  }

  pub fn now() -> YTime {
    YTime(Utc::now())
  }

  pub fn to_timestamp(&self) -> u64 {
    self.0.timestamp() as u64 // cause it will never be < 1/1/1970
  }

  pub fn to_little_endian(&self) -> [u8; 8] {
    let mut buf = [0; 8];
    LittleEndian::write_u64(&mut buf, self.to_timestamp());
    buf
  }

  pub fn to_big_endian(&self) -> [u8; 8] {
    let mut buf = [0; 8];
    BigEndian::write_u64(&mut buf, self.to_timestamp());
    buf
  }

  pub fn to_bytes(&self) -> [u8; 8] {
    self.to_big_endian()
  }

  pub fn from_timestamp(ts: u64) -> YTime {
    let ndt = NaiveDateTime::from_timestamp(ts as i64, 0);
    YTime(DateTime::<Utc>::from_utc(ndt, Utc))
  }

  pub fn from_little_endian(b: &[u8]) -> YResult<YTime> {
    if b.len() != 8 {
      return Err(YErrorKind::InvalidLength(8, b.len()).into());
    }
    let t = YTime::from_timestamp(LittleEndian::read_u64(b));
    Ok(t)
  }

  pub fn from_big_endian(b: &[u8]) -> YResult<YTime> {
    if b.len() != 8 {
      return Err(YErrorKind::InvalidLength(8, b.len()).into());
    }
    let t = YTime::from_timestamp(BigEndian::read_u64(b));
    Ok(t)
  }

  pub fn from_bytes(b: &[u8]) -> YResult<YTime> {
    YTime::from_big_endian(b)
  }

  pub fn years(&self) -> u64 {
    self.0.year() as u64
  }

  pub fn months(&self) -> u64 {
    self.0.month() as u64
  }

  pub fn days(&self) -> u64 {
    self.0.day() as u64
  }

  pub fn hours(&self) -> u64 {
    self.0.hour() as u64
  }

  pub fn mins(&self) -> u64 {
    self.0.minute() as u64
  }

  pub fn secs(&self) -> u64 {
    self.0.second() as u64
  }
}
