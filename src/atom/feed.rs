use crate::{
  common::{AttributeMap, AttributeName, AttributeValue, TimeStamp, XmlText},
  error::XmlSerializeError,
  serializer::{ElementSerializer, Serialize},
  utils::xml_name_token::is_valid_name,
};

use super::{
  Author, Category, Contributor, Entry, Generator, Icon, Id, Link, Logo, Rights, SubTitle, Title,
  Updated,
};

pub struct Feed<'a> {
  attributes: AttributeMap,
  pub authors: Vec<Author<'a>>,
  pub categories: Vec<Category<'a>>,
  pub contributors: Vec<Contributor<'a>>,
  pub entries: Vec<Entry<'a>>,
  pub generator: Option<Generator<'a>>,
  pub icon: Option<Icon<'a>>,
  pub id: Id<'a>,
  pub links: Vec<Link<'a>>,
  pub logo: Option<Logo<'a>>,
  pub rights: Option<Rights<'a>>,
  pub subtitle: Option<SubTitle<'a>>,
  pub title: Title<'a>,
  pub updated: Updated,
}

impl Serialize for Feed<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    if let Some(name) = namespace.as_deref() {
      if !is_valid_name(name) {
        return Err(XmlSerializeError::InvalidNamespace);
      }
    }

    let mut feed = serializer.serialize_element("feed", namespace, Some(&self.attributes))?;

    for author in self.authors.iter() {
      feed.serialize(author, namespace)?;
    }

    for contributor in self.contributors.iter() {
      feed.serialize(contributor, namespace)?;
    }

    for category in self.categories.iter() {
      feed.serialize(category, namespace)?;
    }

    for link in self.links.iter() {
      feed.serialize(link, namespace)?;
    }

    if let Some(generator) = self.generator.as_ref() {
      feed.serialize(generator, namespace)?;
    }

    if let Some(subtitle) = self.subtitle.as_ref() {
      feed.serialize(subtitle, namespace)?;
    }

    if let Some(logo) = self.logo.as_ref() {
      feed.serialize(logo, namespace)?;
    }

    if let Some(icon) = self.icon.as_ref() {
      feed.serialize(icon, namespace)?;
    }

    if let Some(rights) = self.rights.as_ref() {
      feed.serialize(rights, namespace)?;
    }

    feed.serialize(&self.title, namespace)?;
    feed.serialize(&self.id, namespace)?;
    feed.serialize(&self.updated, namespace)?;

    for entry in self.entries.iter() {
      feed.serialize(entry, namespace)?;
    }

    feed.end()?;
    Ok(())
  }
}

impl<'a> Feed<'a> {
  pub fn new(id: &'a str, title: XmlText<'a>, updated: TimeStamp) -> Self {
    Self {
      attributes: AttributeMap::default(),
      authors: Vec::default(),
      categories: Vec::default(),
      contributors: Vec::default(),
      entries: Vec::default(),
      generator: Some(Generator::default()),
      icon: None,
      id: Id {
        attributes: AttributeMap::default(),
        value: id,
      },
      links: Vec::default(),
      logo: None,
      rights: None,
      subtitle: None,
      title: Title {
        attributes: AttributeMap::default(),
        value: title,
      },
      updated: Updated {
        attributes: AttributeMap::default(),
        value: updated,
      },
    }
  }

  #[inline]
  pub fn set_attribute(&mut self, name: AttributeName, value: AttributeValue) -> &mut Self {
    self.attributes.set(name, value);
    self
  }

  #[inline]
  pub fn get_mut_attributes(&'a mut self) -> &'a mut AttributeMap {
    &mut self.attributes
  }
}
