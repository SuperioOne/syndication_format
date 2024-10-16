use core::{
  fmt::{Display, Write},
  str::FromStr,
  time::{self, Duration},
};

use crate::error::TimeStampFormatErrors;

/// Division helper that returns both quotient and remainder.
macro_rules! divide {
  ($dividend:expr, $divisor:expr) => {{
    let quotient = $dividend / $divisor;
    let remainder = $dividend % $divisor;

    (quotient, remainder)
  }};
}

macro_rules! is_leap_year {
  ($year:expr) => {
    !(($year & 0x03) > 0 && ($year % 100 != 0 && $year % 400 != 0))
  };
}

const HOUR_IN_SECS: u64 = 60 * 60;
const DAY_IN_SECS: u64 = HOUR_IN_SECS * 24;
const NON_LEAP_YEAR: u64 = 365 * DAY_IN_SECS;
const LEAP_YEAR_BLOCK: u64 = NON_LEAP_YEAR * 4 + DAY_IN_SECS;
const DAYS_IN_MONTHS: &[u64] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const TZ_UNKNOWN: TimeZoneOffset = TimeZoneOffset {
  hours: 0,
  minutes: 0,
  symbol: OffsetSymbol::Minus,
};

const TZ_Z: TimeZoneOffset = TimeZoneOffset {
  hours: 0,
  minutes: 0,
  symbol: OffsetSymbol::Plus,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OffsetSymbol {
  Minus,
  Plus,
}

impl Display for OffsetSymbol {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      OffsetSymbol::Minus => f.write_char('-'),
      OffsetSymbol::Plus => f.write_char('+'),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TimeZoneOffset {
  hours: u8,
  minutes: u8,
  symbol: OffsetSymbol,
}

impl Display for TimeZoneOffset {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if *self == TZ_Z {
      f.write_char('Z')
    } else {
      f.write_fmt(format_args!(
        "{}{:0>2}:{:0>2}",
        self.symbol, self.hours, self.minutes
      ))
    }
  }
}

impl Default for TimeZoneOffset {
  fn default() -> Self {
    TZ_Z
  }
}

impl TimeZoneOffset {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_hours(&mut self, hours: u8) -> &mut Self {
    self.hours = hours;
    self
  }

  pub fn set_minutes(&mut self, minutes: u8) -> &mut Self {
    self.minutes = minutes;
    self
  }

  pub fn set_symbol(&mut self, symbol: OffsetSymbol) -> &mut Self {
    self.symbol = symbol;
    self
  }
}

/// Basic internet timestamp implementation based on [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339).
#[derive(Copy, Clone)]
pub struct TimeStamp {
  unix_time: time::Duration,
  time_zone: TimeZoneOffset,
}

// Helper struct. It's inlined and struct size doesn't matter for now.
#[derive(Debug)]
struct DateTime {
  year: u64,
  day: u64,
  month: u64,
  hour: u64,
  seconds: u64,
  minute: u64,
}

impl Default for TimeStamp {
  fn default() -> Self {
    Self {
      unix_time: time::Duration::new(0, 0),
      time_zone: TZ_Z,
    }
  }
}

impl Display for TimeStamp {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let date_time = self.as_datetime();

    f.write_fmt(format_args!(
      "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}{}",
      date_time.year,
      date_time.month,
      date_time.day,
      date_time.hour,
      date_time.minute,
      date_time.seconds,
      self.time_zone
    ))
  }
}

impl TimeStamp {
  pub fn new(unix_epoch: time::Duration) -> Self {
    Self {
      unix_time: unix_epoch,
      time_zone: TZ_Z,
    }
  }

  pub fn set_tz(&mut self, time_zone: TimeZoneOffset) {
    self.time_zone = time_zone;
  }

  #[inline]
  // Converts unix timestamp to 'biblically accurate' date time struct.
  fn as_datetime(&self) -> DateTime {
    let mut dt = DateTime {
      day: 1,
      year: 1970,
      hour: 0,
      month: 1,
      seconds: 0,
      minute: 0,
    };

    let tz_offset: u64 =
      ((self.time_zone.hours as u64) * 3600) + ((self.time_zone.minutes as u64) * 60);
    let duration_secs: u64 = match self.time_zone.symbol {
      OffsetSymbol::Minus => self.unix_time.as_secs().saturating_sub(tz_offset),
      OffsetSymbol::Plus => self.unix_time.as_secs().saturating_add(tz_offset),
    };

    let (year_blocks, remainder) = divide!(duration_secs, LEAP_YEAR_BLOCK);
    let (trailing_years, mut remainder) = divide!(remainder, NON_LEAP_YEAR);

    dt.year = 1970 + (year_blocks * 4) + trailing_years;
    let is_leap_year = is_leap_year!(dt.year);

    for (idx, day_count) in DAYS_IN_MONTHS.iter().enumerate() {
      let dc = (idx == 1 && is_leap_year) as u64 + day_count;

      match remainder.checked_sub(dc * DAY_IN_SECS) {
        Some(remaining) => {
          dt.month += 1;
          remainder = remaining;
        }
        None => break,
      }
    }

    let (days, remainder) = divide!(remainder, DAY_IN_SECS);
    dt.day = days + 1; // There are no 0th day. Increments the day by one for formatting.

    let (hours, remainder) = divide!(remainder, HOUR_IN_SECS);
    dt.hour = hours;

    let (minutes, remainder) = divide!(remainder, 60);
    dt.minute = minutes;
    dt.seconds = remainder;

    dt
  }
}

// FIX:
// - [ ] Missing number character check.
// - [ ] seconds fraction support (TBH, no one use that feature)
// - [ ] It does not support edge cases like 1969-12-31T23:00:00-01:00
// - [ ] It only works on little-endian systems (Big-endian systems are also very rare)
//
impl FromStr for TimeStamp {
  type Err = TimeStampFormatErrors;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    const TIME_STR_PATTERN_MASK: u64 = 0xFF_0000_FF_0000;
    const TIME_STR_PATTERN: u64 = 0x3A_0000_3A_0000;
    const TIME_STR_VALUE: u64 = 0x3030_00_3030_00_3030;
    const DATE_STR_PATTERN_MASK: u128 = 0xFF_0000_FF_00000000;
    const DATE_STR_VALUE: u128 = 0x3030_00_3030_00_30303030;
    const DATE_STR_PATTERN: u128 = 0x2D_0000_2D_00000000;

    let bytes = s.as_bytes();

    if bytes.len() > 27 && bytes.len() < 20 {
      return Err(TimeStampFormatErrors::InvalidFormat);
    }

    if bytes[10] != b'T' {
      return Err(TimeStampFormatErrors::InvalidFormat);
    }

    let date_part: u128 = unsafe { bytes.as_ptr().cast::<u128>().read() };
    if date_part & DATE_STR_PATTERN_MASK != DATE_STR_PATTERN {
      return Err(TimeStampFormatErrors::InvalidFormat);
    }

    let date: [u8; 16] = ((date_part & !DATE_STR_PATTERN_MASK) ^ DATE_STR_VALUE).to_le_bytes();

    let year: u64 =
      (date[0] as u64) * 1000 + (date[1] as u64) * 100 + (date[2] as u64) * 10 + (date[3] as u64);
    let month = date[5] * 10 + date[6];
    let day = date[8] * 10 + date[9];

    if year < 1970 {
      return Err(TimeStampFormatErrors::UnsupportedYear);
    }

    if month > 12 && month < 1 {
      return Err(TimeStampFormatErrors::InvalidDate);
    }

    let is_leap_year = is_leap_year!(year);

    let max_day = {
      if month == 2 && is_leap_year {
        DAYS_IN_MONTHS[1] + 1
      } else {
        DAYS_IN_MONTHS[(month - 1) as usize]
      }
    } as u8;

    if max_day < day {
      return Err(TimeStampFormatErrors::InvalidDate);
    }

    // Read 8-byte time string as u64 (excludes time fraction and offset)
    let time_str = &bytes[11..=18];
    let time_part: u64 = unsafe { time_str.as_ptr().cast::<u64>().read() };

    if time_part & TIME_STR_PATTERN_MASK != TIME_STR_PATTERN {
      return Err(TimeStampFormatErrors::InvalidFormat);
    }

    let time: [u8; 8] = ((time_part & !TIME_STR_PATTERN_MASK) ^ TIME_STR_VALUE).to_le_bytes();
    let hours = time[0] * 10 + time[1];
    let minutes = time[3] * 10 + time[4];
    let seconds = time[6] * 10 + time[7];

    if hours > 23 || minutes > 59 || seconds > 59 {
      return Err(TimeStampFormatErrors::InvalidTime);
    }

    let tz_part = &bytes[19..];

    let tz = match tz_part {
      [b'Z'] => TZ_Z,
      [b'-', b'0', b'0', b':', b'0', b'0'] => TZ_UNKNOWN,
      [b'-', h0, h1, b':', m0, m1] => TimeZoneOffset {
        symbol: OffsetSymbol::Minus,
        minutes: (m0 - 0x30) * 10 + (m1 - 0x30),
        hours: (h0 - 0x30) * 10 + (h1 - 0x30),
      },
      [b'+', h0, h1, b':', m0, m1] => TimeZoneOffset {
        symbol: OffsetSymbol::Plus,
        minutes: (m0 - 0x30) * 10 + (m1 - 0x30),
        hours: (h0 - 0x30) * 10 + (h1 - 0x30),
      },
      _ => return Err(TimeStampFormatErrors::InvalidTimeOffset),
    };

    let mut durat_secs: u64 = (seconds as u64)
      + ((minutes as u64) * 60)
      + ((hours as u64) * HOUR_IN_SECS)
      + (((day - 1) as u64) * DAY_IN_SECS);

    for idx in 0..(month - 1) {
      durat_secs +=
        (DAYS_IN_MONTHS[idx as usize] + (is_leap_year && idx == 1) as u64) * DAY_IN_SECS;
    }

    let (leap_year_blocks, remaining_years) = divide!(year - 1970, 4);
    durat_secs += LEAP_YEAR_BLOCK * leap_year_blocks;
    durat_secs += NON_LEAP_YEAR * remaining_years;
    let offset_secs = ((tz.hours as u64) * 3600) + (tz.minutes as u64) * 60;

    durat_secs = match tz.symbol {
      OffsetSymbol::Minus => durat_secs + offset_secs,
      OffsetSymbol::Plus => durat_secs - offset_secs,
    };

    Ok(Self {
      time_zone: tz,
      unix_time: Duration::from_secs(durat_secs),
    })
  }
}

#[cfg(test)]
mod test {
  use crate::common::timestamp;

  use super::TimeStamp;
  use core::time::Duration;

  #[test]
  fn ts_parse() {
    let mut timestamp = TimeStamp::new(Duration::from_secs(1726702105));
    timestamp.set_tz(timestamp::TimeZoneOffset {
      hours: 3,
      minutes: 0,
      symbol: timestamp::OffsetSymbol::Plus,
    });
    let text = timestamp.to_string();

    println!("first {}", text);

    let ts: TimeStamp = text.parse().unwrap();

    println!("second as u64 {}", ts.unix_time.as_secs());
    println!("second as str {}", ts.to_string());

    let mut timestamp2 = TimeStamp::new(ts.unix_time);

    println!("final {}", timestamp2.to_string());
    timestamp2.set_tz(timestamp::TimeZoneOffset {
      hours: 5,
      minutes: 0,
      symbol: timestamp::OffsetSymbol::Minus,
    });
    println!("final {}", timestamp2.to_string());
    assert_eq!(1726702105, ts.unix_time.as_secs());
  }
}
