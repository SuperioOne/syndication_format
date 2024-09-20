use core::{
  fmt::{Display, Write},
  time,
};

/// Division helper that returns both quotient and remainder.
macro_rules! divide {
  ($dividend:expr, $divisor:expr) => {{
    let quotient = $dividend / $divisor;
    let remainder = $dividend % $divisor;

    (quotient, remainder)
  }};
}

const DAYS_IN_MONTHS: &[u64] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const TZ_UNKNOWN: TimeStampOffset = TimeStampOffset {
  hours: 0,
  minutes: 0,
  symbol: OffsetSymbol::Minus,
};

const TZ_Z: TimeStampOffset = TimeStampOffset {
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
pub struct TimeStampOffset {
  hours: u8,
  minutes: u8,
  symbol: OffsetSymbol,
}

impl Display for TimeStampOffset {
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

impl Default for TimeStampOffset {
  fn default() -> Self {
    TZ_Z
  }
}

impl TimeStampOffset {
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
  offset: TimeStampOffset,
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
      offset: TZ_Z,
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
      self.offset
    ))
  }
}

impl TimeStamp {
  pub fn new(unix_epoch: time::Duration) -> Self {
    Self {
      unix_time: unix_epoch,
      offset: TZ_Z,
    }
  }

  pub fn set_offset(&mut self, offset: TimeStampOffset) {
    self.offset = offset;
  }

  #[inline]
  // Converts unix timestamp to 'biblically accurate' date time struct.
  fn as_datetime(&self) -> DateTime {
    const HOUR_IN_SECS: u64 = 60 * 60;
    const DAY_IN_SECS: u64 = HOUR_IN_SECS * 24;
    const NON_LEAP_YEAR: u64 = 365 * DAY_IN_SECS;
    const LEAP_YEAR_BLOCK: u64 = NON_LEAP_YEAR * 4 + DAY_IN_SECS;

    let mut dt = DateTime {
      day: 1,
      year: 1970,
      hour: 0,
      month: 1,
      seconds: 0,
      minute: 0,
    };

    let tz_offset: u64 = ((self.offset.hours as u64) * 3600) + ((self.offset.minutes as u64) * 60);
    let duration_secs: u64 = match self.offset.symbol {
      OffsetSymbol::Minus => self.unix_time.as_secs().saturating_sub(tz_offset),
      OffsetSymbol::Plus => self.unix_time.as_secs().saturating_add(tz_offset),
    };

    let (year_blocks, remainder) = divide!(duration_secs, LEAP_YEAR_BLOCK);
    let (trailing_years, mut remainder) = divide!(remainder, NON_LEAP_YEAR);

    dt.year = 1970 + (year_blocks * 4) + trailing_years;
    let is_leap_year = (dt.year & 0x03) > 0 || (dt.year % 100 != 0 && dt.year % 400 != 0);

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
