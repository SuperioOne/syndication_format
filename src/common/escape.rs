use crate::utils::byte_search::{BytePosition, ByteSearch};
use std::borrow::Cow;

pub static XML_ESCAPE_PATTERNS: &'static [u8; 5] = &[b'<', b'>', b'&', b'\'', b'"'];
pub static XML_ATTR_ESCAPE_PATTERNS: &'static [u8; 4] = &[b'<', b'>', b'&', b'"'];

static EMPTY: &'static str = "";
static ESCAPE_LOOKUP_TABLE: &'static [&'static str; 256] = &[
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, "&quot;", EMPTY, EMPTY, EMPTY, "&amp;", "&apos;",
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, "&lt;", EMPTY, "&gt;", EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
];

#[inline]
fn get_escaped(value: u8) -> &'static str {
  unsafe { *ESCAPE_LOOKUP_TABLE.get_unchecked(value as usize) }
}

pub fn escape<'a>(input: &'a str, byte_patterns: &'a [u8]) -> Cow<'a, str> {
  if input.is_empty() {
    return Cow::Borrowed(input);
  }

  let mut byte_search = input.search_bytes(byte_patterns);

  if let Some(BytePosition { index, value }) = byte_search.next() {
    let mut escaped_input = String::with_capacity(input.len());

    if index != 0 {
      escaped_input.push_str(&input[..index]);
    }

    let escaped = get_escaped(value);
    escaped_input.push_str(escaped);

    let mut head_pos = index + 1;

    for BytePosition { index, value } in byte_search {
      let escaped = get_escaped(value);

      if head_pos != index {
        escaped_input.push_str(&input[head_pos..index]);
      }

      escaped_input.push_str(escaped);
      head_pos = index + 1;
    }

    if head_pos < (input.len() - 1) {
      escaped_input.push_str(&input[head_pos..]);
    }

    Cow::Owned(escaped_input)
  } else {
    Cow::Borrowed(input)
  }
}

#[macro_export]
macro_rules! escape_xml {
  ($input:expr) => {
    $crate::common::escape::escape($input, $crate::common::escape::XML_ESCAPE_PATTERNS)
  };
}

#[macro_export]
macro_rules! escape_xml_attr {
  ($input:expr) => {
    $crate::common::escape::escape($input, $crate::common::escape::XML_ATTR_ESCAPE_PATTERNS)
  };
}

#[cfg(test)]
mod test {
  use super::get_escaped;

  #[test]
  fn escape_lookups() {
    let test_cases = vec![
      (b'"', "&quot;"),
      (b'&', "&amp;"),
      (b'\'', "&apos;"),
      (b'<', "&lt;"),
      (b'>', "&gt;"),
    ];

    for case in test_cases.iter() {
      assert_eq!(case.1, get_escaped(case.0));
    }
  }

  #[test]
  fn escape_special_chars() {
    let input = "<div> '\"COOL&CREATE\"' </div>";
    let escaped = escape_xml!(&input);

    match escaped {
      std::borrow::Cow::Borrowed(_) => {
        assert!(false, "It shouldn't returned borrowed text back.")
      }
      std::borrow::Cow::Owned(escaped_text) => assert_eq!(
        "&lt;div&gt; &apos;&quot;COOL&amp;CREATE&quot;&apos; &lt;/div&gt;",
        &escaped_text
      ),
    }
  }

  // Tests potential edge cases
  #[test]
  fn escape_once() {
    let start_input = "&Test";
    let escaped = escape_xml!(&start_input);

    match escaped {
      std::borrow::Cow::Borrowed(_) => {
        assert!(false, "It shouldn't returned borrowed text back.")
      }
      std::borrow::Cow::Owned(escaped_text) => assert_eq!("&amp;Test", &escaped_text),
    }

    let end_input = "Test&";
    let escaped = escape_xml!(&end_input);
    match escaped {
      std::borrow::Cow::Borrowed(_) => {
        assert!(false, "It shouldn't returned borrowed text back.")
      }
      std::borrow::Cow::Owned(escaped_text) => assert_eq!("Test&amp;", &escaped_text),
    }
  }

  #[test]
  fn escape_non_special_chars() {
    let input = "Cool and Create";
    let escaped = escape_xml!(&input);

    match escaped {
      std::borrow::Cow::Borrowed(escaped_text) => {
        assert_eq!("Cool and Create", escaped_text)
      }
      std::borrow::Cow::Owned(_) => assert!(
        false,
        "It shouldn't allocate new string. There is nothing to escape."
      ),
    }
  }
}
