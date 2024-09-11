use crate::atom_xml::{Author, Feed, Id, Title, Updated};
use crate::attributes::{AttributeMap, AttributeName, AttributeValue};
use crate::common::{DateTime, XmlText};

pub struct Builder<'a> {
  feed: Feed<'a>,
}

impl<'a> Feed<'a> {
  pub fn new(id: &'a str, title: XmlText<'a>, updated: DateTime) -> Self {
    Self {
      attributes: AttributeMap::default(),
      authors: Vec::default(),
      categories: Vec::default(),
      contributors: Vec::default(),
      entries: Vec::default(),
      generator: None,
      icon: None,
      id: Id {
        attributes: AttributeMap::default(),
        namespace: None,
        value: id,
      },
      links: Vec::default(),
      logo: None,
      namespace: None,
      rights: None,
      subtitle: None,
      title: Title {
        attributes: AttributeMap::default(),
        namespace: None,
        value: title,
      },
      updated: Updated {
        attributes: AttributeMap::default(),
        namespace: None,
        value: updated,
      },
    }
  }

  pub fn add_author(&mut self, mut author: Author<'a>) -> &mut Self {
    author.namespace = self.namespace;
    self.authors.push(author);
    self
  }
}

impl<'a> Author<'a> {
  pub fn new(name: &'a str) -> Self {
    Self {
      attributes: AttributeMap::default(),
      namespace: None,
      uri: None,
      email: None,
      name,
    }
  }

  pub fn set_name(&mut self, name: &'a str) -> &mut Self {
    self.name = name;
    self
  }

  pub fn set_email(&mut self, email: &'a str) -> &mut Self {
    self.email = Some(email);
    self
  }

  pub fn set_uri(&mut self, uri: &'a str) -> &mut Self {
    self.uri = Some(uri);
    self
  }

  pub fn set_attribute(&mut self, name: AttributeName, value: AttributeValue) -> &mut Self {
    self.attributes.set(name, value);
    self
  }

  pub fn get_mut_attributes(&'a mut self) -> &'a mut AttributeMap {
    &mut self.attributes
  }
}
