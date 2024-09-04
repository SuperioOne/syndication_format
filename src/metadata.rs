use crate::attributes::AttributeMap;
use crate::common::{Date, LinkRelation, Text, Uri};

pub struct PersonNode {
  pub attributes: AttributeMap,
  pub email: Option<String>,
  pub name: String,
  pub uri: Option<String>,
}

pub struct CategoryNode {
  pub attributes: AttributeMap,
  pub label: Option<String>,
  pub scheme: Option<String>,
  pub term: String,
}

pub struct GeneratorNode {
  pub attributes: AttributeMap,
  pub uri: Option<String>,
  pub value: String,
  pub version: Option<String>,
}

pub struct UriNode {
  pub attributes: AttributeMap,
  pub value: Uri,
}

pub struct RawTextNode {
  pub attributes: AttributeMap,
  pub value: String,
}

pub struct LinkNode {
  pub attributes: AttributeMap,
  pub href: String,
  pub hreflang: Option<String>,
  pub length: Option<usize>,
  pub r#type: Option<String>,
  pub rel: Option<LinkRelation>,
  pub title: Option<String>,
}

pub struct TextNode {
  pub attributes: AttributeMap,
  pub value: Text,
}

pub struct DateNode {
  pub attributes: AttributeMap,
  pub value: Date,
}
