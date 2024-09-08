use crate::attributes::{AttributeMap, AttributeName};
use crate::common::{DateTime, LinkRelation, XmlText};
use crate::serializer::{ElementSerializer, Serialize};

/// Generic helper node to simplify some property serializations
pub struct ElementNode<'a> {
  name: &'a str,
  namespace: Option<&'a str>,
  value: Option<&'a str>,
  attributes: Option<&'a AttributeMap>,
}

pub struct Feed<'a> {
  pub attributes: AttributeMap,
  pub authors: Vec<Author<'a>>,
  pub categories: Vec<Category<'a>>,
  pub contributors: Vec<Contributor<'a>>,
  pub entries: Vec<Entry<'a>>,
  pub generator: Option<Generator<'a>>,
  pub icon: Option<Icon<'a>>,
  pub id: Id<'a>,
  pub links: Vec<Link<'a>>,
  pub logo: Option<Logo<'a>>,
  pub namespace: Option<&'a str>,
  pub rights: Option<Rights<'a>>,
  pub subtitle: Option<SubTitle<'a>>,
  pub title: Title<'a>,
  pub updated: Updated<'a>,
}

pub struct Entry<'a> {
  pub attributes: AttributeMap,
  pub authors: Vec<Author<'a>>,
  pub categories: Vec<Category<'a>>,
  pub content: Option<Content<'a>>,
  pub contributors: Vec<Contributor<'a>>,
  pub id: Id<'a>,
  pub links: Vec<Link<'a>>,
  pub namespace: Option<&'a str>,
  pub rights: Option<Rights<'a>>,
  pub summary: Option<Summary<'a>>,
  pub title: Title<'a>,
  pub updated: Updated<'a>,
}

pub struct Author<'a> {
  pub attributes: AttributeMap,
  pub email: Option<&'a str>,
  pub name: &'a str,
  pub namespace: Option<&'a str>,
  pub uri: Option<&'a str>,
}

pub struct Contributor<'a> {
  pub attributes: AttributeMap,
  pub email: Option<&'a str>,
  pub name: &'a str,
  pub namespace: Option<&'a str>,
  pub uri: Option<&'a str>,
}

pub struct Category<'a> {
  pub attributes: AttributeMap,
  pub label: Option<&'a str>,
  pub namespace: Option<&'a str>,
  pub scheme: Option<&'a str>,
  pub term: &'a str,
}

pub struct Generator<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub uri: Option<&'a str>,
  pub value: &'a str,
  pub version: Option<&'a str>,
}

pub struct Icon<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: &'a str,
}

pub struct Id<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: &'a str,
}

pub struct Link<'a> {
  pub attributes: AttributeMap,
  pub href: &'a str,
  pub hreflang: Option<&'a str>,
  pub length: Option<usize>,
  pub link_type: Option<&'a str>,
  pub namespace: Option<&'a str>,
  pub rel: Option<LinkRelation>,
  pub title: Option<&'a str>,
}

pub struct Logo<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: &'a str,
}

pub struct Rights<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: XmlText<'a>,
}

pub struct Title<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: XmlText<'a>,
}

pub struct SubTitle<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: XmlText<'a>,
}

pub struct Summary<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: XmlText<'a>,
}

pub struct Content<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: XmlText<'a>,
}

pub struct Updated<'a> {
  pub attributes: AttributeMap,
  pub namespace: Option<&'a str>,
  pub value: DateTime,
}

macro_rules! set_from_option {
  ($map:expr, $name:expr, $value:expr) => {
    if let Some(value) = $value {
      $map.set(AttributeName::unchecked_new($name), value.into());
    }
  };
}

macro_rules! set_from_value {
  ($map:expr, $name:expr, $value:expr) => {
    $map.set(AttributeName::unchecked_new($name), $value.into());
  };
}

macro_rules! impl_serialize_for_text_node {
  ($name:expr, $type:ty) => {
    impl $crate::serializer::Serialize for $type {
      fn serialize<S>(&self, serializer: S) -> $crate::serializer::Result<()>
      where
        S: crate::serializer::Serializer,
      {
        let mut attributes = AttributeMap::with_capacity(self.attributes.len() + 1);

        match self.value {
          XmlText::Html(_) => {
            set_from_value!(attributes, "type", "html");
          }
          XmlText::Xhtml(_) => {
            set_from_value!(attributes, "type", "xhtml");
          }
          XmlText::PlainText(_) => {
            set_from_value!(attributes, "type", "text");
          }
        };

        let element = serializer.serialize_element($name, self.namespace, Some(&attributes))?;
        element.serialize_str(&self.value.as_normalized_str())?;

        Ok(())
      }
    }
  };
}

impl_serialize_for_text_node!("rights", Rights<'_>);
impl_serialize_for_text_node!("subtitle", SubTitle<'_>);
impl_serialize_for_text_node!("title", Title<'_>);
impl_serialize_for_text_node!("summary", Summary<'_>);
impl_serialize_for_text_node!("content", Content<'_>);

impl Serialize for ElementNode<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    match self.value {
      Some(value) => {
        let element = serializer.serialize_element(self.name, self.namespace, self.attributes)?;
        element.serialize_str(value)?;
      }
      None => serializer.serialize_empty_element(self.name, self.namespace, self.attributes)?,
    }

    Ok(())
  }
}

impl Serialize for Author<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let mut person_root =
      serializer.serialize_element("author", self.namespace, Some(&self.attributes))?;

    let name_element = ElementNode {
      name: "name",
      namespace: self.namespace,
      value: Some(self.name),
      attributes: None,
    };

    person_root.serialize(name_element)?;

    if let Some(uri) = self.uri {
      let uri_element = ElementNode {
        name: "uri",
        namespace: self.namespace,
        value: Some(uri),
        attributes: None,
      };

      person_root.serialize(uri_element)?;
    }

    if let Some(email) = self.email {
      let email_element = ElementNode {
        name: "email",
        namespace: self.namespace,
        value: Some(email),
        attributes: None,
      };

      person_root.serialize(email_element)?;
    }

    person_root.end()?;

    Ok(())
  }
}

impl Serialize for Category<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let mut attributes = AttributeMap::with_capacity(self.attributes.len() + 3);
    attributes.set_from(&self.attributes);

    set_from_value!(attributes, "term", self.term);
    set_from_option!(attributes, "scheme", self.scheme);
    set_from_option!(attributes, "label", self.label);

    serializer.serialize_empty_element("category", self.namespace, Some(&attributes))?;

    Ok(())
  }
}

impl Serialize for Contributor<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let mut person_root =
      serializer.serialize_element("contributor", self.namespace, Some(&self.attributes))?;

    let name_element = ElementNode {
      name: "name",
      namespace: self.namespace,
      value: Some(self.name),
      attributes: None,
    };

    person_root.serialize(name_element)?;

    if let Some(uri) = self.uri {
      let uri_element = ElementNode {
        name: "uri",
        namespace: self.namespace,
        value: Some(uri),
        attributes: None,
      };

      person_root.serialize(uri_element)?;
    }

    if let Some(email) = self.email {
      let email_element = ElementNode {
        name: "email",
        namespace: self.namespace,
        value: Some(email),
        attributes: None,
      };

      person_root.serialize(email_element)?;
    }

    person_root.end()?;

    Ok(())
  }
}

impl Serialize for Generator<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let mut attributes = AttributeMap::with_capacity(self.attributes.len() + 2);
    attributes.set_from(&self.attributes);

    set_from_option!(attributes, "version", self.version);
    set_from_option!(attributes, "uri", self.uri);

    let generator_element =
      serializer.serialize_element("generator", self.namespace, Some(&attributes))?;

    generator_element.serialize_str(self.value)?;

    Ok(())
  }
}

impl Serialize for Icon<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let icon_element =
      serializer.serialize_element("icon", self.namespace, Some(&self.attributes))?;
    icon_element.serialize_str(self.value)?;
    Ok(())
  }
}

impl Serialize for Logo<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let icon_element =
      serializer.serialize_element("logo", self.namespace, Some(&self.attributes))?;
    icon_element.serialize_str(self.value)?;
    Ok(())
  }
}

impl Serialize for Link<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let mut attributes = AttributeMap::with_capacity(self.attributes.len() + 5);
    attributes.set_from(&self.attributes);

    set_from_value!(attributes, "href", self.href);
    set_from_option!(attributes, "hreflang", self.hreflang);
    set_from_option!(attributes, "rel", self.rel.map(|v| v.as_str()));
    set_from_option!(attributes, "type", self.link_type);
    set_from_option!(attributes, "title", self.title);
    set_from_option!(attributes, "length", self.length.map(|v| v.to_string()));

    serializer.serialize_empty_element("link", self.namespace, Some(&attributes))?;
    Ok(())
  }
}

impl Serialize for Id<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let id_element = serializer.serialize_element("id", self.namespace, Some(&self.attributes))?;
    id_element.serialize_str(self.value)?;
    Ok(())
  }
}

impl Serialize for Updated<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let updated =
      serializer.serialize_element("updated", self.namespace, Some(&self.attributes))?;
    updated.serialize_str(self.value.as_str())?;
    Ok(())
  }
}

impl Serialize for Feed<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let mut feed = serializer.serialize_element("feed", self.namespace, Some(&self.attributes))?;

    for author in self.authors.iter() {
      feed.serialize(author)?;
    }

    for contributor in self.contributors.iter() {
      feed.serialize(contributor)?;
    }

    for category in self.categories.iter() {
      feed.serialize(category)?;
    }

    for link in self.links.iter() {
      feed.serialize(link)?;
    }

    if let Some(generator) = self.generator.as_ref() {
      feed.serialize(generator)?;
    }

    if let Some(subtitle) = self.subtitle.as_ref() {
      feed.serialize(subtitle)?;
    }

    if let Some(logo) = self.logo.as_ref() {
      feed.serialize(logo)?;
    }

    if let Some(icon) = self.icon.as_ref() {
      feed.serialize(icon)?;
    }

    if let Some(rights) = self.rights.as_ref() {
      feed.serialize(rights)?;
    }

    feed.serialize(&self.title)?;
    feed.serialize(&self.id)?;
    feed.serialize(&self.updated)?;

    for entry in self.entries.iter() {
      feed.serialize(entry)?;
    }

    feed.end()?;
    Ok(())
  }
}

impl Serialize for Entry<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let mut entry =
      serializer.serialize_element("entry", self.namespace, Some(&self.attributes))?;

    for author in self.authors.iter() {
      entry.serialize(author)?;
    }

    for contributor in self.contributors.iter() {
      entry.serialize(contributor)?;
    }

    for category in self.categories.iter() {
      entry.serialize(category)?;
    }

    for link in self.links.iter() {
      entry.serialize(link)?;
    }

    if let Some(content) = self.content.as_ref() {
      entry.serialize(content)?;
    }

    if let Some(summary) = self.summary.as_ref() {
      entry.serialize(summary)?;
    }

    if let Some(rights) = self.rights.as_ref() {
      entry.serialize(rights)?;
    }

    entry.serialize(&self.title)?;
    entry.serialize(&self.id)?;
    entry.serialize(&self.updated)?;

    entry.end()?;
    Ok(())
  }
}
