use atom_syndication_format::common::{AttributeMap, AttributeName, AttributeValue};
use atom_syndication_format::error::InvalidAttributeName;

#[test]
fn attribute_map_create() {
  let mut attr_src = AttributeMap::new();
  attr_src.set(AttributeName::new("lang").unwrap(), "unknown".into());
  attr_src.set(AttributeName::new("attr1").unwrap(), "false".into());
  attr_src.set(AttributeName::new("attr2").unwrap(), "42".into());
  attr_src.set(AttributeName::new("attr3").unwrap(), "Hello".into());

  let mut attr_final = AttributeMap::new_from(&attr_src);
  attr_final.set(AttributeName::new("lang").unwrap(), "en-US".into());
  attr_final.set(AttributeName::new("attr4").unwrap(), "New".into());

  assert_eq!(5, attr_final.len());
  assert_eq!(Some("en-US"), attr_final.get("lang").map(|v| v.as_str()));
  assert_eq!(Some("false"), attr_final.get("attr1").map(|v| v.as_str()));
  assert_eq!(Some("42"), attr_final.get("attr2").map(|v| v.as_str()));
  assert_eq!(Some("Hello"), attr_final.get("attr3").map(|v| v.as_str()));
  assert_eq!(Some("New"), attr_final.get("attr4").map(|v| v.as_str()));
}

#[test]
fn attribute_names() {
  assert!(AttributeName::new("-DashAtStart").is_err());
  assert!(AttributeName::new("\"DoubleQuoted\"").is_err());
  assert!(AttributeName::new("2NumberAtStart").is_err());
  assert!(AttributeName::new("whitespace   name").is_err());
  assert!(AttributeName::new("invalid-unicode-\u{FDE1}-0").is_err());
  assert!(AttributeName::new("invalid-unicode-\u{00D7}-1").is_err());
  assert!(AttributeName::new("'(nope)'").is_err());

  assert!(AttributeName::new("correct-name-123_567").is_ok());
  assert!(AttributeName::new("div").is_ok());
  assert!(AttributeName::new("a").is_ok());
  assert!(AttributeName::new("me.com").is_ok());
  assert!(AttributeName::new("valid-unicode-かちかち山").is_ok());
}

#[test]
fn attribute_map_has() {
  let mut attr_map = AttributeMap::new();
  attr_map.set(AttributeName::new("lang").unwrap(), "unknown".into());
  attr_map.set(AttributeName::new("attr1").unwrap(), "false".into());

  assert_eq!(true, attr_map.has("lang"));
  assert_eq!(true, attr_map.has("attr1"));
  assert_eq!(false, attr_map.has("attr2"));
}
