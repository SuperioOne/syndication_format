#[cfg(feature = "std")]
pub mod formatter;

use core::fmt::Arguments;

use crate::{
  common::AttributeMap,
  error::XmlSerializeError,
  escape::{escape_writer, XML_ESCAPE_PATTERNS},
  escape_xml_attr,
};

pub trait Write {
  fn write(&mut self, data: &str) -> Result<(), XmlSerializeError>;
  fn write_line(&mut self, data: &str) -> Result<(), XmlSerializeError>;
  fn write_fmt(&mut self, f: Arguments) -> Result<(), XmlSerializeError>;
  fn increment_level(&mut self);
  fn decrement_level(&mut self);
  fn set_level(&mut self, level: usize);
  fn get_level(&self) -> usize;
}

enum ElementType {
  /// Empty element, or self-closing elements
  /// Example:
  /// ```xml
  ///   <test />
  /// ```
  Empty,

  /// Non-empty elements with children elements or value
  ///
  /// Example:
  /// ```xml
  ///   <test>
  ///     ...
  ///     ...
  ///   </test>
  /// ```
  NonEmpty,
}

pub trait Serialize {
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: Serializer;
}

// Blanket implementation for serializable value type references.
impl<T> Serialize for &'_ T
where
  T: Serialize,
{
  fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    S: Serializer,
  {
    (*self).serialize(serializer, namespace)?;
    Ok(())
  }
}

pub trait Serializer: Sized {
  type ElementSerializer: ElementSerializer;

  /// Serializes string as element's value.
  fn serialize_str(self, value: &str) -> Result<(), XmlSerializeError>;

  /// Escapes and serializes string as element's value.
  fn serialize_escaped_str(self, value: &str) -> Result<(), XmlSerializeError>;

  fn serialize<V>(self, value: V, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    V: Serialize;

  /// Creates an element tag.
  ///
  /// Basic Example:
  /// ```
  /// use atom_syndication_format::common::{ AttributeMap, AttributeName };
  /// use atom_syndication_format::serializer::*;
  /// use atom_syndication_format::error::{XmlSerializeError};
  ///
  /// pub struct BasicExample {
  ///   lang: String,
  ///   content: String
  /// }
  ///
  /// impl Serialize for BasicExample {
  ///   fn serialize<S>(&self, serializer: S, namespace: Option<&str>) -> Result<(),
  ///   XmlSerializeError>
  ///     where
  ///       S: Serializer,
  ///   {
  ///     let mut attributes = AttributeMap::new();
  ///     attributes.set(AttributeName::new("lang")?, self.lang.as_str().into());
  ///
  ///     let mut root = serializer.serialize_element("example", namespace, Some(&attributes))?;
  ///     root.serialize_escaped_str(&self.content)?;
  ///
  ///     Ok(())
  ///   }
  ///}
  /// ```
  ///
  /// Output:
  /// ```xml
  ///   <example lang="en-GB">
  ///     Lorem ipsum &amp; ...
  ///   </example>
  /// ```
  fn serialize_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&AttributeMap>,
  ) -> Result<Self::ElementSerializer, XmlSerializeError>;

  /// Creates an empty element without any child node and value.
  /// ```xml
  ///   <tag />
  /// ```
  fn serialize_empty_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&AttributeMap>,
  ) -> Result<(), XmlSerializeError>;
}

pub trait ElementSerializer {
  fn serialize<V>(&mut self, value: V, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    V: Serialize;

  fn serialize_str(self, value: &str) -> Result<(), XmlSerializeError>;

  fn serialize_escaped_str(self, value: &str) -> Result<(), XmlSerializeError>;

  fn end(self) -> Result<(), XmlSerializeError>;
}

pub struct XmlSerializer<'a, W>
where
  W: Write + ?Sized,
{
  pub(super) writer: &'a mut W,
}

pub struct XmlElementSerializer<'a, W>
where
  W: Write + ?Sized,
{
  inner: &'a mut XmlSerializer<'a, W>,
  name: Box<str>,
  namespace: Option<Box<str>>,
}

impl<'a, W> XmlSerializer<'a, W>
where
  W: Write + ?Sized,
{
  pub fn new(writer: &'a mut W) -> Self {
    Self { writer }
  }

  #[inline]
  fn create_element(
    &mut self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&AttributeMap>,
    element_type: ElementType,
  ) -> Result<(), XmlSerializeError> {
    if let Some(namespace) = namespace {
      self
        .writer
        .write_fmt(format_args!("<{}:{}", namespace, name))?;
    } else {
      self.writer.write_fmt(format_args!("<{}", name))?;
    }

    if let Some(attributes) = attributes {
      for attr in attributes {
        self.writer.write_fmt(format_args!(
          " {}=\"{}\"",
          &attr.name,
          escape_xml_attr!(&attr.value)
        ))?;
      }
    }

    match element_type {
      ElementType::Empty => self.writer.write_line("/>")?,
      ElementType::NonEmpty => {
        self.writer.write_line(">")?;
        self.writer.increment_level();
      }
    };

    Ok(())
  }

  #[inline]
  fn close_element(
    &mut self,
    name: &str,
    namespace: Option<&str>,
  ) -> Result<(), XmlSerializeError> {
    self.writer.decrement_level();

    if let Some(namespace) = namespace {
      self
        .writer
        .write_fmt(format_args!("</{}:{}>", namespace, name))?;
    } else {
      self.writer.write_fmt(format_args!("</{}>", name))?;
    }

    self.writer.write_line("")?;

    Ok(())
  }
}

impl<'a, W> Serializer for &'a mut XmlSerializer<'a, W>
where
  W: Write + ?Sized,
{
  type ElementSerializer = XmlElementSerializer<'a, W>;

  fn serialize_str(self, value: &str) -> Result<(), XmlSerializeError> {
    self.writer.write(value)?;

    Ok(())
  }

  fn serialize<V>(self, value: V, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    V: Serialize,
  {
    value.serialize(self, namespace)
  }

  fn serialize_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&AttributeMap>,
  ) -> Result<Self::ElementSerializer, XmlSerializeError> {
    self.create_element(name, namespace, attributes, ElementType::NonEmpty)?;

    Ok(Self::ElementSerializer {
      inner: self,
      name: Box::from(name),
      namespace: namespace.map(Box::from),
    })
  }

  fn serialize_empty_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&AttributeMap>,
  ) -> Result<(), XmlSerializeError> {
    self.create_element(name, namespace, attributes, ElementType::Empty)?;

    Ok(())
  }

  fn serialize_escaped_str(self, value: &str) -> Result<(), XmlSerializeError> {
    escape_writer(value, self.writer, XML_ESCAPE_PATTERNS)?;
    Ok(())
  }
}

impl<'a, W> ElementSerializer for XmlElementSerializer<'a, W>
where
  W: Write + ?Sized,
{
  fn serialize<V>(&mut self, value: V, namespace: Option<&str>) -> Result<(), XmlSerializeError>
  where
    V: Serialize,
  {
    let mut ser = XmlSerializer {
      writer: self.inner.writer,
    };

    value.serialize(&mut ser, namespace)
  }

  fn serialize_str(self, value: &str) -> Result<(), XmlSerializeError> {
    self.inner.writer.write_line(value)?;
    self.end()
  }

  fn end(self) -> Result<(), XmlSerializeError> {
    self
      .inner
      .close_element(&self.name, self.namespace.as_deref())
  }

  fn serialize_escaped_str(self, value: &str) -> Result<(), XmlSerializeError> {
    escape_writer(value, self.inner.writer, XML_ESCAPE_PATTERNS)?;
    self.inner.writer.write_line("")?;
    self.end()
  }
}
