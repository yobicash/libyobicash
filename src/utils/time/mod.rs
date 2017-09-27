use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use chrono::{Timelike, Datelike};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct YTime(pub DateTime<Utc>);

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

  pub fn from_timestamp(ts: u64) -> YTime {
    let ndt = NaiveDateTime::from_timestamp(ts as i64, 0);
    YTime(DateTime::<Utc>::from_utc(ndt, Utc))
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
