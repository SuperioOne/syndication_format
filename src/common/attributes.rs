use core::borrow::Borrow;
use core::fmt::Display;
use core::ops::Deref;
use core::str::FromStr;

use crate::error::InvalidAttributeName;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AttributeName {
  name: Box<str>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AttributeValue {
  value: Box<str>,
}

#[derive(Clone)]
pub struct Attribute {
  pub name: AttributeName,
  pub value: AttributeValue,
}

// Attributes impls

#[derive(Clone)]
pub struct AttributeMap {
  // implementing Map structure with Vec since expected item count is mostly less than 5.
  inner: Vec<Attribute>,
}

impl Default for AttributeMap {
  fn default() -> Self {
    Self { inner: Vec::new() }
  }
}

pub struct AttributeMapIter<I, T>
where
  I: Sized + Iterator<Item = T>,
{
  inner_iterator: I,
}

impl<I, T> Iterator for AttributeMapIter<I, T>
where
  I: Sized + Iterator<Item = T>,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.inner_iterator.next()
  }
}

impl AttributeMap {
  pub fn new() -> Self {
    Self { inner: Vec::new() }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      inner: Vec::with_capacity(capacity),
    }
  }

  pub fn new_from(other: &Self) -> Self {
    if other.inner.is_empty() {
      Self::new()
    } else {
      Self {
        inner: other.inner.clone(),
      }
    }
  }

  pub fn len(&self) -> usize {
    self.inner.len()
  }

  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  pub fn set(&mut self, name: AttributeName, value: AttributeValue) {
    for attribute in self.inner.iter_mut() {
      if attribute.name.eq(&name) {
        attribute.value = value;
        return;
      }
    }

    self.inner.push(Attribute { name, value });
  }

  pub fn get(&self, name: &str) -> Option<&AttributeValue> {
    for attribute in self.inner.iter() {
      if attribute.name.deref().eq(name) {
        return Some(&attribute.value);
      }
    }

    None
  }

  pub fn iter(&self) -> AttributeMapIter<core::slice::Iter<'_, Attribute>, &Attribute> {
    AttributeMapIter {
      inner_iterator: self.inner.iter(),
    }
  }
}

impl<'a> IntoIterator for &'a AttributeMap {
  type Item = &'a Attribute;

  type IntoIter = AttributeMapIter<core::slice::Iter<'a, Attribute>, &'a Attribute>;

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      inner_iterator: self.inner.iter(),
    }
  }
}

// AttributeName impls

// TODO: check attribute name validity

impl FromStr for AttributeName {
  type Err = InvalidAttributeName;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if true {
      Ok(Self { name: Box::from(s) })
    } else {
      Err(InvalidAttributeName)
    }
  }
}

impl Deref for AttributeName {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.name
  }
}

impl Display for AttributeName {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(&self.name)
  }
}

impl AttributeName {
  pub fn new(name: &str) -> Result<Self, InvalidAttributeName> {
    Ok(Self {
      name: Box::from(name),
    })
  }

  /// Allows name check skipping for the internal well-known attribute names.
  pub(crate) fn unchecked_new(name: &str) -> Self {
    Self {
      name: Box::from(name),
    }
  }

  pub fn get(&self) -> &str {
    &self.name
  }
}

// AttributeValue impls

impl<T> From<T> for AttributeValue
where
  T: Borrow<str>,
{
  fn from(value: T) -> Self {
    let str_ref: &str = value.borrow();

    Self {
      value: Box::from(str_ref),
    }
  }
}

impl Deref for AttributeValue {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl Display for AttributeValue {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(&self.value)
  }
}

impl AttributeValue {
  pub fn new(value: &str) -> Self {
    Self {
      value: Box::from(value),
    }
  }

  pub fn get(&self) -> &str {
    &self.value
  }

  pub fn set(&mut self, new_value: &str) {
    self.value = Box::from(new_value);
  }
}
