use core::fmt::Display;
use core::time;
use std::borrow::Cow;

use self::escape::escape_str;

pub mod escape;

pub type Uri = String;

pub enum XmlText<'a> {
  Html(&'a str),
  Xhtml(&'a str),
  PlainText(&'a str),
}

pub struct DateTime {
  unix_time: time::Duration,
}

#[derive(Copy, Clone, Debug)]
pub enum LinkRelation {
  Alternate,
  Related,
  SelfRel,
  Enclosure,
  Via,
}

impl Default for DateTime {
  fn default() -> Self {
    Self {
      // 1970-01-01 00:00
      unix_time: time::Duration::new(0, 0),
    }
  }
}

impl Display for LinkRelation {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl LinkRelation {
  pub fn as_str(&self) -> &'static str {
    match self {
      LinkRelation::Alternate => "alternate",
      LinkRelation::Related => "related",
      LinkRelation::SelfRel => "self",
      LinkRelation::Enclosure => "enclosure",
      LinkRelation::Via => "via",
    }
  }
}

impl XmlText<'_> {
  pub fn as_normalized_str(&self) -> Cow<'_, str> {
    // TODO: escape all special characters here
    match self {
      XmlText::Html(value) => escape_str(value),
      XmlText::Xhtml(value) => Cow::Borrowed(value),
      XmlText::PlainText(value) => escape_str(value),
    }
  }
}

impl DateTime {
  pub fn as_str(&self) -> &str {
    // TODO:: There are no implemented datetime structure for now.
    "EMPTY"
  }
}
