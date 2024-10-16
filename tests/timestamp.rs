use atom_syndication_format::common::{OffsetSymbol, TimeStamp, TimeZoneOffset};
use core::time::Duration;

#[test]
fn basic_timestamp() {
  let timestamp = TimeStamp::new(Duration::from_millis(1726702105542));
  assert_eq!("2024-09-18T23:28:25Z", &timestamp.to_string())
}

#[test]
fn basic_timestamp_with_utc_plus_03() {
  let mut timestamp = TimeStamp::new(Duration::from_millis(1726702105542));
  let mut offset = TimeZoneOffset::new();
  offset
    .set_hours(3)
    .set_minutes(0)
    .set_symbol(OffsetSymbol::Plus);

  timestamp.set_tz(offset);

  assert_eq!("2024-09-19T02:28:25+03:00", &timestamp.to_string())
}

#[test]
fn basic_timestamp_with_utc_minus_03() {
  let mut timestamp = TimeStamp::new(Duration::from_millis(1726702105542));
  let mut offset = TimeZoneOffset::new();
  offset
    .set_hours(3)
    .set_minutes(0)
    .set_symbol(OffsetSymbol::Minus);

  timestamp.set_tz(offset);

  assert_eq!("2024-09-18T20:28:25-03:00", &timestamp.to_string())
}
