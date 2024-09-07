use std::borrow::Cow;

#[derive(Copy, Clone)]
pub enum EscapeNotation {
  /// Numeric reference style for escaping special characters.
  ///
  /// Examples:
  /// `"` => `&#34;`
  /// `&` => `&#38;`
  /// `'` => `&#39;`
  /// `<` => `&#60;`
  /// `>` => `&#62;`
  Numeric,

  /// Named reference style for escaping special characters.
  ///
  /// Examples:
  /// `"` => `&quot;`
  /// `&` => `&amp;`
  /// `'` => `&apos;`
  /// `<` => `&lt`
  /// `>` => `&gt;`
  Named,
}

impl Default for EscapeNotation {
  fn default() -> Self {
    Self::Named
  }
}

// Hash map for escape sequences
static ESCAPE_LOOKUP_TABLE: &'static [Option<&'static str>; 32] = &[
  Some("&#62;"),
  Some("&quot;"),
  None,
  None,
  Some("&#34;"),
  Some("&amp;"),
  Some("&apos;"),
  None,
  Some("&#38;"),
  Some("&#39;"),
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  Some("&lt;"),
  None,
  Some("&gt;"),
  Some("&#60;"),
  None,
];

#[inline]
fn get_escaped(value: u8, notation: EscapeNotation) -> Option<&'static str> {
  let position: usize = match notation {
    EscapeNotation::Numeric => (value + 2) & 0x1F,
    EscapeNotation::Named => (value - 1) & 0x1F,
  } as usize;

  // unwrap() is safe to use. `position` is always between 0-31.
  *ESCAPE_LOOKUP_TABLE.get(position).unwrap()
}

pub fn escape_str<'a>(text: &'a str, notation: EscapeNotation) -> Cow<'a, str> {
  // TODO: Implement a avx/swar search and replace
  Cow::Borrowed(text)
}

#[cfg(test)]
mod test {
  use super::{get_escaped, EscapeNotation};

  #[test]
  fn escape_lookups() {
    let test_cases = vec![
      (b'"', EscapeNotation::Named, "&quot;"),
      (b'"', EscapeNotation::Numeric, "&#34;"),
      (b'&', EscapeNotation::Named, "&amp;"),
      (b'&', EscapeNotation::Numeric, "&#38;"),
      (b'\'', EscapeNotation::Named, "&apos;"),
      (b'\'', EscapeNotation::Numeric, "&#39;"),
      (b'<', EscapeNotation::Named, "&lt;"),
      (b'<', EscapeNotation::Numeric, "&#60;"),
      (b'>', EscapeNotation::Named, "&gt;"),
      (b'>', EscapeNotation::Numeric, "&#62;"),
    ];

    for case in test_cases.iter() {
      assert_eq!(Some(case.2), get_escaped(case.0, case.1));
    }
  }
}
