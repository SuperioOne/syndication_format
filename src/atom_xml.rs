use crate::attributes::AttributeMap;
use crate::common::{DateTime, LinkRelation, XmlText};

pub mod builder;
mod impls;

/// Generic helper node to simplify some property serializations
pub struct ElementNode<'a> {
  name: &'a str,
  namespace: Option<&'a str>,
  value: Option<&'a str>,
  attributes: Option<&'a AttributeMap>,
}

pub struct Feed<'a> {
  attributes: AttributeMap,
  authors: Vec<Author<'a>>,
  categories: Vec<Category<'a>>,
  contributors: Vec<Contributor<'a>>,
  entries: Vec<Entry<'a>>,
  generator: Option<Generator<'a>>,
  icon: Option<Icon<'a>>,
  id: Id<'a>,
  links: Vec<Link<'a>>,
  logo: Option<Logo<'a>>,
  namespace: Option<&'a str>,
  rights: Option<Rights<'a>>,
  subtitle: Option<SubTitle<'a>>,
  title: Title<'a>,
  updated: Updated<'a>,
}

pub struct Entry<'a> {
  attributes: AttributeMap,
  authors: Vec<Author<'a>>,
  categories: Vec<Category<'a>>,
  content: Option<Content<'a>>,
  contributors: Vec<Contributor<'a>>,
  id: Id<'a>,
  links: Vec<Link<'a>>,
  namespace: Option<&'a str>,
  rights: Option<Rights<'a>>,
  summary: Option<Summary<'a>>,
  title: Title<'a>,
  updated: Updated<'a>,
}

pub struct Author<'a> {
  attributes: AttributeMap,
  email: Option<&'a str>,
  name: &'a str,
  namespace: Option<&'a str>,
  uri: Option<&'a str>,
}

pub struct Contributor<'a> {
  attributes: AttributeMap,
  email: Option<&'a str>,
  name: &'a str,
  namespace: Option<&'a str>,
  uri: Option<&'a str>,
}

pub struct Category<'a> {
  attributes: AttributeMap,
  label: Option<&'a str>,
  namespace: Option<&'a str>,
  scheme: Option<&'a str>,
  term: &'a str,
}

pub struct Generator<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  uri: Option<&'a str>,
  value: &'a str,
  version: Option<&'a str>,
}

pub struct Icon<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: &'a str,
}

pub struct Id<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: &'a str,
}

pub struct Link<'a> {
  attributes: AttributeMap,
  href: &'a str,
  hreflang: Option<&'a str>,
  length: Option<usize>,
  link_type: Option<&'a str>,
  namespace: Option<&'a str>,
  rel: Option<LinkRelation>,
  title: Option<&'a str>,
}

pub struct Logo<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: &'a str,
}

pub struct Rights<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: XmlText<'a>,
}

pub struct Title<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: XmlText<'a>,
}

pub struct SubTitle<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: XmlText<'a>,
}

pub struct Summary<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: XmlText<'a>,
}

pub struct Content<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: XmlText<'a>,
}

pub struct Updated<'a> {
  attributes: AttributeMap,
  namespace: Option<&'a str>,
  value: DateTime,
}
