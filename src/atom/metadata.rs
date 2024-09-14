use super::ElementNode;
use crate::{
  common::{AttributeMap, AttributeName, AttributeValue, DateTime, LinkRelation, Uri, XmlText},
  error::XmlSerializeError,
  serializer::{ElementSerializer, Serialize},
};

macro_rules! impl_attribute_fns {
  () => {
    #[inline]
    pub fn set_attribute(&mut self, name: AttributeName, value: AttributeValue) -> &mut Self {
      self.attributes.set(name, value);
      self
    }

    #[inline]
    pub fn get_mut_attributes(&'a mut self) -> &'a mut AttributeMap {
      &mut self.attributes
    }
  };
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
      fn serialize<S>(
        &self,
        serializer: S,
        namespace: Option<&str>,
      ) -> Result<(), $crate::error::XmlSerializeError>
      where
        S: crate::serializer::Serializer,
      {
        let mut attributes = AttributeMap::new_from(&self.attributes);

        match self.value {
          XmlText::Html(value) => {
            set_from_value!(attributes, "type", "html");
            let element = serializer.serialize_element($name, namespace, Some(&attributes))?;
            element.serialize_escaped_str(&value)?;
          }
          XmlText::HtmlUnchecked(value) => {
            set_from_value!(attributes, "type", "html");
            let element = serializer.serialize_element($name, namespace, Some(&attributes))?;
            element.serialize_str(&value)?;
          }
          XmlText::Xhtml(value) => {
            set_from_value!(attributes, "type", "xhtml");
            let element = serializer.serialize_element($name, namespace, Some(&attributes))?;
            element.serialize_str(&value)?;
          }
          XmlText::PlainText(value) => {
            set_from_value!(attributes, "type", "text");
            let element = serializer.serialize_element($name, namespace, Some(&attributes))?;
            element.serialize_escaped_str(&value)?;
          }
          XmlText::PlainTextUnchecked(value) => {
            set_from_value!(attributes, "type", "text");
            let element = serializer.serialize_element($name, namespace, Some(&attributes))?;
            element.serialize_str(&value)?;
          }
        };

        Ok(())
      }
    }
  };
}

pub struct Author<'a> {
  pub(crate) attributes: AttributeMap,
  pub email: Option<&'a str>,
  pub name: &'a str,
  pub uri: Option<&'a str>,
}

impl<'a> Author<'a> {
  pub fn new(name: &'a str) -> Self {
    Self {
      attributes: AttributeMap::default(),
      uri: None,
      email: None,
      name,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Author<'_> {
  fn serialize<S>(
    &self,
    serializer: S,
    namespace: Option<&str>,
  ) -> core::result::Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let mut person = serializer.serialize_element("author", namespace, Some(&self.attributes))?;

    let name_element = ElementNode {
      name: "name",
      value: Some(self.name),
      attributes: None,
    };

    person.serialize(name_element, namespace)?;

    if let Some(uri) = self.uri {
      let uri_element = ElementNode {
        name: "uri",
        value: Some(uri),
        attributes: None,
      };

      person.serialize(uri_element, namespace)?;
    }

    if let Some(email) = self.email {
      let email_element = ElementNode {
        name: "email",
        value: Some(email),
        attributes: None,
      };

      person.serialize(email_element, namespace)?;
    }

    person.end()?;

    Ok(())
  }
}

pub struct Contributor<'a> {
  pub(crate) attributes: AttributeMap,
  pub email: Option<&'a str>,
  pub name: &'a str,
  pub uri: Option<&'a str>,
}

impl<'a> Contributor<'a> {
  pub fn new(name: &'a str) -> Self {
    Self {
      attributes: AttributeMap::default(),
      uri: None,
      email: None,
      name,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Contributor<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let mut person =
      serializer.serialize_element("contributor", namespace, Some(&self.attributes))?;

    let name_element = ElementNode {
      name: "name",
      value: Some(self.name),
      attributes: None,
    };

    person.serialize(name_element, namespace)?;

    if let Some(uri) = self.uri {
      let uri_element = ElementNode {
        name: "uri",
        value: Some(uri),
        attributes: None,
      };

      person.serialize(uri_element, namespace)?;
    }

    if let Some(email) = self.email {
      let email_element = ElementNode {
        name: "email",
        value: Some(email),
        attributes: None,
      };

      person.serialize(email_element, namespace)?;
    }

    person.end()?;

    Ok(())
  }
}

pub struct Category<'a> {
  pub(crate) attributes: AttributeMap,
  pub label: Option<&'a str>,
  pub scheme: Option<&'a str>,
  pub term: &'a str,
}

impl<'a> Category<'a> {
  pub fn new(term: &'a str) -> Self {
    Self {
      attributes: AttributeMap::default(),
      term,
      scheme: None,
      label: None,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Category<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let mut attributes = AttributeMap::new_from(&self.attributes);
    set_from_value!(attributes, "term", self.term);
    set_from_option!(attributes, "scheme", self.scheme);
    set_from_option!(attributes, "label", self.label);

    serializer.serialize_empty_element("category", namespace, Some(&attributes))?;

    Ok(())
  }
}

pub struct Generator<'a> {
  pub(crate) attributes: AttributeMap,
  pub uri: Option<&'a str>,
  pub value: &'a str,
  pub version: Option<&'a str>,
}

impl Default for Generator<'_> {
  fn default() -> Self {
    Self {
      attributes: AttributeMap::default(),
      value: env!("CARGO_PKG_NAME"),
      uri: Some(env!("CARGO_PKG_HOMEPAGE")),
      version: Some(env!("CARGO_PKG_VERSION")),
    }
  }
}

impl<'a> Generator<'a> {
  pub fn new(value: &'a str) -> Self {
    Self {
      attributes: AttributeMap::default(),
      value,
      uri: None,
      version: None,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Generator<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let mut attributes = AttributeMap::new_from(&self.attributes);
    set_from_option!(attributes, "version", self.version);
    set_from_option!(attributes, "uri", self.uri);

    let generator = serializer.serialize_element("generator", namespace, Some(&attributes))?;

    generator.serialize_escaped_str(self.value)?;

    Ok(())
  }
}

pub struct Icon<'a> {
  pub(crate) attributes: AttributeMap,
  pub uri: Uri<'a>,
}

impl<'a> Icon<'a> {
  pub fn new(uri: Uri<'a>) -> Self {
    Self {
      attributes: AttributeMap::default(),
      uri,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Icon<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let icon = serializer.serialize_element("icon", namespace, Some(&self.attributes))?;
    icon.serialize_escaped_str(self.uri)?;
    Ok(())
  }
}

pub struct Id<'a> {
  pub(crate) attributes: AttributeMap,
  pub value: &'a str,
}

impl<'a> Id<'a> {
  pub fn new(value: &'a str) -> Self {
    Self {
      attributes: AttributeMap::default(),
      value,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Id<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let id = serializer.serialize_element("id", namespace, Some(&self.attributes))?;
    id.serialize_escaped_str(self.value)?;
    Ok(())
  }
}

pub struct Link<'a> {
  pub(crate) attributes: AttributeMap,
  pub href: Uri<'a>,
  pub hreflang: Option<&'a str>,
  pub length: Option<usize>,
  pub link_type: Option<&'a str>,
  pub rel: Option<LinkRelation>,
  pub title: Option<&'a str>,
}

impl<'a> Link<'a> {
  pub fn new(href: Uri<'a>) -> Self {
    Self {
      attributes: AttributeMap::default(),
      title: None,
      href,
      hreflang: None,
      length: None,
      link_type: None,
      rel: None,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Link<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let mut attributes = AttributeMap::new_from(&self.attributes);
    set_from_value!(attributes, "href", self.href);
    set_from_option!(attributes, "hreflang", self.hreflang);
    set_from_option!(attributes, "rel", self.rel.map(|v| v.as_str()));
    set_from_option!(attributes, "type", self.link_type);
    set_from_option!(attributes, "title", self.title);
    set_from_option!(attributes, "length", self.length.map(|v| v.to_string()));

    serializer.serialize_empty_element("link", namespace, Some(&attributes))?;
    Ok(())
  }
}

pub struct Logo<'a> {
  pub(crate) attributes: AttributeMap,
  pub uri: Uri<'a>,
}

impl<'a> Logo<'a> {
  pub fn new(uri: Uri<'a>) -> Self {
    Self {
      attributes: AttributeMap::default(),
      uri,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Logo<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let logo = serializer.serialize_element("logo", namespace, Some(&self.attributes))?;
    logo.serialize_escaped_str(self.uri)?;
    Ok(())
  }
}

pub struct Rights<'a> {
  pub(crate) attributes: AttributeMap,
  pub value: XmlText<'a>,
}

impl<'a> Rights<'a> {
  pub fn new(value: XmlText<'a>) -> Self {
    Self {
      attributes: AttributeMap::default(),
      value,
    }
  }

  impl_attribute_fns!();
}

impl_serialize_for_text_node!("rights", Rights<'_>);

pub struct Title<'a> {
  pub(crate) attributes: AttributeMap,
  pub value: XmlText<'a>,
}

impl<'a> Title<'a> {
  pub fn new(value: XmlText<'a>) -> Self {
    Self {
      attributes: AttributeMap::default(),
      value,
    }
  }

  impl_attribute_fns!();
}

impl_serialize_for_text_node!("title", Title<'_>);

pub struct SubTitle<'a> {
  pub(crate) attributes: AttributeMap,
  pub value: XmlText<'a>,
}

impl<'a> SubTitle<'a> {
  pub fn new(value: XmlText<'a>) -> Self {
    Self {
      attributes: AttributeMap::default(),
      value,
    }
  }

  impl_attribute_fns!();
}

impl_serialize_for_text_node!("subtitle", SubTitle<'_>);

pub struct Summary<'a> {
  pub(crate) attributes: AttributeMap,
  pub value: XmlText<'a>,
}

impl<'a> Summary<'a> {
  pub fn new(value: XmlText<'a>) -> Self {
    Self {
      attributes: AttributeMap::default(),
      value,
    }
  }

  impl_attribute_fns!();
}

impl_serialize_for_text_node!("summary", Summary<'_>);

pub struct Content<'a> {
  pub value: ContentValue<'a>,
  pub(crate) attributes: AttributeMap,
}

pub enum ContentValue<'a> {
  TextContent { text: XmlText<'a> },
  LinkContent { media_type: &'a str, src: Uri<'a> },
  InlinedMedia { media_type: &'a str, data: &'a str },
}

impl<'a> From<XmlText<'a>> for ContentValue<'a> {
  fn from(value: XmlText<'a>) -> Self {
    Self::TextContent { text: value }
  }
}

impl<'a> Content<'a> {
  pub fn new(value: ContentValue<'a>) -> Self {
    Self {
      attributes: AttributeMap::default(),
      value,
    }
  }

  impl_attribute_fns!();
}

impl Serialize for Content<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    const ELEMENT_NAME: &str = "content";

    match &self.value {
      ContentValue::TextContent { text } => {
        let mut attributes = AttributeMap::with_capacity(self.attributes.len() + 1);

        match *text {
          XmlText::Html(value) => {
            set_from_value!(attributes, "type", "html");
            let content =
              serializer.serialize_element(ELEMENT_NAME, namespace, Some(&self.attributes))?;
            content.serialize_escaped_str(value)?;
          }
          XmlText::HtmlUnchecked(value) => {
            set_from_value!(attributes, "type", "html");
            let content =
              serializer.serialize_element(ELEMENT_NAME, namespace, Some(&self.attributes))?;
            content.serialize_str(value)?;
          }
          XmlText::Xhtml(value) => {
            set_from_value!(attributes, "type", "xhtml");
            let content =
              serializer.serialize_element(ELEMENT_NAME, namespace, Some(&self.attributes))?;
            content.serialize_str(value)?;
          }
          XmlText::PlainText(value) => {
            set_from_value!(attributes, "type", "text");
            let content =
              serializer.serialize_element(ELEMENT_NAME, namespace, Some(&self.attributes))?;
            content.serialize_escaped_str(value)?;
          }
          XmlText::PlainTextUnchecked(value) => {
            set_from_value!(attributes, "type", "text");
            let content =
              serializer.serialize_element(ELEMENT_NAME, namespace, Some(&self.attributes))?;
            content.serialize_str(value)?;
          }
        };

        Ok(())
      }
      ContentValue::LinkContent { media_type, src } => {
        let mut attributes = AttributeMap::new_from(&self.attributes);
        set_from_value!(attributes, "type", *media_type);
        set_from_value!(attributes, "src", *src);

        serializer.serialize_empty_element(ELEMENT_NAME, namespace, Some(&attributes))?;
        Ok(())
      }
      ContentValue::InlinedMedia { media_type, data } => {
        let mut attributes = AttributeMap::new_from(&self.attributes);
        set_from_value!(attributes, "type", *media_type);

        let content = serializer.serialize_element(ELEMENT_NAME, namespace, Some(&attributes))?;
        content.serialize_escaped_str(data)?;
        Ok(())
      }
    }
  }
}

pub struct Updated {
  pub(crate) attributes: AttributeMap,
  pub value: DateTime,
}

impl Serialize for Updated {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let updated = serializer.serialize_element("updated", namespace, Some(&self.attributes))?;
    updated.serialize_str(self.value.as_str())?;
    Ok(())
  }
}
