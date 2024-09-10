use crate::utils::byte_search::{swar_search::SwarByteSearchIter, BasicByteSearch, BytePosition};
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

#[inline]
fn internal_escape<'a, S>(mut byte_search_iter: S, input: &'a str) -> Cow<'a, str>
where
  S: Iterator<Item = BytePosition> + Sized,
{
  if input.is_empty() {
    return Cow::Borrowed(input);
  }

  if let Some(BytePosition { index, value }) = byte_search_iter.next() {
    let mut escaped_input = String::with_capacity(input.len());

    if index != 0 {
      escaped_input.push_str(&input[..index]);
    }

    let escaped_char = get_escaped(value);
    escaped_input.push_str(escaped_char);

    let mut head_pos = index + 1;

    for BytePosition { index, value } in byte_search_iter {
      let escaped_char = get_escaped(value);

      if head_pos != index {
        escaped_input.push_str(&input[head_pos..index]);
      }

      escaped_input.push_str(escaped_char);
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

pub fn escape<'a>(input: &'a str, search_bytes: &'a [u8]) -> Cow<'a, str> {
  #[cfg(not(any(
    target_feature = "avx2",
    all(target_feature = "avx512f", feature = "avx512")
  )))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      _ => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
    }
  }

  //
  // x86-64 with AVX2
  //

  #[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx2",
    not(all(feature = "avx512", target_feature = "avx512f"))
  ))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      8..=63 => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      _ => {
        let iter = crate::utils::byte_search::avx2_search::Avx2ByteSearchIter::new(
          input.as_bytes(),
          search_bytes,
        );
        internal_escape(iter, input)
      }
    }
  }

  //
  // x86-64 with AVX512 cpu and avx512 feature enabled
  //

  #[cfg(all(feature = "avx512", target_arch = "x86_64", target_feature = "avx512f"))]
  {
    match input.len() {
      0..=7 => {
        let iter = BasicByteSearch::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      8..=63 => {
        let iter = SwarByteSearchIter::new(input.as_bytes(), search_bytes);
        internal_escape(iter, input)
      }
      _ => {
        let iter = crate::utils::byte_search::avx512_search::Avx512ByteSearchIter::new(
          input.as_bytes(),
          search_bytes,
        );
        internal_escape(iter, input)
      }
    }
  }
}

#[macro_export]
macro_rules! escape_xml {
  ($input:expr) => {
    $crate::escape::escape($input, $crate::escape::XML_ESCAPE_PATTERNS)
  };
}

#[macro_export]
macro_rules! escape_xml_attr {
  ($input:expr) => {
    $crate::escape::escape($input, $crate::escape::XML_ATTR_ESCAPE_PATTERNS)
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
