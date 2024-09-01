use core::time;

pub type Uri = String;

pub struct Attribute(String, String);

pub struct Attributes {
    inner: Vec<Attribute>,
}

pub enum Text {
    Html(String),
    Xhtml(String),
    PlainText(String),
}

pub struct Date {
    unix_time: time::Duration,
}

pub enum LinkRelation {
    Alternate,
    Related,
    SelfRel,
    Enclosure,
    Via,
}

pub enum ExtensionXmlChildren {
    Text { value: String },
    Nodes { children: Vec<ExtensionXmlNode> },
}

pub struct ExtensionXmlNode {
    name: String,
    attributes: Attributes,
    children: ExtensionXmlChildren,
    namespace: Option<String>,
}

impl Default for Attributes {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            // 1970-01-01 00:00
            unix_time: time::Duration::new(0, 0),
        }
    }
}
