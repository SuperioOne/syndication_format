pub(crate) fn is_valid_name(name: &str) -> bool {
  let mut chars_iter = name.chars();

  if let Some(first_char) = chars_iter.next() {
    if name_start_char(first_char) == false {
      return false;
    }

    for char in name.chars() {
      if name_char(char) == false {
        return false;
      }
    }

    return true;
  } else {
    false
  }
}

#[inline]
fn name_start_char<'a>(value: char) -> bool {
  match value {
    ':' => true,
    'A'..'Z' => true,
    '_' => true,
    'a'..'z' => true,
    '\u{C0}'..='\u{D6}' => true,
    '\u{D8}'..='\u{F6}' => true,
    '\u{F8}'..='\u{2FF}' => true,
    '\u{370}'..='\u{37D}' => true,
    '\u{37F}'..='\u{1FFF}' => true,
    '\u{200C}'..='\u{200D}' => true,
    '\u{2070}'..='\u{218F}' => true,
    '\u{2C00}'..='\u{2FEF}' => true,
    '\u{3001}'..='\u{D7FF}' => true,
    '\u{F900}'..='\u{FDCF}' => true,
    '\u{FDF0}'..='\u{FFFD}' => true,
    '\u{10000}'..='\u{EFFFF}' => true,
    _ => false,
  }
}

#[inline]
fn name_char<'a>(value: char) -> bool {
  match value {
    '.' => true,
    '-' => true,
    '0'..='9' => true,
    ':' => true,
    'A'..'Z' => true,
    '_' => true,
    'a'..'z' => true,
    '\u{C0}'..='\u{D6}' => true,
    '\u{D8}'..='\u{F6}' => true,
    '\u{F8}'..='\u{2FF}' => true,
    '\u{300}'..='\u{36F}' => true,
    '\u{370}'..='\u{37D}' => true,
    '\u{37F}'..='\u{1FFF}' => true,
    '\u{203F}'..='\u{2040}' => true,
    '\u{200C}'..='\u{200D}' => true,
    '\u{2070}'..='\u{218F}' => true,
    '\u{2C00}'..='\u{2FEF}' => true,
    '\u{3001}'..='\u{D7FF}' => true,
    '\u{F900}'..='\u{FDCF}' => true,
    '\u{FDF0}'..='\u{FFFD}' => true,
    '\u{10000}'..='\u{EFFFF}' => true,
    _ => false,
  }
}

#[cfg(test)]
mod test {
  use super::is_valid_name;

  #[test]
  fn validate_name() {
    assert!(is_valid_name("example"));
    assert!(is_valid_name("example-element-name"));
    assert!(!is_valid_name("123invalid"));
    assert!(!is_valid_name("-invalid"));
  }
}
