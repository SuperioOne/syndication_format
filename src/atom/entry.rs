use crate::{
  common::{AttributeMap, AttributeName, AttributeValue, DateTime, XmlText},
  error::XmlSerializeError,
  serializer::{ElementSerializer, Serialize},
};

use super::{Author, Category, Content, Contributor, Id, Link, Rights, Summary, Title, Updated};

pub struct Entry<'a> {
  pub(crate) attributes: AttributeMap,
  pub authors: Vec<Author<'a>>,
  pub categories: Vec<Category<'a>>,
  pub content: Option<Content<'a>>,
  pub contributors: Vec<Contributor<'a>>,
  pub id: Id<'a>,
  pub links: Vec<Link<'a>>,
  pub summary: Option<Summary<'a>>,
  pub title: Title<'a>,
  pub updated: Updated,
  pub rights: Option<Rights<'a>>,
}

impl Serialize for Entry<'_> {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: crate::serializer::Serializer,
  {
    let mut entry = serializer.serialize_element("entry", namespace, Some(&self.attributes))?;

    for author in self.authors.iter() {
      entry.serialize(author, namespace)?;
    }

    for contributor in self.contributors.iter() {
      entry.serialize(contributor, namespace)?;
    }

    for category in self.categories.iter() {
      entry.serialize(category, namespace)?;
    }

    for link in self.links.iter() {
      entry.serialize(link, namespace)?;
    }

    if let Some(content) = self.content.as_ref() {
      entry.serialize(content, namespace)?;
    }

    if let Some(summary) = self.summary.as_ref() {
      entry.serialize(summary, namespace)?;
    }

    if let Some(rights) = self.rights.as_ref() {
      entry.serialize(rights, namespace)?;
    }

    entry.serialize(&self.title, namespace)?;
    entry.serialize(&self.id, namespace)?;
    entry.serialize(&self.updated, namespace)?;

    entry.end()?;
    Ok(())
  }
}

impl<'a> Entry<'a> {
  pub fn new(id: &'a str, title: XmlText<'a>, updated: DateTime) -> Self {
    Self {
      attributes: AttributeMap::default(),
      authors: Vec::default(),
      categories: Vec::default(),
      contributors: Vec::default(),
      id: Id {
        attributes: AttributeMap::default(),
        value: id,
      },
      links: Vec::default(),
      rights: None,
      content: None,
      summary: None,
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
