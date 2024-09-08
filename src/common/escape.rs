use std::borrow::Cow;

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
  *ESCAPE_LOOKUP_TABLE.get(value as usize).unwrap()
}

pub fn escape_str<'a>(text: &'a str) -> Cow<'a, str> {
  for (idx, byte) in text.as_bytes().iter().enumerate() {
    if get_escaped(*byte) != EMPTY {
      let mut escaped_text = String::with_capacity(text.len());
      escaped_text.push_str(&text[..idx]);

      return Cow::Owned(escape_string(&text[idx..], escaped_text));
    }
  }

  Cow::Borrowed(text)
}

#[inline]
fn escape_string<'a>(src: &'a str, mut dest: String) -> String {
  let mut last = 0;

  for (idx, byte) in src.as_bytes().iter().enumerate() {
    let escaped = get_escaped(*byte);

    if escaped != EMPTY {
      if last != idx {
        dest.push_str(&src[last..idx]);
      }

      dest.push_str(escaped);
      last = idx + 1;
    }
  }

  if last < (src.len() - 1) {
    dest.push_str(&src[last..]);
  }

  dest
}

#[cfg(test)]
mod test {
  use super::{escape_str, get_escaped};

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
    let escaped = escape_str(&input);

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
    let escaped = escape_str(&start_input);

    match escaped {
      std::borrow::Cow::Borrowed(_) => {
        assert!(false, "It shouldn't returned borrowed text back.")
      }
      std::borrow::Cow::Owned(escaped_text) => assert_eq!("&amp;Test", &escaped_text),
    }

    let end_input = "Test&";
    let escaped = escape_str(&end_input);
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
    let escaped = escape_str(&input);

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
