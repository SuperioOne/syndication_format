pub mod formatter;

use core::fmt::Arguments;

use crate::attributes::AttributeMap;
use crate::error::XmlSerializeError;

pub trait Write {
  fn write(&mut self, data: &str) -> Result<()>;
  fn write_line(&mut self, data: &str) -> Result<()>;
  fn write_fmt(&mut self, f: Arguments) -> Result<()>;
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

  /// Non empty elements with children elements or value
  ///
  /// Example:
  /// ```xml
  ///   <test>
  ///     ...
  ///     ...
  ///   </test>
  /// ```
  NoEmpty,
}

pub type Result<T> = core::result::Result<T, XmlSerializeError>;

pub trait Serialize {
  fn serialize<S>(&self, serializer: S) -> Result<()>
  where
    S: Serializer;
}

// Blanket implementation for serializable value type references.
impl<T> Serialize for &'_ T
where
  T: Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<()>
  where
    S: Serializer,
  {
    (*self).serialize(serializer)?;
    Ok(())
  }
}

pub trait Serializer: Sized {
  type ElementSerializer: ElementSerializer;

  /// Serializes `str` reference as element's value.
  fn serialize_str(self, value: &str) -> Result<()>;

  fn serialize<V>(self, value: V) -> Result<()>
  where
    V: Serialize;

  /// Creates an element tag.
  ///
  /// Example:
  /// ```ignore
  /// impl Serialize for Test {
  ///   fn serialize<S>(&self, serializer: S) -> Result<()>
  ///     where
  ///       S: Serializer,
  ///   {
  ///     let mut tag_element = serializer.serialize_element("tag", None, None)?;
  ///     tag_element.serialize(&self.value)?;
  ///
  ///     for nested in self.nested_elements.iter() {
  ///       tag_element.serialize(nested)?;
  ///     }
  ///
  ///     tag_element.end()?;
  ///
  ///     Ok(())
  ///   }
  ///}
  /// ```
  ///
  /// Output:
  /// ```xml
  ///   <tag>
  ///     <nested> .... </nested>
  ///     <nested> .... </nested>
  ///   </tag>
  /// ```
  fn serialize_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&AttributeMap>,
  ) -> Result<Self::ElementSerializer>;

  /// Creates an empty element without any child node and value.
  /// ```xml
  ///   <tag />
  /// ```
  fn serialize_empty_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&AttributeMap>,
  ) -> Result<()>;
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
  ) -> Result<()> {
    if let Some(namespace) = namespace {
      self
        .writer
        .write_fmt(format_args!("<{}:{}", namespace, name))?;
    } else {
      self.writer.write_fmt(format_args!("<{}", name))?;
    }

    if let Some(attributes) = attributes {
      for attr in attributes {
        self
          .writer
          .write_fmt(format_args!(" {}=\"{}\"", &attr.name, &attr.value))?;
      }
    }

    match element_type {
      ElementType::Empty => self.writer.write_line("/>")?,
      ElementType::NoEmpty => {
        self.writer.write_line(">")?;
        self.writer.increment_level();
      }
    };

    Ok(())
  }

  #[inline]
  fn close_element(&mut self, name: &str, namespace: Option<&str>) -> Result<()> {
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

  fn serialize_str(self, value: &str) -> Result<()> {
    self.writer.write(value)?;

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
    attributes: Option<&AttributeMap>,
  ) -> Result<Self::ElementSerializer> {
    self.create_element(name, namespace, attributes.as_deref(), ElementType::NoEmpty)?;

    Ok(Self::ElementSerializer {
      inner: self,
      name: Box::from(name),
      namespace: namespace.map(|f| Box::from(f)),
    })
  }

  fn serialize_empty_element(
    self,
    name: &str,
    namespace: Option<&str>,
    attributes: Option<&AttributeMap>,
  ) -> Result<()> {
    self.create_element(name, namespace, attributes.as_deref(), ElementType::Empty)?;

    Ok(())
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

    value.serialize(&mut temp)
  }

  fn serialize_str(self, value: &str) -> Result<()> {
    self.inner.writer.write_line(value)?;
    self.end()
  }

  fn end(self) -> Result<()> {
    self
      .inner
      .close_element(&self.name, self.namespace.as_deref())
  }
}

#[cfg(test)]
mod test {
  use super::{ElementSerializer, Result, Serialize, Serializer};
  use crate::attributes::{AttributeMap, AttributeName, AttributeValue};
  use crate::serializer::formatter::{DefaultWriter, IndentedWriter, SpaceStyle};
  use crate::serializer::XmlSerializer;
  use std::str::from_utf8_unchecked;

  pub struct Title {
    text: String,
  }

  pub struct Root {
    title: Title,
    list: Vec<ListItem>,
  }

  pub struct ListItem {
    value: usize,
  }

  impl Serialize for ListItem {
    fn serialize<S>(&self, serializer: S) -> Result<()>
    where
      S: Serializer,
    {
      let mut attr = AttributeMap::new();

      attr.set(
        AttributeName::new("value")?,
        AttributeValue::new(&self.value.to_string()),
      );

      serializer.serialize_empty_element("li", Some("xhtml"), Some(&attr))?;

      Ok(())
    }
  }

  impl Serialize for Vec<ListItem> {
    fn serialize<S>(&self, serializer: S) -> Result<()>
    where
      S: Serializer,
    {
      let mut list_element = serializer.serialize_element("ul", Some("xhtml"), None)?;

      for item in self.iter() {
        list_element.serialize(item)?;
      }

      list_element.end()?;

      Ok(())
    }
  }

  impl Serialize for Title {
    fn serialize<S>(&self, serializer: S) -> Result<()>
    where
      S: Serializer,
    {
      let title_element = serializer.serialize_element("h1", Some("xhtml"), None)?;
      title_element.serialize_str(&self.text)?;

      Ok(())
    }
  }

  impl Serialize for Root {
    fn serialize<S>(&self, serializer: S) -> Result<()>
    where
      S: Serializer,
    {
      let mut root_element = serializer.serialize_element("root", None, None).unwrap();

      root_element.serialize(&self.title)?;
      root_element.serialize(&self.list)?;
      root_element.end()?;

      Ok(())
    }
  }

  #[test]
  fn xml_serializer_write() {
    let expected = r#"<root><xhtml:h1>Hello world!</xhtml:h1><xhtml:ul><xhtml:li value="12"/><xhtml:li value="13"/></xhtml:ul></root>"#;
    let obj = Root {
      list: vec![ListItem { value: 12 }, ListItem { value: 13 }],
      title: Title {
        text: String::from("Hello world!"),
      },
    };

    let mut buffer: Vec<u8> = Vec::new();
    let mut default_writer = DefaultWriter::new(&mut buffer);
    let mut xml_writer = XmlSerializer {
      writer: &mut default_writer,
    };

    let _ = xml_writer.serialize(obj);
    let serialized_text = unsafe { from_utf8_unchecked(&buffer) };

    assert_eq!(expected, serialized_text);
  }

  #[test]
  fn xml_serializer_formatted_write() {
    let expected = "<root>
    <xhtml:h1>
        Hello world!
    </xhtml:h1>
    <xhtml:ul>
        <xhtml:li value=\"12\"/>
        <xhtml:li value=\"13\"/>
    </xhtml:ul>
</root>
";

    let obj = Root {
      list: vec![ListItem { value: 12 }, ListItem { value: 13 }],
      title: Title {
        text: String::from("Hello world!"),
      },
    };

    let mut buffer: Vec<u8> = Vec::new();
    let mut default_writer = IndentedWriter::new(&mut buffer, SpaceStyle::WhiteSpace, 4);
    let mut xml_writer = XmlSerializer {
      writer: &mut default_writer,
    };

    let _ = xml_writer.serialize(obj);
    let serialized_text = unsafe { from_utf8_unchecked(&buffer) };

    assert_eq!(expected, serialized_text);
  }

  #[test]
  fn xml_serializer_tab_formatted_write() {
    let expected = "<root>
\t<xhtml:h1>
\t\tHello world!
\t</xhtml:h1>
\t<xhtml:ul>
\t\t<xhtml:li value=\"12\"/>
\t\t<xhtml:li value=\"13\"/>
\t</xhtml:ul>
</root>
";

    let obj = Root {
      list: vec![ListItem { value: 12 }, ListItem { value: 13 }],
      title: Title {
        text: String::from("Hello world!"),
      },
    };

    let mut buffer: Vec<u8> = Vec::new();
    let mut default_writer = IndentedWriter::new(&mut buffer, SpaceStyle::Tabs, 1);
    let mut xml_writer = XmlSerializer {
      writer: &mut default_writer,
    };

    let _ = xml_writer.serialize(obj);
    let serialized_text = unsafe { from_utf8_unchecked(&buffer) };

    assert_eq!(expected, serialized_text);
  }
}
