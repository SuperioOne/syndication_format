use core::time;

pub type Uri = String;

pub enum Text {
  Html(String),
  Xhtml(String),
  PlainText(String),
}

pub struct Date {
  unix_time: time::Duration,
}

pub enum LinkRelation {
  Alternate,
  Related,
  SelfRel,
  Enclosure,
  Via,
}

impl Default for Date {
  fn default() -> Self {
    Self {
      // 1970-01-01 00:00
      unix_time: time::Duration::new(0, 0),
    }
  }
}
