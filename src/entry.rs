use crate::common::{Attributes, ExtensionXmlNode};
use crate::metadata::{CategoryNode, DateNode, LinkNode, PersonNode, RawTextNode, TextNode};

pub struct EntryNode {
    pub attributes: Attributes,
    pub authors: Vec<PersonNode>,
    pub categories: Vec<CategoryNode>,
    pub content: Option<TextNode>,
    pub contributors: Vec<PersonNode>,
    pub extensions: Vec<ExtensionXmlNode>,
    pub id: RawTextNode,
    pub links: Vec<LinkNode>,
    pub namespace: Option<String>,
    pub rights: Option<TextNode>,
    pub summary: Option<TextNode>,
    pub title: TextNode,
    pub updated: DateNode,
}
