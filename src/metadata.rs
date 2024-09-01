use crate::common::{Attributes, Date, ExtensionXmlNode, LinkRelation, Text, Uri};

pub struct PersonNode {
    pub attributes: Attributes,
    pub email: Option<String>,
    pub extensions: Vec<ExtensionXmlNode>,
    pub name: String,
    pub uri: Option<String>,
}

pub struct CategoryNode {
    pub attributes: Attributes,
    pub label: Option<String>,
    pub scheme: Option<String>,
    pub term: String,
}

pub struct GeneratorNode {
    pub attributes: Attributes,
    pub uri: Option<String>,
    pub value: String,
    pub version: Option<String>,
}

pub struct UriNode {
    pub attributes: Attributes,
    pub value: Uri,
}

pub struct RawTextNode {
    pub attributes: Attributes,
    pub value: String,
}

pub struct LinkNode {
    pub attributes: Attributes,
    pub href: String,
    pub hreflang: Option<String>,
    pub length: Option<usize>,
    pub r#type: Option<String>,
    pub rel: Option<LinkRelation>,
    pub title: Option<String>,
}

pub struct TextNode {
    pub attributes: Attributes,
    pub value: Text,
}

pub struct DateNode {
    pub attributes: Attributes,
    pub value: Date,
}
