use crate::common::{Attributes, ExtensionXmlNode};
use crate::entry::EntryNode;
use crate::metadata::{
  CategoryNode, DateNode, GeneratorNode, LinkNode, PersonNode, RawTextNode, TextNode, UriNode,
};

pub struct FeedNode {
  pub attributes: Attributes,
  pub authors: Vec<PersonNode>,
  pub categories: Vec<CategoryNode>,
  pub contributors: Vec<PersonNode>,
  pub entries: Vec<EntryNode>,
  pub extensions: Vec<ExtensionXmlNode>,
  pub generator: Option<GeneratorNode>,
  pub icon: Option<UriNode>,
  pub id: RawTextNode,
  pub links: Vec<LinkNode>,
  pub logo: Option<UriNode>,
  pub namespace: Option<String>,
  pub rights: Option<TextNode>,
  pub subtitle: Option<TextNode>,
  pub title: TextNode,
  pub updated: DateNode,
}
