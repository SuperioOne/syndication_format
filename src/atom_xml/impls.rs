use crate::{
  attributes::{AttributeMap, AttributeName},
  common::XmlText,
  serializer::{ElementSerializer, Serialize},
};

use super::{
  Author, Category, Content, Contributor, ElementNode, Entry, Feed, Generator, Icon, Id, Link,
  Logo, Rights, SubTitle, Summary, Title, Updated,
};

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
          XmlText::HtmlUnchecked(_) => {
            set_from_value!(attributes, "type", "html");
          }
          XmlText::Xhtml(_) => {
            set_from_value!(attributes, "type", "xhtml");
          }
          XmlText::PlainText(_) => {
            set_from_value!(attributes, "type", "text");
          }
          XmlText::PlainTextUnchecked(_) => {
            set_from_value!(attributes, "type", "text");
          }
        };

        let element = serializer.serialize_element($name, self.namespace, Some(&attributes))?;

        match self.value {
          XmlText::Html(value) => {
            element.serialize_escaped_str(&value)?;
          }
          XmlText::HtmlUnchecked(value) => {
            element.serialize_str(&value)?;
          }
          XmlText::Xhtml(value) => {
            element.serialize_str(&value)?;
          }
          XmlText::PlainText(value) => {
            element.serialize_escaped_str(&value)?;
          }
          XmlText::PlainTextUnchecked(value) => {
            element.serialize_str(&value)?;
          }
        };

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
        element.serialize_escaped_str(&value)?;
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

    let generator = serializer.serialize_element("generator", self.namespace, Some(&attributes))?;

    generator.serialize_escaped_str(&self.value)?;

    Ok(())
  }
}

impl Serialize for Icon<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let icon = serializer.serialize_element("icon", self.namespace, Some(&self.attributes))?;
    icon.serialize_escaped_str(&self.value)?;
    Ok(())
  }
}

impl Serialize for Logo<'_> {
  fn serialize<S>(&self, serializer: S) -> crate::serializer::Result<()>
  where
    S: crate::serializer::Serializer,
  {
    let logo = serializer.serialize_element("logo", self.namespace, Some(&self.attributes))?;
    logo.serialize_escaped_str(&self.value)?;
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
    let id = serializer.serialize_element("id", self.namespace, Some(&self.attributes))?;
    id.serialize_escaped_str(&self.value)?;
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
