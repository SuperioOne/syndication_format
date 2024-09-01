pub mod indented_writer;

use std::io::{Error, Write};

use crate::common::Attributes;

enum ElementState {
  EmptyNode,
  LeafNode,
  TreeNode,
}

#[derive(Debug)]
pub enum XmlSerializeError {
  IOError { inner: Box<Error> },
}

pub type Result<T> = core::result::Result<T, XmlSerializeError>;

pub trait Serialize {
  fn serialize<S>(&self, serializer: S) -> Result<()>
  where
    S: Serializer;
}

pub trait Serializer: Sized {
  type ElementSerializer: ElementSerializer;

  fn serialize_str(self, value: &str) -> Result<()>;
  fn serialize<V>(self, value: V) -> Result<()>
  where
    V: Serialize;

  fn serialize_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&Attributes>,
  ) -> Result<Self::ElementSerializer>;
}

pub trait ElementSerializer {
  fn serialize<V>(&mut self, value: V) -> Result<()>
  where
    V: Serialize;
  fn serialize_str(self, value: &str) -> Result<()>;
  fn end(self) -> Result<()>;
}

pub struct XmlSerializer<'a, W>
where
  W: Write + ?Sized,
{
  pub(super) writer: &'a mut W,
}

pub struct XmlElementSerializer<'o, W>
where
  W: Write + ?Sized,
{
  pub(super) inner: &'o mut XmlSerializer<'o, W>,
}

impl<'a, W> Serializer for &'a mut XmlSerializer<'a, W>
where
  W: Write + ?Sized,
{
  type ElementSerializer = XmlElementSerializer<'a, W>;

  fn serialize_str(self, value: &str) -> Result<()> {
    let _ = self.writer.write_all(value.as_bytes());
    Ok(())
  }

  fn serialize<V>(self, value: V) -> Result<()>
  where
    V: Serialize,
  {
    value.serialize(self)
  }

  fn serialize_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&Attributes>,
  ) -> Result<Self::ElementSerializer> {
    let _ = self.writer.write_all(format!("<{}>", name).as_bytes());
    Ok(Self::ElementSerializer { inner: self })
  }
}

impl<'a, W> ElementSerializer for XmlElementSerializer<'a, W>
where
  W: Write + ?Sized,
{
  fn serialize<V>(&mut self, value: V) -> Result<()>
  where
    V: Serialize,
  {
    let mut temp = XmlSerializer {
      writer: self.inner.writer,
    };

    let _ = value.serialize(&mut temp);

    Ok(())
  }

  fn serialize_str(self, value: &str) -> Result<()> {
    let _ = self.inner.writer.write_all(value.as_bytes());

    Ok(())
  }

  fn end(self) -> Result<()> {
    let _ = self
      .inner
      .writer
      .write_all(format!("</{}>", "test").as_bytes());

    Ok(())
  }
}

pub struct Nested {
  value: String,
}

pub struct Test {
  value: Nested,
}

impl Serialize for &Nested {
  fn serialize<S>(&self, serializer: S) -> Result<()>
  where
    S: Serializer,
  {
    let mut test = serializer.serialize_element("nested", None, None).unwrap();

    let _ = test.serialize_str(&self.value);

    Ok(())
  }
}

impl Serialize for Test {
  fn serialize<S>(&self, serializer: S) -> Result<()>
  where
    S: Serializer,
  {
    let mut test = serializer.serialize_element("test", None, None).unwrap();

    let _ = test.serialize(&self.value);

    let _ = test.end();
    Ok(())
  }
}

mod test {
  use std::str::from_utf8_unchecked;

  use super::{Serializer, Test, XmlSerializer};

  #[test]
  fn does_it_work() {
    let obj = Test {
      value: super::Nested {
        value: String::from("Hello world"),
      },
    };

    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = XmlSerializer {
      writer: &mut buffer,
    };

    let _ = writer.serialize(obj);

    println!("{}", unsafe { from_utf8_unchecked(&buffer) });
    assert!(true);
  }
}
